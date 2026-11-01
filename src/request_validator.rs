//! Checks a request against API v2's request schemas (request_schemas.json, the schemas the API validates requests
//! with) before it's sent, and says what the API would say about it. api_clients/validator_conformance checks that it
//! agrees with the API.

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::sync::{Mutex, OnceLock};

use regex::Regex;
use serde_json::{Map, Number, Value};

use crate::apis::configuration::Configuration;
use crate::models::ErrorDetail;

static REQUEST_SCHEMAS_JSON: &str = include_str!("../request_schemas.json");

/// Returned before a request is sent when the API would reject it with a 422. `details` are the API's error details.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidationError {
    pub details: Vec<ErrorDetail>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shown: Vec<String> = self
            .details
            .iter()
            .take(3)
            .map(|detail| {
                let subject = match (&detail.pointer, &detail.parameter) {
                    (Some(pointer), _) if pointer.is_empty() => "the request body",
                    (Some(pointer), _) => pointer.as_str(),
                    (None, Some(parameter)) => parameter.as_str(),
                    (None, None) => "",
                };
                format!("{subject} {}", detail.message)
            })
            .collect();
        write!(f, "Invalid request: {}", shown.join("; "))?;
        if self.details.len() > 3 {
            write!(f, " (and {} more)", self.details.len() - 3)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}

// What a multipart body's file field holds when the API checks it
pub(crate) const FILE_PLACEHOLDER: &str = "(file)";

fn type_name(name: &str) -> &str {
    match name {
        "string" => "a string",
        "integer" => "an integer",
        "number" => "a number",
        "boolean" => "true or false",
        "object" => "an object",
        "array" => "an array",
        "null" => "null",
        other => other,
    }
}

fn request_schemas() -> &'static Value {
    static SCHEMAS: OnceLock<Value> = OnceLock::new();
    SCHEMAS.get_or_init(|| {
        serde_json::from_str(REQUEST_SCHEMAS_JSON)
            .unwrap_or_else(|error| panic!("docspring: can't read request_schemas.json: {error}"))
    })
}

fn regex(source: &str) -> Regex {
    Regex::new(source).unwrap_or_else(|error| panic!("docspring: invalid regexp {source}: {error}"))
}

// A date and time as the API reads it (Ruby's DateTime.rfc3339, then json_schemer's checks)
fn date_time_regex() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        regex(
            r"\A[ \t\n\x0B\x0C\r]*(-?[0-9]{4})-([0-9]{2})-([0-9]{2})[Tt \t\n\x0B\x0C\r]([0-9]{2}):([0-9]{2}):([0-9]{2})(?:\.[0-9]+)?(?:[Zz]|([+-])([0-9]{2}):([0-9]{2}))\z",
        )
    })
}

// An absolute URI as the API reads it (RFC 3986, as Ruby's URI parser has it), a little looser for IP addresses
fn uri_regex() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        regex(concat!(
            r"\A[A-Za-z][+\-.0-9A-Za-z]*:",
            r"(?://(?:(?:%[0-9A-Fa-f]{2}|[!$&-.0-9:;=A-Z_a-z~])*@)?",
            r"(?:\[(?:[0-9A-Fa-f:.]+|[vV][0-9A-Fa-f]+\.[!$&-.0-9:;=A-Z_a-z~]+)\]|(?:%[0-9A-Fa-f]{2}|[!$&-.0-9;=A-Z_a-z~])*)",
            r"(?::[0-9]*)?(?:/(?:%[0-9A-Fa-f]{2}|[!$&-.0-9:;=@A-Z_a-z~/])*)?",
            r"|/(?:(?:%[0-9A-Fa-f]{2}|[!$&-.0-9:;=@A-Z_a-z~])(?:%[0-9A-Fa-f]{2}|[!$&-.0-9:;=@A-Z_a-z~/])*)?",
            r"|(?:%[0-9A-Fa-f]{2}|[!$&-.0-9:;=@A-Z_a-z~])(?:%[0-9A-Fa-f]{2}|[!$&-.0-9:;=@A-Z_a-z~/])*",
            r"|)",
            r"(?:\?[^#]*)?",
            r"(?:#(?:%[0-9A-Fa-f]{2}|[!$&-.0-9:;=@A-Z_a-z~/?])*)?\z",
        ))
    })
}

