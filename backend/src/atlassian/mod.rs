//! Atlassian module
//!
//! Provides integration with Atlassian Admin API for user, group, and license management.

pub mod client;
pub mod error;
pub mod link_service;
pub mod service;
pub mod types;

pub use client::AtlassianClient;
pub use error::ServiceError;
pub use link_service::AtlassianLinkService;
pub use service::AtlassianService;
