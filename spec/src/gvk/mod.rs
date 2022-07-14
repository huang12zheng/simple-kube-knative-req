// #![allow(rustdoc::missing_doc_code_examples)]
mod api;
pub use api::{get_default_api, GetApi};
mod client;
pub use client::get_client;
mod gvk_spec;
pub use gvk_spec::{GVKSpec, IntoGVKSpec};
