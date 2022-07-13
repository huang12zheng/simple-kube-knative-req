use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{gherkin::Step, given, then, World, WorldInit};
use envconfig::Envconfig;
use insta::assert_debug_snapshot;
use kube_do_spec::{ksvc::KnativeSpec, GetApi};

#[derive(Debug, WorldInit)]
struct ApiWorld {
    knative_spec: KnativeSpec,
    insta_label: String,
}

#[async_trait(?Send)]
impl World for ApiWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(ApiWorld {
            knative_spec: KnativeSpec::default(),
            insta_label: "label".to_owned(),
        })
    }
}

#[given(expr = "No set var to environment, use default ApiWorld")]
fn get_gvk_and_ksvc(_w: &mut ApiWorld) {}

#[given(expr = "Port and Image")]
fn port_and_image(w: &mut ApiWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            // NOTE: skip header
            let port = &row[0];
            let image = &row[1];

            std::env::set_var("PORT", port);
            std::env::set_var("IMAGE", image);
            w.knative_spec = KnativeSpec::init_from_env().unwrap();
            w.insta_label = format!("{}-{}", port, image);
        }
    }
}

#[then(expr = "I get api")]
async fn show_api(w: &mut ApiWorld) {
    let knative_spec = &w.knative_spec;
    insta_util::set_snapshot_suffix!("{}", w.insta_label);
    assert_debug_snapshot!(knative_spec);

    let api = knative_spec.get_api().await;
    assert_debug_snapshot!(api);
}

mod insta_util {
    macro_rules! set_snapshot_suffix {
        ($($expr:expr),*) => {{
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_suffix(format!($($expr,)*));
            settings.bind_to_thread();
        }}
    }
    pub(crate) use set_snapshot_suffix;
}

#[tokio::main]
async fn main() {
    ApiWorld::run("tests/features/get_api.feature").await;
}
