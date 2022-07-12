use envconfig::Envconfig;
use kube_do_spec::gvk::GVKSpec;

#[test]
fn gvk_doc() {
    let gvk = GVKSpec::init_from_env().unwrap();
    insta::assert_debug_snapshot!(gvk, @r###"
    GVKSpec {
        group: "serving.knative.dev",
        version: "v1",
        kind: "Service",
    }
    "###);
    insta::assert_snapshot!(gvk.group, @"serving.knative.dev");
    insta::assert_snapshot!(gvk.kind, @"Service");
}
