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
//! ```rust
// #![doc = include_str!("../examples/doc.rs")]
#![doc = include_str!("../tests/gvk_doc.rs")]
//! ```
//!
//! # LOOK ALSO
//!
//! - [insta quickstart](https://insta.rs/docs/quickstart/)
//! - [Youtube Insta 0.16](https://www.youtube.com/watch?v=rCHrMqE4JOY&t=654s)

#![deny(rustdoc::missing_doc_code_examples)]
#![warn(missing_docs)]

#[macro_use]
extern crate educe;

pub mod gvk;
pub mod ksvc;
// mod ksvc;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
