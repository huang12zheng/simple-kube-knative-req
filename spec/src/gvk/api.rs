use crate::*;

/// A trait to get [Api<DynamicObject>] by [BuilderApi::builder_api]
#[async_trait]
trait BuilderApi {
    /// ```snap, no_run
    #[doc = include_str!("snapshots/kube_do_spec__gvk__api__show_api.snap")]
    /// ```
    ///
    /// [] with [BuilderApi::get_ar] and [BuilderApi::get_client] into [Client] and [ApiResource]
    ///
    /// [Client] and [ApiResource] with [Api::<DynamicObject>::default_namespaced_with] into [Api<DynamicObject>]
    ///
    /// `let api = Api::<DynamicObject>::default_namespaced_with(client, &ar);`
    async fn builder_api(&self) -> Api<DynamicObject> {
        let client = self.get_client().await.clone();
        let ar = self.get_ar();
        let api = Api::<DynamicObject>::default_namespaced_with(client, &ar);
        api
    }
    /// ```snap
    #[doc = include_str!("snapshots/kube_do_spec__gvk__api__show_ar.snap")]
    /// ```
    /// [] with [BuilderApi::get_gvk] into [GroupVersionKind]
    /// [GroupVersionKind] with [ApiResource::from_gvk] into [ApiResource]
    fn get_ar(&self) -> ApiResource {
        return ApiResource::from_gvk(&self.get_gvk());
    }

    /// [] with [get_client] into [Client]
    async fn get_client(&self) -> &'async_trait Client {
        return get_client().await;
    }
    // ! not use ` [] with [] into [GroupVersionKind]` due to it isn't in implementation
    /// ```snap
    #[doc = include_str!("snapshots/kube_do_spec__gvk__gvk_spec__tests__gvk_spec.snap")]
    /// ```
    /// A required method to get GroupVersionKind
    ///
    /// Example:
    /// ```
    /// #[tokio::test]
    /// async fn show_ar() {
    ///     use envconfig::Envconfig;
    ///     let gvk: GroupVersionKind = GVKSpec::init_from_env().unwrap().get_gvk();
    ///
    ///     let ar = ApiResource::from_gvk(&gvk);
    ///     insta::assert_debug_snapshot!(ar);
    /// }
    /// ```
    fn get_gvk(&self) -> GroupVersionKind;
}

/// A trait to get [Api<DynamicObject>]
#[async_trait]
pub trait GetApi {
    /// A required method to get [Api<DynamicObject>].
    // ! you can click `get_api` to look implementation for more info.
    ///
    /// ```snap
    #[doc = include_str!("snapshots/kube_do_spec__gvk__api__show_api.snap")]
    /// ```
    /// Example:
    /// ```
    /// #[tokio::test]
    /// async fn show_api() {
    ///     let spec = KnativeSpec::default();
    ///     let api = spec.get_api().await;
    ///     insta::assert_debug_snapshot!(api);
    /// }
    /// ```
    async fn get_api(&self) -> Api<DynamicObject>;
}

/// [GVKSpec] with [BuilderApi] into [Api<DynamicObject>]
/// - `self.builder_api().await`
#[async_trait]
impl GetApi for GVKSpec {
    async fn get_api(&self) -> Api<DynamicObject> {
        self.builder_api().await
    }
}
/// [GVKSpec] with [From<GVKSpec>] into [GroupVersionKind]
/// - `let gvk: GroupVersionKind = self.clone().into();`
impl BuilderApi for GVKSpec {
    fn get_gvk(&self) -> GroupVersionKind {
        let gvk: GroupVersionKind = self.clone().into();
        gvk
    }
}

impl GVKSpec {
    /// Associated function [get_default_api] use data from [Envconfig]
    /// From this point of view, it's different with [GVKSpec::get_api]
    async fn get_default_api() -> Api<DynamicObject> {
        let client: Client = get_client().await.clone();
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
async fn show_api() {
    let api = get_default_api().await;
    insta::assert_debug_snapshot!(api);
}
