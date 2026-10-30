//! Builds a [`Client`] from an API token and a host, or from environment variables.

use std::fmt;
use std::sync::Arc;

use crate::apis::client::Client;
use crate::apis::configuration::Configuration;

/// A client setting that can't work, such as a `DOCSPRING_REGION` that isn't `US` or `EU`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigurationError {
    message: String,
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ConfigurationError {}

/// Builds a [`Client`]. A setting you leave out (or set to an empty string) comes from an environment variable:
/// `DOCSPRING_TOKEN_ID` and `DOCSPRING_TOKEN_SECRET` for the API token, and `DOCSPRING_HOST` or `DOCSPRING_REGION`
/// (`US` or `EU`) for the host. A host wins over a region, however each is set: the host passed in, then
/// `DOCSPRING_HOST`, then the region passed in, then `DOCSPRING_REGION`. Without any of them, the client uses the US
/// API.
///
/// ```no_run
/// let client = docspring::Client::builder()
///     .token_id("API_TOKEN_ID")
///     .token_secret("API_TOKEN_SECRET")
///     .region("EU")
///     .build()?;
/// # Ok::<(), docspring::ConfigurationError>(())
/// ```
#[derive(Clone, Default)]
pub struct ClientBuilder {
    token_id: Option<String>,
    token_secret: Option<String>,
    host: Option<String>,
    region: Option<String>,
    http_client: Option<reqwest::Client>,
    validate_requests: Option<bool>,
}

// Like Configuration's, without the token secret
impl fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientBuilder")
            .field("token_id", &self.token_id)
            .field(
                "token_secret",
                &self.token_secret.as_ref().map(|_| "[REDACTED]"),
            )
            .field("host", &self.host)
            .field("region", &self.region)
            .field("http_client", &self.http_client)
            .field("validate_requests", &self.validate_requests)
            .finish()
    }
}

impl ClientBuilder {
    /// The API token's ID
    pub fn token_id(mut self, token_id: impl Into<String>) -> Self {
        self.token_id = Some(token_id.into());
        self
    }

    /// The API token's secret
    pub fn token_secret(mut self, token_secret: impl Into<String>) -> Self {
        self.token_secret = Some(token_secret.into());
        self
    }

    /// The API's host, such as `sync.api-eu.docspring.com` or `https://sync.api-eu.docspring.com`. It wins over a
    /// region.
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    /// The region your DocSpring account is in: `US` or `EU`. A host wins over it, even one from `DOCSPRING_HOST`.
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// The reqwest client that sends the requests, such as one with a timeout or a proxy
    pub fn http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    /// Whether to check each request against the API's request schemas before sending it (true by default). A request
    /// the API would reject then fails with [`crate::Error::RequestValidation`] without being sent.
    pub fn validate_requests(mut self, validate_requests: bool) -> Self {
        self.validate_requests = Some(validate_requests);
        self
    }

    /// The client, or a [`ConfigurationError`] for a region that isn't `US` or `EU`, or when there's no HTTP client
    /// passed in and reqwest can't create one (no TLS backend)
    pub fn build(self) -> Result<Client, ConfigurationError> {
        let configuration = self.configuration(|variable| std::env::var(variable).ok())?;
        Ok(Client::new(Arc::new(configuration)))
    }

    // The client's configuration, with `env` reading environment variables. It doesn't start from
    // Configuration::default(), which creates a reqwest client and panics when it can't.
    fn configuration(
        self,
        env: impl Fn(&str) -> Option<String>,
    ) -> Result<Configuration, ConfigurationError> {
        let setting = |value: Option<String>, variable: &str| {
            present(value).or_else(|| present(env(variable)))
        };
        let token_id = setting(self.token_id, "DOCSPRING_TOKEN_ID");
        let token_secret = setting(self.token_secret, "DOCSPRING_TOKEN_SECRET");
        let host = match setting(self.host, "DOCSPRING_HOST") {
            Some(host) => host,
            None => match setting(self.region, "DOCSPRING_REGION") {
                Some(region) => region_host(&region)?.to_owned(),
                None => US_HOST.to_owned(),
            },
        };
        let client = match self.http_client {
            Some(client) => client,
            None => reqwest::Client::builder()
                .build()
                .map_err(|error| ConfigurationError {
                    message: format!("reqwest couldn't create an HTTP client: {error}"),
                })?,
        };
        let basic_auth = (token_id.is_some() || token_secret.is_some())
            .then(|| (token_id.unwrap_or_default(), token_secret));
        Ok(Configuration {
            base_path: base_path(&host),
            user_agent: Some(USER_AGENT.to_owned()),
            client,
            basic_auth,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
            validate_requests: self.validate_requests.unwrap_or(true),
        })
    }
}

impl Client {
    /// A [`ClientBuilder`], to set the API token and the host
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// A client with the API token in `DOCSPRING_TOKEN_ID` and `DOCSPRING_TOKEN_SECRET`, and the host from
    /// `DOCSPRING_HOST` or `DOCSPRING_REGION`
    pub fn from_env() -> Result<Client, ConfigurationError> {
        ClientBuilder::default().build()
    }
}

