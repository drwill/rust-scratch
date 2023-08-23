use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Serialize, Deserialize, JsonSchema, Default, PartialEq)]
#[kube(group = "stable.example.com", version = "v1", kind = "Widget")]
#[kube(shortname = "wt", namespaced)]
pub struct WidgetSpec {
    pub age: Option<i32>,
}