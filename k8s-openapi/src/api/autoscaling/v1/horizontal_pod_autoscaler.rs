// Generated from definition io.k8s.api.autoscaling.v1.HorizontalPodAutoscaler

/// configuration of a horizontal pod autoscaler.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HorizontalPodAutoscaler {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Standard object metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// behaviour of autoscaler. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<::api::autoscaling::v1::HorizontalPodAutoscalerSpec>,

    /// current information about the autoscaler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<::api::autoscaling::v1::HorizontalPodAutoscalerStatus>,
}
