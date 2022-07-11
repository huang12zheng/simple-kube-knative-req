mod client;
mod gvk;

#[allow(unused_imports)]
use client::CLIENT;
#[allow(unused_imports)]
use gvk::*;
use kube::{
    api::{Api, ApiResource},
    core::{DynamicObject, GroupVersionKind},
    discovery, Client,
};

struct Req {
    pub client: Client,
    // pub gvk: GroupVersionKind,
    pub ar: ApiResource,
}

#[derive(Default)]
struct ReqBuilder {
    pub client: Option<Client>,
    pub gvk: Option<GroupVersionKind>,
    pub ar: Option<ApiResource>,
    // pub api: Option<Api<DynamicObject>>,
}

#[allow(dead_code)]
impl ReqBuilder {
    pub fn client<Value: Into<Client>>(&mut self, value: Value) -> &mut Self {
        self.client = Some(value.into());
        self
    }
    // pub async fn set_client<Value: Into<Client>>(&mut self, value: Value) -> &mut Self {
    //     self.client = Some(CLIENT.await);
    //     self
    // }
    pub fn gvk<Value: Into<GroupVersionKind>>(&mut self, value: Value) -> &mut Self {
        self.gvk = Some(value.into());
        self
    }

    pub async fn set_ar(&mut self) -> &mut Self {
        // let client = self.client.clone().unwrap();
        let client = self.client.clone().unwrap();
        let gvk = self.gvk.clone().unwrap();
        let (ar, _) = discovery::pinned_kind(&client, &gvk).await.unwrap();
        self.ar = Some(ar);
        self
    }

    pub fn build(&self) -> Req {
        Req {
            client: self.client.clone().unwrap(),
            // gvk: self.gvk.clone().unwrap(),
            ar: self.ar.clone().unwrap(),
        }
    }
}

impl Into<Api<DynamicObject>> for Req {
    fn into(self) -> Api<DynamicObject> {
        let api = Api::<DynamicObject>::default_namespaced_with(self.client.clone(), &self.ar);
        api
    }
}

#[tokio::test]
async fn test() {
    let client = CLIENT.get().await.clone();

    let gvk: GroupVersionKind = GKVSpec::from_env().into();

    let api: Api<DynamicObject> = ReqBuilder::default()
        .client(client)
        .gvk(gvk)
        .set_ar()
        .await
        .build()
        .into();

    println!("{:?}", api);
    let str = format!("{:?}", api);
    // let expect_str = "Api { request: Request { url_path: \"/apis/serving.knative.dev/v1/namespaces/default/services\" }, client: \"...\", namespace: Some(\"default\") }";
    let expect_str = r#"Api { request: Request { url_path: "/apis/serving.knative.dev/v1/namespaces/default/services" }, client: "...", namespace: Some("default") }"#;
    assert_eq!(str, expect_str);
}

fn main() {}