// Checks a request an operation (createSubmission) built, unless the configuration's validate_requests is false. A
// multipart request's fields are passed in, with each file as "(file)": its body can't be read back.
pub(crate) fn check_request(
    configuration: &Configuration,
    operation_id: &str,
    request: &reqwest::Request,
    form: Option<Map<String, Value>>,
) -> Result<(), ValidationError> {
    if !configuration.validate_requests {
        return Ok(());
    }
    let mut query = Map::new();
    for (name, value) in request.url().query_pairs() {
        query
            .entry(name.into_owned())
            .or_insert(Value::String(value.into_owned()));
    }
    let body = match form {
        Some(fields) => Some(Value::Object(fields)),
        None => request
            .body()
            .and_then(|body| body.as_bytes())
            .and_then(|bytes| serde_json::from_slice(bytes).ok()),
    };
    let details = request_errors(operation_id, &query, body.as_ref());
    if details.is_empty() {
        Ok(())
    } else {
        Err(ValidationError { details })
    }
}

/// The details for a request to an operation (`createSubmission`), with its query parameters as strings
pub fn request_errors(
    operation_id: &str,
    query: &Map<String, Value>,
    body: Option<&Value>,
) -> Vec<ErrorDetail> {
    let entry = match request_schemas().pointer(&format!("/operations/{operation_id}")) {
        Some(Value::Object(entry)) => entry,
        _ => return Vec::new(),
    };
    let mut details = Vec::new();
    if let (Some(Value::Object(params)), false) = (entry.get("query"), query.is_empty()) {
        let mut properties = Map::new();
        let mut required = Vec::new();
        for (name, param) in params {
            properties.insert(
                name.clone(),
                param.get("schema").cloned().unwrap_or(Value::Null),
            );
            if param.get("required") == Some(&Value::Bool(true)) {
                required.push(Value::String(name.clone()));
            }
        }
        let schema = serde_json::json!({
            "type": "object",
            "properties": properties,
            "required": required,
            "additionalProperties": false,
        });
        details.extend(validate_params(&schema, query, "query"));
    }
    if let (Some(Value::Object(body_entry)), Some(body)) = (entry.get("body"), body) {
        let schema = body_entry.get("schema").unwrap_or(&Value::Null);
        if body_entry.get("mediaType").and_then(Value::as_str) == Some("multipart/form-data") {
            if let Value::Object(values) = body {
                details.extend(validate_params(schema, values, "body"));
            }
        } else {
            details.extend(validate_value(schema, body, "body"));
        }
    }
    details
}

/// The details for data checked against a schema at a location (`body` or `query`)
pub fn validate_value(schema: &Value, data: &Value, location: &str) -> Vec<ErrorDetail> {
    let mut details = Vec::new();
    check(schema, data, "", location, &mut details);
    details
}

// Query and multipart params arrive as strings, which the API converts where the schema types them
fn validate_params(schema: &Value, data: &Map<String, Value>, location: &str) -> Vec<ErrorDetail> {
    let properties = schema.get("properties");
    let converted: Map<String, Value> = data
        .iter()
        .map(|(name, value)| {
            (
                name.clone(),
                convert_param(
                    value,
                    properties.and_then(|properties| properties.get(name)),
                ),
            )
        })
        .collect();
    validate_value(schema, &Value::Object(converted), location)
}

