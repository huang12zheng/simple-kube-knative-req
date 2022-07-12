#[macro_use]
extern crate educe;

use kube::core::DynamicObject;
use serde_json::{json, Value};

#[derive(Educe)]
#[educe(Default, Debug)]
struct KnativeSpec {
    #[educe(Default = "serving.knative.dev")]
    group: String,
    version: String,
    #[educe(Default = "Service")]
    kind: String,
    // #[educe(Defaul(ignore))]
    #[educe(Default = "my-coder")]
    container_name: String,
    #[educe(Default = "hzgood/my-coder")]
    svc_image: String,
    #[educe(Default = 8080)]
    port: i32,
    env: Value,
}

trait IntoDynamicObject {
    fn gv(&self) -> String;
    fn into_do(&self) -> DynamicObject;
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

#[test]
fn spec_works() {
    let spec = KnativeSpec {
        ..Default::default()
    };
    dbg!(spec);
    // assert_eq!(2 + 2, 4);
}

#[test]
fn show_into_do() {
    let spec = KnativeSpec {
        container_name: "my-coder".to_owned(),
        ..Default::default()
    };
    let ksvc = spec.into_do();
    insta::assert_debug_snapshot!(ksvc)
}
fn main() {}
