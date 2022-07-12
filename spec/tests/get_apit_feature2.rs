// use envconfig::Envconfig;
// use kube::{core::GroupVersionKind, discovery, Client};
// use kube_do_spec::{
//     gvk::GVKSpec,
//     ksvc::{
//         ksvc::KnativeSpec,
//         ksvc_api::{KnativeSpecIntoApi, KnativeSpecWrapper},
//     },
// };

// #[tokio::test]
// async fn aa() {
//     // ApiWorld::run("tests/features/get_api.feature").await;
//     // show_api(ApiWorld());
//     // let mut w = ApiWorld::new().await.unwrap();
//     // let wrapper = &w.knative_wrapper;
//     // let wrapper = KnativeSpecWrapper(KnativeSpec::default());
//     // // assert_debug_snapshot!(wrapper);
//     // let api = wrapper.get_api().await;
//     // let client = Client::try_default().await?;
//     // let gvk = GroupVersionKind::gvk("apiregistration.k8s.io", "v1", "APIService");

//     let client = Client::try_default().await.unwrap();
//     let gvk: GroupVersionKind = GVKSpec::init_from_env().unwrap().into();

//     let (ar, caps) = discovery::pinned_kind(&client, &gvk).await.unwrap();
//     insta::assert_debug_snapshot!(ar);
// }