fn convert_param(value: &Value, schema: Option<&Value>) -> Value {
    let schema = match schema {
        Some(schema @ Value::Object(_)) => schema,
        _ => return value.clone(),
    };
    let types = schema_types(schema);
    if let (Value::Array(items), true) = (value, types.contains(&"array")) {
        return Value::Array(
            items
                .iter()
                .map(|item| convert_param(item, schema.get("items")))
                .collect(),
        );
    }
    let text = match value {
        Value::String(text) => text,
        _ => return value.clone(),
    };
    if types.contains(&"integer") && is_integer_text(text) {
        if let Ok(number) = text.parse::<Number>() {
            return Value::Number(number);
        }
    }
    if types.contains(&"number") && is_number_text(text) {
        if let Ok(number) = text.parse::<Number>() {
            return Value::Number(number);
        }
    }
    if types.contains(&"boolean") && (text == "true" || text == "false") {
        return Value::Bool(text == "true");
    }
    value.clone()
}

fn is_integer_text(text: &str) -> bool {
    let digits = text.strip_prefix('-').unwrap_or(text);
    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_number_text(text: &str) -> bool {
    match text.split_once('.') {
        Some((whole, fraction)) => {
            is_integer_text(whole)
                && !fraction.is_empty()
                && fraction.bytes().all(|byte| byte.is_ascii_digit())
        }
        None => is_integer_text(text),
    }
}

fn schema_types(schema: &Value) -> Vec<&str> {
    match schema.get("type") {
        Some(Value::String(name)) => vec![name.as_str()],
        Some(Value::Array(names)) => names.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    }
}

fn resolve_schema(mut schema: &Value) -> &Value {
    while let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        let name = reference.trim_start_matches("#/components/schemas/");
        schema = match request_schemas()
            .get("components")
            .and_then(|components| components.get("schemas"))
        {
            Some(schemas) => schemas.get(name).unwrap_or(&Value::Null),
            None => &Value::Null,
        };
    }
    schema
}

fn check(
    raw_schema: &Value,
    data: &Value,
    pointer: &str,
    location: &str,
    details: &mut Vec<ErrorDetail>,
) {
    let schema = resolve_schema(raw_schema);
    if !schema.is_object() {
        return;
    }
    if let Some(discriminator @ Value::Object(_)) = schema.get("discriminator") {
        check_union(discriminator, data, pointer, location, details);
        return;
    }

    if schema.get("type").is_some() {
        let types = schema_types(schema);
        if !types.iter().any(|name| is_json_type(data, name)) {
            let message = if pointer.is_empty() && location == "body" {
                "must be a JSON object".to_owned()
            } else {
                type_message(&types)
            };
            details.push(detail(location, pointer, "type", message));
        }
    }
    if let Some(Value::Array(values)) = schema.get("enum") {
        if !values.iter().any(|value| json_equal(value, data)) {
            let shown: Vec<String> = values.iter().map(to_json).collect();
            details.push(detail(
                location,
                pointer,
                "enum",
                format!("must be one of: {}", shown.join(", ")),
            ));
        }
    }
    if let Some(value) = schema.get("const") {
        if !json_equal(value, data) {
            details.push(detail(
                location,
                pointer,
                "const",
                format!("must be {}", to_json(value)),
            ));
        }
    }

    match data {
        Value::String(value) => check_string(schema, value, pointer, location, details),
        Value::Number(value) => {
            if let Some(number) = value.as_f64() {
                check_number(schema, number, pointer, location, details);
            }
        }
        Value::Array(value) => check_array(schema, value, pointer, location, details),
        Value::Object(value) => check_object(schema, value, pointer, location, details),
        _ => {}
    }
}

