use crate::*;

/// [KnativeSpec] with [IntoGVKSpec] Into [GVKSpec]
/// - `self.into_gvk()`
///
/// [GVKSpec] with [GetApi] into [Api<DynamicObject>]
/// - `gvk.get_api().await`
///
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
