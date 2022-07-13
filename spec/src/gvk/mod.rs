//! assembly GVK
mod api;
pub use api::{get_default_api, GetApi};
mod client;
pub use client::CLIENT;
mod gvk_spec;
pub use gvk_spec::{GVKSpec, IntoGVKSpec};