// A union (oneOf with a discriminator): the discriminator property's value picks the variant, and only that one is
// checked, as the API does. The mapping is [value, $ref] pairs, in the order the API lists them.
fn check_union(
    discriminator: &Value,
    data: &Value,
    pointer: &str,
    location: &str,
    details: &mut Vec<ErrorDetail>,
) {
    let object = match data {
        Value::Object(object) => object,
        _ => {
            details.push(detail(
                location,
                pointer,
                "type",
                format!("must be {}", type_name("object")),
            ));
            return;
        }
    };

    let name = discriminator
        .get("propertyName")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let child = format!("{pointer}/{}", escape_pointer(name));
    let value = match object.get(name) {
        Some(value) => value,
        None => {
            details.push(detail(
                location,
                &child,
                "required",
                "is required".to_owned(),
            ));
            return;
        }
    };

    let mut shown = Vec::new();
    if let Some(Value::Array(mapping)) = discriminator.get("mapping") {
        for pair in mapping {
            let pair = match pair {
                Value::Array(pair) if pair.len() == 2 => pair,
                _ => continue,
            };
            if value.is_string() && &pair[0] == value {
                check(
                    &serde_json::json!({ "$ref": pair[1] }),
                    data,
                    pointer,
                    location,
                    details,
                );
                return;
            }
            shown.push(to_json(&pair[0]));
        }
    }
    details.push(detail(
        location,
        &child,
        "enum",
        format!("must be one of: {}", shown.join(", ")),
    ));
}

fn check_string(
    schema: &Value,
    data: &str,
    pointer: &str,
    location: &str,
    details: &mut Vec<ErrorDetail>,
) {
    let length = data.chars().count() as f64;
    if let Some(limit) = schema_number(schema, "minLength") {
        if length < limit {
            let message = at_least(
                limit,
                format!(
                    "must be at least {}",
                    count_noun(schema, "minLength", limit, "character")
                ),
            );
            details.push(detail(location, pointer, "minLength", message));
        }
    }
    if let Some(limit) = schema_number(schema, "maxLength") {
        if length > limit {
            let message = format!(
                "must be at most {}",
                count_noun(schema, "maxLength", limit, "character")
            );
            details.push(detail(location, pointer, "maxLength", message));
        }
    }
    if let Some(source) = schema.get("pattern").and_then(Value::as_str) {
        if let Some(compiled) = pattern(source) {
            if !compiled.is_match(data) {
                let message = match schema.get("x-pattern-message").and_then(Value::as_str) {
                    Some(message) => message.to_owned(),
                    None => format!("must match the pattern {source}"),
                };
                details.push(detail(location, pointer, "pattern", message));
            }
        }
    }
    if let Some(name) = schema.get("format").and_then(Value::as_str) {
        if !valid_format(name, data) {
            let message = match request_schemas()
                .pointer(&format!("/formatMessages/{name}"))
                .and_then(Value::as_str)
            {
                Some(message) => message.to_owned(),
                None => format!("must be a valid {name}"),
            };
            details.push(detail(location, pointer, "format", message));
        }
    }
}

fn check_number(
    schema: &Value,
    data: f64,
    pointer: &str,
    location: &str,
    details: &mut Vec<ErrorDetail>,
) {
    let mut limit = |keyword: &str, fails: bool, text: &str| {
        if fails {
            details.push(detail(
                location,
                pointer,
                keyword,
                format!("{text} {}", number_text(&schema[keyword])),
            ));
        }
    };
    if let Some(minimum) = schema_number(schema, "minimum") {
        limit("minimum", data < minimum, "must be at least");
    }
    if let Some(maximum) = schema_number(schema, "maximum") {
        limit("maximum", data > maximum, "must be at most");
    }
    if let Some(minimum) = schema_number(schema, "exclusiveMinimum") {
        limit("exclusiveMinimum", data <= minimum, "must be greater than");
    }
    if let Some(maximum) = schema_number(schema, "exclusiveMaximum") {
        limit("exclusiveMaximum", data >= maximum, "must be less than");
    }
}

