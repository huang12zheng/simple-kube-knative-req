use crate::*;
/// GVKSpec is spec of [GroupVersionKind]
/// # Example For Default Data
/// ```snap
#[doc = include_str!("snapshots/kube_do_spec__gvk__gvk_spec__tests__gvk_spec.snap")]
/// ```
#[doc(alias = "gvk")] // for Support search function in doc
#[derive(Envconfig, Debug, Clone)]
pub struct GVKSpec {
    /// source from [GroupVersionKind::group]
    #[envconfig(from = "GROUP", default = "serving.knative.dev")]
    pub group: String,
    /// source from [GroupVersionKind::version]
    #[envconfig(from = "VERSION", default = "v1")]
    pub version: String,
    /// source from [GroupVersionKind::kind]
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
