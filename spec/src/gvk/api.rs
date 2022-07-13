#![doc = include_str!("api.md")]

use crate::*;

/// newtype wrapper for [KnativeSpec]
#[async_trait]
/// trait for [KnativeSpec] to get [Api]
trait BuilderApi {
    /// merge [Client] and [ApiResource] into [Api]
    ///
    /// Usually, use data from [GVKSpecWrapper]. Such example as [GVKSpecWrapper::get_api].
    async fn builder_api(&self) -> Api<DynamicObject>;
    /// Get [ApiResource] from [GroupVersionKind]
    fn get_ar(&self) -> ApiResource;

    /// Get [Client] from [CLIENT]
    async fn get_client(&self) -> &'async_trait Client {
        return CLIENT.get().await;
    }
    /// Get [GroupVersionKind] from [GVKSpec] with trait [IntoGVKSpec]
    fn get_gvk(&self) -> GroupVersionKind;
}

/// A example ref to [BuilderApi]
#[async_trait]
pub trait GetApi {
    /// A example ref to [BuilderApi::builder_api]
    async fn get_api(&self) -> Api<DynamicObject>;
}

#[async_trait]
impl GetApi for GVKSpec {
    async fn get_api(&self) -> Api<DynamicObject> {
        self.builder_api().await
    }
}

#[async_trait]
impl BuilderApi for GVKSpec {
    async fn builder_api(&self) -> Api<DynamicObject> {
        let client = self.get_client().await.clone();
        let ar = self.get_ar();
        let api = Api::<DynamicObject>::default_namespaced_with(client, &ar);
        api
    }

    fn get_ar(&self) -> ApiResource {
        return ApiResource::from_gvk(&self.get_gvk());
    }

    fn get_gvk(&self) -> GroupVersionKind {
        let gvk: GroupVersionKind = self.clone().into();
        gvk
    }
}

impl GVKSpec {
    /// Associated function [get_default_api] use data from [Envconfig]
    /// From this point of view, it's different with [GVKSpecWrapper::get_api]
    #[doc = include_str!("api.md")]
    async fn get_default_api() -> Api<DynamicObject> {
        let client: Client = CLIENT.get().await.clone();
        let gvk: GroupVersionKind = GVKSpec::init_from_env().unwrap().into();
        let ar = ApiResource::from_gvk(&gvk);
        let api = Api::<DynamicObject>::default_namespaced_with(client, &ar);
        api
    }
}

/// ref to [GVKSpec::get_default_api]
pub async fn get_default_api() -> Api<DynamicObject> {
    return GVKSpec::get_default_api().await;
}

#[cfg(test)]
#[tokio::test]
async fn show_ar() {
    use envconfig::Envconfig;
    let gvk: GroupVersionKind = GVKSpec::init_from_env().unwrap().into();

    let ar = ApiResource::from_gvk(&gvk);
    insta::assert_debug_snapshot!(ar);
}

#[cfg(test)]
#[tokio::test]
async fn show_api() {
    let api = get_default_api().await;
    insta::assert_debug_snapshot!(api);
}
