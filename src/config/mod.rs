//! Configuration management module.

mod loader;
mod profile;

pub use loader::{load_config, Config, ConfigPaths};
pub use profile::{Profile, ProfileStore};
