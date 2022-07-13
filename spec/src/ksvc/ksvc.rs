//! [KnativeSpec] is all spec from knative service yaml
//! it impl some method:
//! - [IntoDynamicObject::into_do] to return [DynamicObject] for [Api]
//! - [IntoDynamicObject::gv] to get String like "serving.knative.dev/v1"
//! - [IntoGVKSpec] into [GVKSpec] with clone
use crate::*;

/// [struct@KnativeSpec] is all spec from knative service yaml
#[derive(Educe)]
#[educe(Default)]
#[derive(Envconfig, Debug)]
pub struct KnativeSpec {
    /// source from [GVKSpec::group]
    #[envconfig(from = "GROUP", default = "serving.knative.dev")]
    #[educe(Default = "serving.knative.dev")]
    pub group: String,
    /// source from [GVKSpec::version]
    #[educe(Default = "v1")]
    #[envconfig(from = "VERSION", default = "v1")]
    pub version: String,
    #[envconfig(from = "KIND", default = "Service")]
    #[educe(Default = "Service")]
    /// source from [GVKSpec::kind]
    pub kind: String,
    #[envconfig(from = "CONTAINER_NAME", default = "my-coder")]
    #[educe(Default = "my-coder")]
    container_name: String,
    #[envconfig(from = "IMAGE", default = "hzgood/my-coder")]
    #[educe(Default = "hzgood/my-coder")]
    svc_image: String,
    #[envconfig(from = "PORT", default = "8080")]
    #[educe(Default = "8080")]
    port: String,
    #[envconfig(from = "ENV", default = "")]
    #[educe(Default = "")]
    env: String,
}

// ! impl KnativeSpec {
// !     /// Returns the gv of this [`KnativeSpec`].
// !     fn gv(&self) -> String {
// !         if self.version == "" {
// !             self.group.clone()
// !         } else {
// !             format!("{}/{}", self.group, self.version)
// !         }
// !     }
// !     pub fn ksvc(self) -> anyhow::Result<DynamicObject> {
// !         let ksvc: DynamicObject = serde_json::from_value(json!({
// !         "apiVersion": self.gv(),
// !         "kind": self.kind,
// !         "metadata": {
// !             // "name": format!("{}",container_name),
// !             "name": self.container_name,
// !         },
// !         "spec": {
// !             "template":{
// !                 "spec": {
// !                     "containers": [{
// !                         "ports": [
// !                             {
// !                                 "containerPort": self.port
// !                             }
// !                         ],
// !                         "image": self.svc_image,
// !                         "env": self.env
// !                     }],
// !                 }
// !             }
// !         }
// !         }))?;
// !         Ok(ksvc)
// !     }
// ! }
impl IntoDynamicObject for KnativeSpec {
    /// Returns the gv of this [`KnativeSpec`].
    fn gv(&self) -> String {
        if self.version == "" {
            self.group.clone()
        } else {
            format!("{}/{}", self.group, self.version)
        }
    }
    fn into_do(&self) -> DynamicObject {
        let ksvc: DynamicObject = serde_json::from_value(json!({
        "apiVersion": self.gv(),
        "kind": self.kind,
        "metadata": {
            // "name": format!("{}",container_name),
            "name": self.container_name,
        },
        "spec": {
            "template":{
                "spec": {
                    "containers": [{
                        "ports": [
                            {
                                "containerPort": self.port
                            }
                        ],
                        "image": self.svc_image,
                        "env": self.env
                    }],
                }
            }

        }
        }))
        .unwrap();
        // Ok(ksvc)
        ksvc
    }
}

/// Look more by [IntoGVKSpec]
impl IntoGVKSpec for KnativeSpec {
    fn into_gvk(&self) -> GVKSpec {
        GVKSpec {
            group: self.group.clone(),
            version: self.version.clone(),
            kind: self.kind.clone(),
        }
    }
}

#[cfg(test)]
pub mod test {

    use super::KnativeSpec;
    use envconfig::Envconfig;

    /// test [Envconfig::init_from_env]
    #[test]
    pub fn spec_works() {
        let spec = KnativeSpec::init_from_env().unwrap();
        insta::assert_debug_snapshot!(spec)
    }

    /// test [Default::default]
    #[test]
    pub fn default_works() {
        let spec = KnativeSpec {
            ..Default::default()
        };
        insta::assert_debug_snapshot!(spec)
    }

    /// test [IntoDynamicObject::into_do]
    #[test]
    pub fn show_do() {
        let spec = KnativeSpec::init_from_env().unwrap();
        // let ksvc = spec.into_do();
        // insta::assert_debug_snapshot!(ksvc)
    }

    /// test [IntoGVKSpec::into_gvk]
    #[test]
    fn ksvc_into_gvk() {
        let ksvc_spec = KnativeSpec::init_from_env().unwrap();
        // let gvk: GVKSpec = ksvc_spec.into_gvk();

        // insta::assert_debug_snapshot!(gvk);
    }
}
