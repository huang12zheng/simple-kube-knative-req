// use std::fmt;

// use async_trait::async_trait;
// use kube::{
//     api::{Api, ApiResource, PostParams},
//     core::{DynamicObject, GroupVersionKind},
//     discovery, Client,
// };
// use serde_json::json;

// #[macro_use]
// extern crate derive_builder;

// #[builder(setter(into))]
// #[derive(Builder)]
// struct Req {
//     pub client: Client,
//     pub gvk: GroupVersionKind,
//     pub ar: ApiResource,
//     pub api: Api<DynamicObject>,
// }

// fn main() {
//     let ch = ReqBuilder::default();
//     // .special_info(42u8)
//     // .token(19124)
//     // .build()
//     // .unwrap();
//     // println!("{:?}", ch);
// }