fn check_array(
    schema: &Value,
    data: &[Value],
    pointer: &str,
    location: &str,
    details: &mut Vec<ErrorDetail>,
) {
    let size = data.len() as f64;
    if let Some(limit) = schema_number(schema, "minItems") {
        if size < limit {
            let message = at_least(
                limit,
                format!(
                    "must have at least {}",
                    count_noun(schema, "minItems", limit, "item")
                ),
            );
            details.push(detail(location, pointer, "minItems", message));
        }
    }
    if let Some(limit) = schema_number(schema, "maxItems") {
        if size > limit {
            let message = format!(
                "must have at most {}",
                count_noun(schema, "maxItems", limit, "item")
            );
            details.push(detail(location, pointer, "maxItems", message));
        }
    }
    if schema.get("uniqueItems") == Some(&Value::Bool(true)) && !unique_items(data) {
        let message = match first_duplicate(data) {
            Some(duplicate) => format!("has {} more than once", to_json(duplicate)),
            None => "can't contain the same item twice".to_owned(),
        };
        details.push(detail(location, pointer, "uniqueItems", message));
    }
    if let Some(items) = schema.get("items") {
        for (index, item) in data.iter().enumerate() {
            check(
                items,
                item,
                &format!("{pointer}/{index}"),
                location,
                details,
            );
        }
    }
}

fn check_object(
    schema: &Value,
    data: &Map<String, Value>,
    pointer: &str,
    location: &str,
    details: &mut Vec<ErrorDetail>,
) {
    if let Some(Value::Array(required)) = schema.get("required") {
        for name in required.iter().filter_map(Value::as_str) {
            if !data.contains_key(name) {
                details.push(detail(
                    location,
                    &format!("{pointer}/{}", escape_pointer(name)),
                    "required",
                    "is required".to_owned(),
                ));
            }
        }
    }
    // Only an update's body has minProperties, and it's always 1 (api_clients/scripts/extract_request_schemas)
    if let Some(limit) = schema_number(schema, "minProperties") {
        if (data.len() as f64) < limit {
            details.push(detail(
                location,
                pointer,
                "minProperties",
                "must have at least one property to change".to_owned(),
            ));
        }
    }

    let properties = schema.get("properties").and_then(Value::as_object);
    let mut names: Vec<&String> = data.keys().collect();
    names.sort();
    for name in names {
        let child = format!("{pointer}/{}", escape_pointer(name));
        if let Some(property) = properties.and_then(|properties| properties.get(name)) {
            check(property, &data[name], &child, location, details);
        } else if schema.get("additionalProperties") == Some(&Value::Bool(false)) {
            details.push(unknown_property(schema, name, &child, location));
        } else if let Some(additional @ Value::Object(_)) = schema.get("additionalProperties") {
            check(additional, &data[name], &child, location, details);
        }
    }
}

fn unknown_property(schema: &Value, name: &str, pointer: &str, location: &str) -> ErrorDetail {
    if let Some(message) = schema
        .get("x-not-allowed-properties")
        .and_then(|not_allowed| not_allowed.get(name))
        .and_then(Value::as_str)
    {
        return detail(location, pointer, "not_allowed", message.to_owned());
    }

    let noun = if location == "query" {
        "query parameter"
    } else {
        "property"
    };
    let custom = schema
        .get("x-unknown-property-message")
        .and_then(Value::as_str);
    let mut message = match custom {
        Some(custom) => custom.to_owned(),
        None => format!("isn't a documented {noun}"),
    };
    let snake = underscore(name);
    let documented = schema
        .get("properties")
        .and_then(|properties| properties.get(&snake))
        .is_some();
    if documented && snake != name {
        let plural = if custom.is_some() {
            "settings"
        } else if noun == "query parameter" {
            "query parameters"
        } else {
            "properties"
        };
        message.push_str(&format!(". API v2 names {plural} in snake_case: '{snake}'"));
    }
    detail(location, pointer, "unknown_property", message)
}

