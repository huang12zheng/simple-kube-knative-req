// use std::convert::Infallible;

// use async_trait::async_trait;
// use cucumber::{gherkin::Step, given, then, when, World, WorldInit};
// use envconfig::Envconfig;
// use insta::assert_debug_snapshot;
// use kube_do_spec::ksvc::ksvc::KnativeSpec;
// use kube_do_spec::ksvc::ksvc_api::{KnativeSpecIntoApi, KnativeSpecWrapper};

// // #[derive(Debug)]

// // #[derive(Debug, WorldInit)]
// // struct ApiWorld {
// //     // user: Option<String>,
// //     // capacity: usize,
// //     // api: Api<DynamicObject>,
// //     knative_spec: KnativeSpec,
// //     // knative_wrapper: KnativeSpecWrapper,
// // }

// // #[async_trait(?Send)]
// // impl World for ApiWorld {
// //     type Error = Infallible;

// //     async fn new() -> Result<Self, Self::Error> {
// //         Ok(ApiWorld {
// //             knative_spec: KnativeSpec::default(),
// //         })
// //     }
// // }
// #[derive(Debug, WorldInit)]
// struct ApiWorld {
//     knative_wrapper: KnativeSpecWrapper,
//     insta_label: String,
// }

// #[async_trait(?Send)]
// impl World for ApiWorld {
//     type Error = Infallible;

//     async fn new() -> Result<Self, Self::Error> {
//         Ok(ApiWorld {
//             knative_wrapper: KnativeSpecWrapper(KnativeSpec::default()),
//             insta_label: "".to_owned(),
//         })
//     }
// }

// #[given(expr = "No set var to environment, use default ApiWorld")]
// fn get_gvk_and_ksvc(w: &mut ApiWorld) {}

// #[given(expr = "Port and Image")]
// fn port_and_image(w: &mut ApiWorld, step: &Step) {
//     if let Some(table) = step.table.as_ref() {
//         for row in table.rows.iter().skip(1) {
//             // NOTE: skip header
//             let port = &row[0];
//             let image = &row[1];

//             std::env::set_var("PORT", port);
//             std::env::set_var("IMAGE", image);
//             w.knative_wrapper = KnativeSpecWrapper(KnativeSpec::init_from_env().unwrap());
//             w.insta_label = format!("{}-{}", port, image);
//         }
//     }
// }

// // !error from let wrapper = KnativeSpecWrapper(&w.knative_spec.clone());
// // !use let wrapper = &w.knative_wrapper; and World{knative_wrapper: KnativeSpecWrapper,}

// // #[then(expr = "I get api")]
// async fn show_api(w: &mut ApiWorld) {
//     let wrapper = &w.knative_wrapper;
//     assert_debug_snapshot!(wrapper);
//     let api = wrapper.get_api().await;

//     assert_eq!(1, 1);
//     // insta_util::set_snapshot_suffix!("{}", w.insta_label);
//     // assert_debug_snapshot!(api);
// }

// #[tokio::test]
// async fn do_debug_show_api() {
//     // ApiWorld::run("tests/features/get_api.feature").await;
//     // show_api(ApiWorld());
//     // let mut w = ApiWorld::new().await.unwrap();
//     // let wrapper = &w.knative_wrapper;
//     let wrapper = KnativeSpecWrapper(KnativeSpec::default());
//     // assert_debug_snapshot!(wrapper);
//     let api = wrapper.get_api().await;
// }

// mod insta_util {
//     macro_rules! set_snapshot_suffix {
//         ($($expr:expr),*) => {{
//             let mut settings = insta::Settings::clone_current();
//             settings.set_snapshot_suffix(format!($($expr,)*));
//             settings.bind_to_thread();
//         }}
//     }
//     pub(crate) use set_snapshot_suffix;
// }
// // #[macro_use]

// #[tokio::main]
// async fn main() {
//     ApiWorld::run("tests/features/get_api.feature").await;
// }
fn main() {}
