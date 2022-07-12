// use kube::{
//     api::{Api, ApiResource, PostParams},
//     core::{DynamicObject, GroupVersionKind},
//     discovery, Client,
// };
// use serde_json::json;

// #[macro_use]
// extern crate derive_builder;

// #[derive(Default, Builder, Debug)]
// #[builder(setter(into))]
// struct Channel {
//     token: i32,
//     special_info: i32,
//     // .. a whole bunch of other fields ..
// }

// // struct DynamicObjectReq {
// //     pub client: Client,
// //     pub gvk: GroupVersionKind,
// //     // pub ar:  ApiResource,
// //     // pub api: Api<DynamicObject>
// //     pub ar: ApiResource,
// //     pub api: Api<DynamicObject>,
// // }

// // #[derive(Default, Builder, Debug)]
// // pub struct ReqBuilder {
// //     pub client: Client,
// //     pub gvk: GroupVersionKind,
// //     pub ar: ApiResource,
// //     pub api: Api<DynamicObject>,
// // }
// // impl ReqBuilder {
// //     // #[must_use]
// //     // pub fn new(client: Client, gvk: GroupVersionKind) -> Self {
// //     //     // let (ar, _) = discovery::pinned_kind(&client, &gvk).await?;
// //     //     Self { client, gvk, ar: None, api: None }
// //     // }

// //     async fn set_ar(mut self) -> anyhow::Result<()> {
// //         let (ar, _) = discovery::pinned_kind(&self.client, &self.gvk).await?;
// //         self.ar = ar;
// //         Ok(())
// //     }
// //     fn set_api(mut self) {
// //         self.api = Api::<DynamicObject>::default_namespaced_with(self.client, &self.ar);
// //         // Ok(())
// //     }

// //     fn init(mut self) {}
// // }

// // impl DynamicObjectReq {
// //     // async fn create(client: &Client, gvk: &GroupVersionKind) -> anyhow::Result<()> {
// //     async fn create(self) -> anyhow::Result<()> {
// //         println!("");
// //         let (ar, _) = discovery::pinned_kind(&self.client, &self.gvk).await?;
// //         /// * get api object by input `gvk`.
// //         /// [ ] use namespace var instead of default
// //         /// > LOOK ALSO: 404 with `all_with`
// //         let api = Api::<DynamicObject>::default_namespaced_with(self.client, &ar);

// //         let _r = api.create(&PostParams::default(), &ksvc).await?;

// //         #[cfg(debug_assertions)]
// //         println!("{:?}", _r);

// //         Ok(())
// //     }
// // }