fn detail(location: &str, pointer: &str, code: &str, message: String) -> ErrorDetail {
    let mut detail = ErrorDetail::new(code.to_owned(), message);
    if location == "query" {
        let parameter = pointer
            .strip_prefix('/')
            .unwrap_or(pointer)
            .split('/')
            .next()
            .unwrap_or_default();
        detail.parameter = Some(parameter.replace("~1", "/").replace("~0", "~"));
    } else {
        detail.pointer = Some(pointer.to_owned());
    }
    detail
}

fn schema_number(schema: &Value, keyword: &str) -> Option<f64> {
    schema.get(keyword).and_then(Value::as_f64)
}

fn is_json_type(data: &Value, name: &str) -> bool {
    match name {
        "string" => data.is_string(),
        "integer" => data
            .as_f64()
            .is_some_and(|number| number.is_finite() && number == number.floor()),
        "number" => data.is_number(),
        "boolean" => data.is_boolean(),
        "object" => data.is_object(),
        "array" => data.is_array(),
        "null" => data.is_null(),
        _ => false,
    }
}

// Patterns are ECMA-262 regexps, as OpenAPI says. Without the multi-line flag, ^ and $ are already the start and end
// of the value. ECMA-262's \d is ASCII, and the only class escape the API's patterns use
// (api_clients/scripts/extract_request_schemas), so it becomes [0-9]: Rust's \d is any Unicode digit. One Rust can't
// compile isn't checked.
fn pattern(source: &str) -> Option<Regex> {
    static PATTERNS: OnceLock<Mutex<HashMap<String, Option<Regex>>>> = OnceLock::new();
    let patterns = PATTERNS.get_or_init(Default::default);
    let mut patterns = patterns
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    patterns
        .entry(source.to_owned())
        .or_insert_with(|| {
            let mut translated = String::with_capacity(source.len());
            let mut chars = source.chars();
            while let Some(char) = chars.next() {
                match (char, char == '\\') {
                    (_, true) => match chars.next() {
                        Some('d') => translated.push_str("[0-9]"),
                        Some(escaped) => {
                            translated.push('\\');
                            translated.push(escaped);
                        }
                        None => translated.push('\\'),
                    },
                    (char, false) => translated.push(char),
                }
            }
            Regex::new(&translated).ok()
        })
        .clone()
}

fn valid_format(name: &str, data: &str) -> bool {
    match name {
        "uri" => {
            data.is_ascii()
                && !data.bytes().any(|byte| b" \t\n\x0B\x0C\r".contains(&byte))
                && uri_regex().is_match(data)
        }
        "date-time" => valid_date_time(data),
        _ => true,
    }
}

fn valid_date_time(data: &str) -> bool {
    let captures = match date_time_regex().captures(data) {
        Some(captures) => captures,
        None => return false,
    };
    let number = |index: usize| -> i64 {
        captures
            .get(index)
            .and_then(|found| found.as_str().parse().ok())
            .unwrap_or(0)
    };
    let (year, month, day, hour, minute, second) = (
        number(1),
        number(2),
        number(3),
        number(4),
        number(5),
        number(6),
    );
    if !(1..=12).contains(&month) || day < 1 || day > days_in_month(year, month) {
        return false;
    }
    if hour > 23 || minute > 59 || second > 60 {
        return false;
    }
    let mut offset = 0;
    if let Some(sign) = captures.get(7) {
        let (offset_hours, offset_minutes) = (number(8), number(9));
        if offset_hours > 23 || offset_minutes > 59 {
            return false;
        }
        offset = offset_hours * 60 + offset_minutes;
        if sign.as_str() == "-" {
            offset = -offset;
        }
    }
    // A leap second is only at the end of a UTC day
    second < 60 || (hour * 60 + minute - offset).rem_euclid(1440) == 1439
}

// Before the Gregorian calendar (1582), the API's dates are Julian: every fourth year is a leap year
fn days_in_month(year: i64, month: i64) -> i64 {
    let leap = year % 4 == 0 && (year < 1583 || year % 100 != 0 || year % 400 == 0);
    if month == 2 && leap {
        return 29;
    }
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][(month - 1) as usize]
}

