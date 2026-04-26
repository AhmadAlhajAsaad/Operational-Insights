//! Equans Operational Insights Backend Library
//!
//! Re-exports all backend modules so integration tests (in tests/) and
//! utility binaries (in src/bin/) can access the application internals.

#![allow(dead_code)]

pub mod atlassian;
pub mod auth;
pub mod cache;
pub mod config;
pub mod error;
pub mod github;
pub mod github_cache;
pub mod github_link;
pub mod health;
pub mod imports;
pub mod jobs;
pub mod organizations;
pub mod persons;
pub mod routes;
pub mod security;
