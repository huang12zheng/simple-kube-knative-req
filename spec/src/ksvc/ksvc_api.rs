//! It want to get the [Api<DynamicObject>]
// ! when developing, what might come to mind:
// !
// ! # Step 1: async fn
// ! ```
// ! #[tokio::test]
// ! async fn test() {
// !     let client = CLIENT.get().await.clone();
// !
// !     let gvk: GroupVersionKind = GVKSpec::from_env().into();
// !
// !     let api: Api<DynamicObject> = ReqBuilder::default()
// !         .client(client)
// !         .gvk(gvk)
// !         .set_ar()
// !         .await
// !         .build()
// !         .into();
// !
// !     println!("{:?}", api);
// !     let str = format!("{:?}", api);
// !     let expect_str = r#"Api { request: Request { url_path: "/apis/serving.knative.dev/v1/namespaces/default/services" }, client: "...", namespace: Some("default") }"#;
// !     assert_eq!(str, expect_str);
// !
// !     let ksvc = KnativeSpec::default().ksvc();
// ! }
// ! ```
// ! # Steps 2: split async fn into multi async fn
// ! # Steps 3: make multi async fn in a trait ~~[KnativeSpecIntoReq]~~ [KnativeSpecIntoApi::get_api] with [async_trait], newtype [KnativeSpecWrapper]
// !
// ! fn in trait [KnativeSpecIntoApi] have:
// ! - get_client
// ! - get_gvk
// ! - get_ar
// ! - get_api
// !
// ! # Last Step:
// ! `api.create(&PostParams::default(), &ksvc).await?;`
// !
// ! # Other Info:
// ! - no need `get_client` in KnativeSpecIntoApi
// !
// ! make `let client =<Self as KnativeSpecIntoApi>::get_client().await;`  into
// ! `let client = CLIENT.get().await.clone();`

use async_trait::async_trait;

// #[allow(unused_imports)]
// use client::CLIENT;
// #[allow(unused_imports)]
// use gvk::*;
use kube::{
    api::{Api, ApiResource},
    core::{DynamicObject, GroupVersionKind},
    Client,
};

use crate::gvk::IntoGVKSpec;

use super::{client::CLIENT, ksvc::KnativeSpec};

// !due to KnativeSpecWrapper, there is no need Req
// !```
// !struct Req {
// !    pub client: Client,
// !    pub ar: ApiResource,
// !}
// !impl Into<Api<DynamicObject>> for Req {
// !    fn into(self) -> Api<DynamicObject> {
// !        let api = Api::<DynamicObject>::default_namespaced_with(self.client.to_owned(), &self.ar);
// !        api
// !    }
// !}
// !```
/// newtype wrapper for [KnativeSpec]
#[derive(Debug)]
pub struct KnativeSpecWrapper(pub KnativeSpec);
#[async_trait]
/// trait for [KnativeSpecWrapper] to get [Api]
pub trait KnativeSpecIntoApi {
    /// merge [Client] and [ApiResource] into [Api]
    async fn get_api(&self) -> Api<DynamicObject>;
    /// Get [ApiResource] from [GroupVersionKind]
    fn get_ar(&self) -> ApiResource;

    /// Get [Client] from [CLIENT]
    async fn get_client(&self) -> &'async_trait Client {
        return CLIENT.get().await;
    }
    /// Get gvk from KnativeSpec with trait [IntoGVKSpec]
    fn get_gvk(&self) -> GroupVersionKind;
}

#[async_trait]
impl KnativeSpecIntoApi for KnativeSpecWrapper {
    async fn get_api(&self) -> Api<DynamicObject> {
        let client = self.get_client().await.clone();
        let ar = self.get_ar();
        let api = Api::<DynamicObject>::default_namespaced_with(client, &ar);
        api
    }

    // !20220713 hz
    // !instead of use async fn, ApiResource::from_gvk(&gvk) do better
    // !async fn get_ar(&self) -> ApiResource {
    // !    let client = CLIENT.get().await;
    // !    let gvk = self.get_gvk();
    // !    let ar = discovery::pinned_kind(client, &gvk).await;
    // !    // insta::assert_debug_snapshot!(ar);
    // !    let (ar, _) = discovery::pinned_kind(client, &gvk).await.unwrap();
    // !    ar
    // !}
    fn get_ar(&self) -> ApiResource {
        return ApiResource::from_gvk(&self.get_gvk());
    }

    fn get_gvk(&self) -> GroupVersionKind {
        let gvk = <KnativeSpec as IntoGVKSpec>::into_gvk(&self.0);
        return gvk.into();
    }
}

#[cfg(test)]
#[tokio::test]
async fn show_ar() {
    use envconfig::Envconfig;
    let gvk: GroupVersionKind = crate::gvk::GVKSpec::init_from_env().unwrap().into();

    let ar = ApiResource::from_gvk(&gvk);
    insta::assert_debug_snapshot!(ar);
}
#[cfg(test)]
#[tokio::test]
async fn show_api() {
    use envconfig::Envconfig;
    let client: Client = CLIENT.get().await.clone();
    let gvk: GroupVersionKind = crate::gvk::GVKSpec::init_from_env().unwrap().into();
    let ar = ApiResource::from_gvk(&gvk);
    let api = Api::<DynamicObject>::default_namespaced_with(client, &ar);
    insta::assert_debug_snapshot!(api);
}
