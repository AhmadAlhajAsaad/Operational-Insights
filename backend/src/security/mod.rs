//! Security module for privacy and data protection measures (TM-04, TS-06)
//!
//! Implements the Privacy & Security Plan requirements:
//! - PII masking in logs and API responses
//! - Security headers middleware
//! - GDPR-compliant data handling

pub mod headers;
pub mod masking;