fn unique_items(data: &[Value]) -> bool {
    data.iter().enumerate().all(|(index, item)| {
        data[index + 1..]
            .iter()
            .all(|other| !json_equal(item, other))
    })
}

// The first value that's in the list twice, as the API names it
fn first_duplicate(data: &[Value]) -> Option<&Value> {
    let mut keys: Vec<String> = Vec::new();
    let mut counts: HashMap<String, (usize, &Value)> = HashMap::new();
    for item in data {
        let key = tally_key(item);
        let entry = counts.entry(key.clone()).or_insert_with(|| {
            keys.push(key);
            (0, item)
        });
        entry.0 += 1;
    }
    keys.iter()
        .map(|key| counts[key])
        .find(|(count, _)| *count > 1)
        .map(|(_, item)| item)
}

fn tally_key(value: &Value) -> String {
    match value {
        Value::Array(items) => format!(
            "[{}]",
            items.iter().map(tally_key).collect::<Vec<_>>().join(",")
        ),
        Value::Object(object) => {
            let sorted: BTreeMap<&String, &Value> = object.iter().collect();
            let parts: Vec<String> = sorted
                .iter()
                .map(|(name, item)| {
                    format!(
                        "{}:{}",
                        to_json(&Value::String((*name).clone())),
                        tally_key(item)
                    )
                })
                .collect();
            format!("{{{}}}", parts.join(","))
        }
        _ => to_json(value),
    }
}

// JSON equality: numbers by value, objects regardless of key order
fn json_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.as_f64() == right.as_f64(),
        (Value::Array(left), Value::Array(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(left, right)| json_equal(left, right))
        }
        (Value::Object(left), Value::Object(right)) => {
            left.len() == right.len()
                && left.iter().all(|(name, value)| {
                    right
                        .get(name)
                        .is_some_and(|other| json_equal(value, other))
                })
        }
        _ => left == right,
    }
}

fn type_message(types: &[&str]) -> String {
    let names: Vec<&str> = types.iter().map(|name| type_name(name)).collect();
    let connector = if names.len() > 1 && names.iter().any(|name| name.contains(" or ")) {
        ", or "
    } else {
        " or "
    };
    match names.len() {
        0 => "must be valid".to_owned(),
        1 => format!("must be {}", names[0]),
        2 => format!("must be {}", names.join(connector)),
        count => format!(
            "must be {}{connector}{}",
            names[..count - 1].join(", "),
            names[count - 1]
        ),
    }
}

fn at_least(limit: f64, message: String) -> String {
    if limit == 1.0 {
        "can't be empty".to_owned()
    } else {
        message
    }
}

fn count_noun(schema: &Value, keyword: &str, number: f64, noun: &str) -> String {
    let text = number_text(&schema[keyword]);
    if number == 1.0 {
        format!("{text} {noun}")
    } else {
        format!("{text} {noun}s")
    }
}

// A number as the schema writes it
fn number_text(value: &Value) -> String {
    match value {
        Value::Number(number) => number.to_string(),
        other => to_json(other),
    }
}

fn to_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| value.to_string())
}

fn escape_pointer(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

// The snake_case name the API would have for a property (templateId -> template_id, PDFFile -> pdf_file)
fn underscore(name: &str) -> String {
    static ACRONYM_BOUNDARY: OnceLock<Regex> = OnceLock::new();
    static WORD_BOUNDARY: OnceLock<Regex> = OnceLock::new();
    let acronym_boundary = ACRONYM_BOUNDARY.get_or_init(|| regex(r"([A-Z0-9]+)([A-Z][a-z])"));
    let word_boundary = WORD_BOUNDARY.get_or_init(|| regex(r"([a-z0-9])([A-Z])"));
    let name = acronym_boundary.replace_all(name, "${1}_${2}");
    let name = word_boundary.replace_all(&name, "${1}_${2}");
    name.replace('-', "_").to_lowercase()
}
