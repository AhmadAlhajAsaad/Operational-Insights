//! Person management module (FR-005)

pub mod gid_matcher;
pub mod repository;
pub mod types;

pub use gid_matcher::GidMatcher;
pub use repository::PersonRepository;
pub use types::*;
