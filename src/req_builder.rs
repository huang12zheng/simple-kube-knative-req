use kube::{
    api::{Api, ApiResource, PostParams},
    core::{DynamicObject, GroupVersionKind},
    discovery, Client,
};
use serde_json::json;

#[macro_use]
extern crate derive_builder;

// #[allow(dead_code)]
#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
struct ReqBuilder {
    pub client: Client,
    pub gvk: GroupVersionKind,
    pub ar: ApiResource,
    pub api: Api<DynamicObject>,
}

fn main() {
    let ch = ReqBuilder::default();
    // .special_info(42u8)
    // .token(19124)
    // .build()
    // .unwrap();
    println!("{:?}", ch);
}