const US_HOST: &str = "sync.api.docspring.com";
const EU_HOST: &str = "sync.api-eu.docspring.com";
// The generated Configuration::default()'s
const USER_AGENT: &str = concat!("docspring-rust-", env!("CARGO_PKG_VERSION"));

// The value, unless it's empty or blank
fn present(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let value = value.trim();
        (!value.is_empty()).then(|| value.to_owned())
    })
}

fn region_host(region: &str) -> Result<&'static str, ConfigurationError> {
    match region.trim().to_uppercase().as_str() {
        "US" => Ok(US_HOST),
        "EU" => Ok(EU_HOST),
        _ => Err(ConfigurationError {
            message: format!("{region} is not a valid region. Valid regions: US, EU"),
        }),
    }
}

// The API's base path on a host: https:// when the host has no scheme, and any API version's path (such as /api/v1
// from a 3.x setup) becomes /api/v2
fn base_path(host: &str) -> String {
    let lower = host.to_ascii_lowercase();
    let mut url = if lower.starts_with("http://") || lower.starts_with("https://") {
        host.to_owned()
    } else {
        format!("https://{host}")
    };
    while url.ends_with('/') {
        url.pop();
    }
    if let Some(index) = url.to_ascii_lowercase().rfind("/api/v") {
        let version = &url[index + "/api/v".len()..];
        if !version.is_empty() && version.bytes().all(|byte| byte.is_ascii_digit()) {
            url.truncate(index);
        }
    }
    format!("{url}/api/v2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_path_adds_the_scheme_and_api_v2() {
        assert_eq!(
            base_path("sync.api-eu.docspring.com"),
            "https://sync.api-eu.docspring.com/api/v2"
        );
        assert_eq!(
            base_path("http://localhost:3000/"),
            "http://localhost:3000/api/v2"
        );
        assert_eq!(
            base_path("https://sync.api.docspring.com/api/v1"),
            "https://sync.api.docspring.com/api/v2"
        );
        assert_eq!(
            base_path("HTTPS://example.com/API/V2/"),
            "HTTPS://example.com/api/v2"
        );
    }

    #[test]
    fn region_host_rejects_other_regions() {
        assert_eq!(region_host(" eu ").unwrap(), "sync.api-eu.docspring.com");
        assert_eq!(
            region_host("AU").unwrap_err().to_string(),
            "AU is not a valid region. Valid regions: US, EU"
        );
    }

    // The base path a builder picks with these environment variables
    fn base_path_with(builder: ClientBuilder, variables: &[(&str, &str)]) -> String {
        builder
            .configuration(|variable| {
                variables
                    .iter()
                    .find(|(name, _)| *name == variable)
                    .map(|(_, value)| value.to_string())
            })
            .unwrap()
            .base_path
    }

    #[test]
    fn a_host_wins_over_a_region_however_each_is_set() {
        let us = "https://sync.api.docspring.com/api/v2";
        let eu = "https://sync.api-eu.docspring.com/api/v2";
        let local = "http://localhost:3000/api/v2";
        assert_eq!(base_path_with(Client::builder(), &[]), us);
        assert_eq!(base_path_with(Client::builder().region("eu"), &[]), eu);
        assert_eq!(
            base_path_with(
                Client::builder().host("http://localhost:3000"),
                &[("DOCSPRING_HOST", "example.com")]
            ),
            local
        );
        assert_eq!(
            base_path_with(
                Client::builder().region("EU"),
                &[("DOCSPRING_HOST", "http://localhost:3000")]
            ),
            local
        );
        assert_eq!(
            base_path_with(
                Client::builder().region("EU"),
                &[("DOCSPRING_REGION", "US")]
            ),
            eu
        );
        assert_eq!(
            base_path_with(
                Client::builder().host(" ").region(""),
                &[("DOCSPRING_REGION", "eu")]
            ),
            eu
        );
        assert_eq!(
            Client::builder()
                .configuration(|variable| (variable == "DOCSPRING_REGION").then(|| "AU".to_owned()))
                .unwrap_err()
                .to_string(),
            "AU is not a valid region. Valid regions: US, EU"
        );
    }

    #[test]
    fn the_token_comes_from_the_builder_or_the_environment() {
        let configuration = Client::builder()
            .token_id("id")
            .configuration(|variable| {
                (variable == "DOCSPRING_TOKEN_SECRET").then(|| "secret".to_owned())
            })
            .unwrap();
        assert_eq!(
            configuration.basic_auth,
            Some(("id".to_owned(), Some("secret".to_owned())))
        );
        assert_eq!(
            Client::builder()
                .configuration(|_| None)
                .unwrap()
                .basic_auth,
            None
        );
    }

    #[test]
    fn debug_hides_the_token_secret() {
        let builder = Client::builder()
            .token_id("api_token_id")
            .token_secret("api_token_secret");
        let debug = format!("{builder:?}");
        assert!(debug.contains("api_token_id"), "{debug}");
        assert!(!debug.contains("api_token_secret"), "{debug}");
    }

    #[test]
    fn it_sends_the_generated_user_agent() {
        assert_eq!(
            Client::builder()
                .configuration(|_| None)
                .unwrap()
                .user_agent,
            Configuration::default().user_agent
        );
    }
}
