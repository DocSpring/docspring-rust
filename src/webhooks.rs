//! Checks webhook requests' `X-DocSpring-Signature` header
//! (<https://docspring.com/docs/webhooks/verifying-signatures/>).

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use sha2::Sha256;

/// How old (or early) a webhook request's timestamp can be ([`verify_webhook_signature`])
pub const DEFAULT_WEBHOOK_TOLERANCE: Duration = Duration::from_secs(300);

/// Changes how [`verify_webhook_signature_with_options`] checks the timestamp
#[derive(Clone, Copy, Debug, Default)]
pub struct WebhookSignatureOptions {
    /// How old (or early) the timestamp can be. `None` means [`DEFAULT_WEBHOOK_TOLERANCE`].
    pub tolerance: Option<Duration>,
    /// The current time, for tests. `None` means now.
    pub now: Option<SystemTime>,
}

/// Checks a webhook request's `X-DocSpring-Signature` header. It returns true when one of its `v1` signatures is the
/// HMAC-SHA256 of `{t}.{payload}` with the webhook's secret, and its timestamp `t` is at most 5 minutes from now. It
/// returns false for anything else, including a missing or malformed header.
///
/// `payload` is the request body exactly as it was received, not parsed JSON.
pub fn verify_webhook_signature(payload: &[u8], signature_header: &str, secret: &str) -> bool {
    verify_webhook_signature_with_options(
        payload,
        signature_header,
        secret,
        WebhookSignatureOptions::default(),
    )
}

/// [`verify_webhook_signature`] with another tolerance or current time
pub fn verify_webhook_signature_with_options(
    payload: &[u8],
    signature_header: &str,
    secret: &str,
    options: WebhookSignatureOptions,
) -> bool {
    if secret.is_empty() {
        return false;
    }

    let mut timestamp: Option<&str> = None;
    let mut signatures: Vec<&str> = Vec::new();
    for part in signature_header.split(',') {
        let part = part.trim();
        let (key, value) = part.split_once('=').unwrap_or((part, ""));
        if key == "t" && timestamp.is_none() {
            timestamp = Some(value);
        } else if key == "v1" && !value.is_empty() {
            signatures.push(value);
        }
    }
    let timestamp = match timestamp {
        Some(timestamp)
            if !timestamp.is_empty() && timestamp.bytes().all(|byte| byte.is_ascii_digit()) =>
        {
            timestamp
        }
        _ => return false,
    };
    if signatures.is_empty() {
        return false;
    }
    let seconds: i64 = match timestamp.parse() {
        Ok(seconds) => seconds,
        Err(_) => return false,
    };

    let tolerance = options.tolerance.unwrap_or(DEFAULT_WEBHOOK_TOLERANCE);
    let now = options.now.unwrap_or_else(SystemTime::now);
    let now = match now.duration_since(UNIX_EPOCH) {
        Ok(since_epoch) => since_epoch.as_secs() as i64,
        Err(before_epoch) => -(before_epoch.duration().as_secs() as i64),
    };
    if now.abs_diff(seconds) > tolerance.as_secs() {
        return false;
    }

    let mut mac = match Hmac::<Sha256>::new_from_slice(secret.as_bytes()) {
        Ok(mac) => mac,
        Err(_) => return false,
    };
    mac.update(timestamp.as_bytes());
    mac.update(b".");
    mac.update(payload);
    let expected: String = mac
        .finalize()
        .into_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let mut valid = false;
    for signature in signatures {
        // Constant time, so a timing attack can't find a valid signature byte by byte
        valid |= constant_time_eq(signature.as_bytes(), expected.as_bytes());
    }
    valid
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0u8, |difference, (a, b)| difference | (a ^ b))
        == 0
}
