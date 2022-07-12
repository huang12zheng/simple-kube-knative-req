use kube::api::GroupVersionKind;

pub struct GKVSpec {
    pub group: String,
    pub version: String,
    pub kind: String,
}

impl GKVSpec {
    #[allow(dead_code)]
    pub fn from_env() -> Self {
        let group = std::env::var("GROUP").unwrap_or_else(|_| "serving.knative.dev".into());
        let version = std::env::var("VERSION").unwrap_or_else(|_| "v1".into());
        let kind = std::env::var("KIND").unwrap_or_else(|_| "Service".into());
        Self {
            group,
            version,
            kind,
        }
    }
}

impl Into<GroupVersionKind> for GKVSpec {
    fn into(self) -> GroupVersionKind {
        let gkv = GroupVersionKind::gvk(&self.group, &self.version, &self.kind);
        gkv
    }
}
