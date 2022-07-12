//! GVKSpec is spec of [GroupVersionKind]
use envconfig::Envconfig;
use kube::api::GroupVersionKind;

/// GVKSpec is spec of [GroupVersionKind]
// !too simple to use #[allow(missing_docs)]
// !#[doc(alias = "gvk")] for Support search function
#[doc(alias = "gvk")]
#[derive(Envconfig, Debug)]
#[allow(missing_docs)]
pub struct GVKSpec {
    // !pub due to trait [Into]
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
    // ! ```output
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

/// From [KnativeSpec] into [GVKSpec]
pub trait IntoGVKSpec {
    /// From [KnativeSpec] into [GVKSpec]
    fn into_gvk(&self) -> GVKSpec;
}

impl Into<GroupVersionKind> for GVKSpec {
    fn into(self) -> GroupVersionKind {
        let gkv = GroupVersionKind::gvk(&self.group, &self.version, &self.kind);
        gkv
    }
}
