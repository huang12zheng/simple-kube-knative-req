use std::fmt;

use async_once::AsyncOnce;
use async_trait::async_trait;
use kube::{
    api::{Api, ApiResource, PostParams},
    core::{DynamicObject, GroupVersionKind},
    discovery, Client,
};
use lazy_static::lazy_static;
use serde_json::json;

struct Req {
    pub client: Client,
    pub gvk: GroupVersionKind,
    pub ar: ApiResource,
    pub api: Api<DynamicObject>,
}

#[derive(Default)]
struct ReqBuilder {
    pub client: Option<Client>,
    pub gvk: Option<GroupVersionKind>,
    pub ar: Option<ApiResource>,
    pub api: Option<Api<DynamicObject>>,
}

#[allow(dead_code)]
impl ReqBuilder {
    pub fn client<Value: Into<Client>>(&mut self, value: Value) -> &mut Self {
        self.client = Some(value.into());
        self
    }
    pub fn gvk<Value: Into<GroupVersionKind>>(&mut self, value: Value) -> &mut Self {
        self.gvk = Some(value.into());
        self
    }

    pub async fn set_ar<Value: Into<ApiResource>>(&mut self) -> &mut Self {
        let client = self.client.clone().unwrap();
        let gvk = self.gvk.clone().unwrap();
        let (ar, _) = discovery::pinned_kind(&client, &gvk).await.unwrap();
        // let api = Api::<DynamicObject>::default_namespaced_with(client, &ar);
        self.ar = Some(ar);
        self
    }
    // pub fn api<Value: Into<Client>>(&mut self, value: Value) -> &mut Self {
    //     self.client = Some(value.into());
    //     self
    // }
}

// #[async_trait]
// impl Default for Client {
//     async fn default() -> Self {
//         // Self { inner: Default::default(), default_ns: Default::default() }
//         let client = Client::try_default().await?;
//         client
//     }
// }

lazy_static! {
    static ref CLIENT: AsyncOnce<Client> =
        AsyncOnce::new(async { Client::try_default().await.unwrap() });
}

#[macro_use]
extern crate derive_builder;

// #[derive(Builder, Debug)]
// #[builder(setter(into))]
pub struct GKVSpec {
    pub group: String,
    pub version: String,
    pub kind: String,
}

impl GKVSpec {
    fn from_env() -> Self {
        let group = std::env::var("GROUP").unwrap_or_else(|_| "serving.knative.dev".into());
        let version = std::env::var("VERSION").unwrap_or_else(|_| "v1".into());
        let kind = std::env::var("KIND").unwrap_or_else(|_| "Service".into());
        Self {
            group,
            version,
            kind,
        }
    }
}

impl Into<GroupVersionKind> for GKVSpec {
    fn into(self) -> GroupVersionKind {
        let gkv = GroupVersionKind::gvk(&self.group, &self.version, &self.kind);
        gkv
    }
}

#[tokio::test]
async fn test() {
    let client = CLIENT.await;

    let gvk: GroupVersionKind = GKVSpec::from_env().into();

    let ch = ReqBuilder::default().client(client).gvk(gvk);
    // .special_info(42u8)
    // .token(19124)
    // .build()
    // .unwrap();
    // println!("{:?}", ch);
}

fn main() {}
