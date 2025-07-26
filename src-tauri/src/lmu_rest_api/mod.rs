//! LMU REST API module
//!
//! This module provides functionality for interacting with the LMU REST API watch endpoints.
//! It is organized into separate submodules for better maintainability:
//!
//! - [`error`] - Error types and result definitions
//! - [`types`] - Data structures used by the API
//! - [`client`] - Main API client implementation
//!
//! # Examples
//!
//! ```rust
//! use lmu_steward_companion::lmu_rest_api::{LmuWatchApi, LmuApiResult};
//!
//! #[tokio::main]
//! async fn main() -> LmuApiResult<()> {
//!     let api = LmuWatchApi::localhost();
//!     
//!     // Check if the API is reachable
//!     if api.health_check().await? {
//!         println!("API is reachable");
//!         
//!         // Get session information
//!         let session_info = api.get_session_info().await?;
//!         println!("Session info: {}", session_info);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod types;

// Re-export the main types for convenience
pub use client::LmuWatchApi;
