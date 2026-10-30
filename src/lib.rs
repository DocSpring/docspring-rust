//! DocSpring's API client: fill in PDF templates, generate PDFs from HTML, merge PDFs and request e-signatures.
//!
//! ```no_run
//! use docspring::Api;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Reads DOCSPRING_TOKEN_ID, DOCSPRING_TOKEN_SECRET, and DOCSPRING_HOST or DOCSPRING_REGION
//! let client = docspring::Client::from_env()?;
//! let template = client
//!     .get_template(docspring::GetTemplateParams::builder().template_id("tpl_1234567890abcdef01").build())
//!     .await?;
//! println!("{}", template.name.unwrap_or_default());
//! # Ok(())
//! # }
//! ```
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

pub mod apis;
pub mod client_builder;
pub mod models;
pub mod request_validator;
pub mod webhooks;

pub use apis::client::*;
pub use apis::configuration::Configuration;
pub use apis::{Error, ResponseContent};
pub use client_builder::{ClientBuilder, ConfigurationError};
pub use request_validator::ValidationError;
pub use webhooks::{
    verify_webhook_signature, verify_webhook_signature_with_options, WebhookSignatureOptions,
    DEFAULT_WEBHOOK_TOLERANCE,
};
