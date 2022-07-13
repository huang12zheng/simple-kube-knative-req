//! A library for acquiring a [DynamicObject](kube::core::DynamicObject) spec.
//!
//! - This crate provides a number of schema
//! - The schema would get data from env by [envconfig](envconfig)
//!
//! Detail of Spec:
//!
//! - [GVKSpec](gvk::GVKSpec)
//! - [KnativeSpec](ksvc)
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
//! - [get_default_api](gvk::api::GVKSpecWrapper::get_default_api)
//!
//! # LOOK ALSO
//!
//! - [insta quickstart](https://insta.rs/docs/quickstart/)
//! - [Youtube Insta 0.16](https://www.youtube.com/watch?v=rCHrMqE4JOY&t=654s)

#![deny(rustdoc::missing_doc_code_examples)]
#![warn(missing_docs)]

#[macro_use]
extern crate educe;
use kube::{
    api::{ApiResource, DynamicObject, GroupVersionKind},
    Api, Client,
};

use async_trait::async_trait;
use envconfig::Envconfig;
use serde_json::json;

pub mod gvk;
pub use gvk::*;
pub mod ksvc;
pub use ksvc::*;

trait IntoDynamicObject {
    fn gv(&self) -> String;
    fn into_do(&self) -> kube::api::DynamicObject;
}
