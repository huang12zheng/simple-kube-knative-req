use crate::*;

#[doc = include_str!("gvk_spec.md")]
#[doc = include_str!("snapshots/kube_do_spec__gvk__gvk_spec__tests__gvk_spec.snap")]
// !#[doc(alias = "gvk")] for Support search function
#[doc(alias = "gvk")]
#[derive(Envconfig, Debug, Clone)]
#[allow(missing_docs)] // !too simple to use #[allow(missing_docs)]
pub struct GVKSpec {
    #[envconfig(from = "GROUP", default = "serving.knative.dev")]
    pub group: String,
    // ! # 20220711 hz
    // ! use `#[envconfig(from = "VERSION")]` would get error:
    // ! - source
    // ! ```
    // ! use envconfig::Envconfig;
    // ! use kube_do_spec::gvk::GVKSpec;
    // !
    // ! let gvk = GVKSpec::init_from_env();
    // ! dbg!(gvk);
    // ! ```
    // ! - output
    // ! ```rust,no_run
    // ! Err(
    // !     EnvVarMissing {
    // !         name: "VERSION",
    // !     },
    // ! )
    // ! ```
    #[envconfig(from = "VERSION", default = "v1")]
    pub version: String,
    #[envconfig(from = "KIND", default = "Service")]
    pub kind: String,
}

/// From generic [T] into [GVKSpec], such as [crate::ksvc::KnativeSpec]
/// Detail are in [crate::ksvc::KnativeSpec::into_gvk]
pub trait IntoGVKSpec {
    /// From [KnativeSpec] into [GVKSpec]
    fn into_gvk(&self) -> GVKSpec;
}

impl From<GVKSpec> for GroupVersionKind {
    fn from(val: GVKSpec) -> Self {
        let gkv = GroupVersionKind::gvk(&val.group, &val.version, &val.kind);
        gkv
    }
}

#[cfg(test)]
mod tests {
    use crate::gvk::GVKSpec;
    use envconfig::Envconfig;

    /// [GVKSpec::init_from_env]
    /// snap would use by doc of [GVKSpec]
    #[test]
    fn gvk_spec() {
        let gvk = GVKSpec::init_from_env().unwrap();
        insta::assert_debug_snapshot!(gvk);
    }
}
