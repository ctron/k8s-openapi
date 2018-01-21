// Generated from definition io.k8s.api.extensions.v1beta1.ReplicaSetCondition

/// ReplicaSetCondition describes the state of a replica set at a certain point.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ReplicaSetCondition {
    /// The last time the condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,

    /// A human readable message indicating details about the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// The reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Status of the condition, one of True, False, Unknown.
    pub status: String,

    /// Type of replica set condition.
    #[serde(rename = "type")]
    pub type_: String,
}
