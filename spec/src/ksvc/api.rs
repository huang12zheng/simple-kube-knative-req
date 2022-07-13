//! In the end, [] ref to [GVKSpecWrapper] and [BuilderApi].
//!
//! We need From [KnativeSpec] Into [GVKSpecWrapper],
//!
//! LOOK ALSO:
//! - [src/gvk/api.md]
//! - [BuilderApi]

use crate::*;

#[async_trait]
impl GetApi for KnativeSpec {
    async fn get_api(&self) -> Api<DynamicObject> {
        let gvk = self.into_gvk();
        return gvk.get_api().await;
    }
}

#[cfg(test)]
mod test {
    use crate::{GetApi, KnativeSpec};

    // #[cfg(test)]
    // use crate::ksvc_into_gvk;

    #[tokio::test]
    async fn show_api() {
        let spec = KnativeSpec::default();
        let api = spec.get_api().await;
        insta::assert_debug_snapshot!(api);
    }
}
