GVKSpec is spec of [GroupVersionKind]

You can use it with [Client] to get [Api<DynamicObject>]
[Clone] due to [BuilderApi::get_gvk]:
`let gvk: GroupVersionKind = self.0.clone().into();`