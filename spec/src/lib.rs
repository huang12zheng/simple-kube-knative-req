//! A library for acquiring a [DynamicObject](kube::core::DynamicObject) spec.
//!
//! - This crate provides a number of schema
//! - The schema would get data from env by [envconfig](envconfig)
//!
//! Detail of Content:
//!
//! - [GVKSpec]
//! - [KnativeSpec]
//! - [GetApi]
//! - [BuilderApi]
//! - [IntoDynamicObject]
//!
//! # Usage
//!
//! First, add this to your Cargo.toml
//! ```toml
//! [dependencies]
//! kube-do-spec = "^0.1"
//! ```
//! # Example
//!
//! - [get_default_api](get_default_api)
//!
//! # LOOK ALSO
//!
//! - [insta quickstart](https://insta.rs/docs/quickstart/)
//! - [Youtube Insta 0.16](https://www.youtube.com/watch?v=rCHrMqE4JOY&t=654s)
//! - run test use `INSTA_UPDATE=1 cargo test --lib`
//! - gen doc  use `cargo doc -Zunstable-options -Zrustdoc-scrape-examples=all --open --document-private-items`

// #![deny(rustdoc::missing_doc_code_examples)]
#![warn(missing_docs)]
// #![feature(doc_cfg)]

#[macro_use]
extern crate educe;
use kube::{
    api::{ApiResource, DynamicObject, GroupVersionKind},
    Api, Client,
};

use async_trait::async_trait;
use envconfig::Envconfig;
use serde_json::{json, Value};

pub mod gvk;
pub use gvk::*;
pub mod ksvc;
pub use ksvc::*;

/// A trait to get [DynamicObject]
/// ```snap
// #[doc = include_str!("ksvc/snapshots/kube_do_spec__ksvc__ksvc__show_do.snap")]
/// include_str!("ksvc/snapshots/kube_do_spec__ksvc__ksvc__show_do.snap")
/// ```
pub trait IntoDynamicObject {
    /// [] into GV::String
    fn gv(&self) -> String;
    /// [] into [DynamicObject]
    fn into_do(&self) -> kube::api::DynamicObject;
}
