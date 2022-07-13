//! - use [serial_test::serial] to make concurrent test success
//! > it maybe no need remove_var in [test1::kube_gvk_debug]
//! - use [insta::assert_debug_snapshot] to auto change test expect_str

#[cfg(test)]
mod test1 {
    use envconfig::Envconfig;
    use kube::api::GroupVersionKind;
    use kube_do_spec::gvk::GVKSpec;
    use serial_test::serial;

    #[test]
    #[serial]
    fn kube_gvk_debug() {
        // do `remove_var` for pall with proptest_gvk_none.
        std::env::remove_var("GROUP");
        std::env::remove_var("VERSION");
        std::env::remove_var("KIND");
        // std::thread::sleep_ms(200);
        let gvk: GroupVersionKind = GVKSpec::init_from_env().unwrap().into();

        insta::assert_debug_snapshot!(gvk, @r###"
        GroupVersionKind {
            group: "serving.knative.dev",
            version: "v1",
            kind: "Service",
        }
        "###);
    }
}

use envconfig::Envconfig;
use kube::api::GroupVersionKind;
use kube_do_spec::gvk::GVKSpec;
use proptest::proptest;
use serial_test::serial;
proptest! {
    #[test]
    #[serial]
    fn proptest_gvk(g in "\\PC*" ,v in "\\PC*",k in "\\PC*") {
        std::env::set_var("GROUP", g.clone());
        std::env::set_var("VERSION", v.clone());
        std::env::set_var("KIND", k.clone());
        let gvk: GroupVersionKind = GVKSpec::init_from_env().unwrap().into();
        let kube_gvk = GroupVersionKind::gvk(&g,&v,&k );
        assert_eq!(format!("{:?}", kube_gvk),format!("{:?}",gvk));

    }
}
#[cfg(test)]
mod test2_proptest {
    use envconfig::Envconfig;
    use kube::api::GroupVersionKind;
    use kube_do_spec::gvk::GVKSpec;
    use serial_test::serial;
    #[test]
    #[serial]
    fn proptest_gvk_none() {
        std::env::set_var("GROUP", "");
        std::env::set_var("VERSION", "");
        std::env::set_var("KIND", "");
        // !would get error due to `set_var`
        // !use #[serial]
        let gvk: GroupVersionKind = GVKSpec::init_from_env().unwrap().into();
        // let kube_gvk = GroupVersionKind::gvk(g.clone()),v.clone(),&k.clone() );
        let kube_gvk = GroupVersionKind::gvk("", "", "");
        insta::assert_snapshot!(format!("{:?}", kube_gvk), @r###"GroupVersionKind { group: "", version: "", kind: "" }"###);
        insta::assert_snapshot!(format!("{:?}", gvk),      @r###"GroupVersionKind { group: "", version: "", kind: "" }"###);
    }

    #[test]
    #[serial]

    fn proptest_gvk_case2() {
        std::env::set_var("GROUP", "");
        std::env::set_var("VERSION", "");
        std::env::set_var("KIND", "🫗");
        // !would get error due to `set_var`
        let gvk: GroupVersionKind = GVKSpec::init_from_env().unwrap().into();
        // let kube_gvk = GroupVersionKind::gvk(g.clone()),v.clone(),&k.clone() );
        let kube_gvk = GroupVersionKind::gvk("", "", "🫗");
        insta::assert_snapshot!(format!("{:?}", kube_gvk), @r###"GroupVersionKind { group: "", version: "", kind: "🫗" }"###);
        insta::assert_snapshot!(format!("{:?}", gvk),      @r###"GroupVersionKind { group: "", version: "", kind: "🫗" }"###);
        // !fail
        // insta::assert_snapshot!(kube_gvk.kind, gvk.kind);
        // insta::assert_snapshot!("🫗", "🫗");
        // !success
        // insta::assert_snapshot!(kube_gvk.kind, @"🫗");
        // insta::assert_snapshot!(gvk.kind,@"🫗");
    }
}
