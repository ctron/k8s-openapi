// Generated from definition io.k8s.api.core.v1.ConfigMapVolumeSource

/// Adapts a ConfigMap into a volume.
///
/// The contents of the target ConfigMap's Data field will be presented in a volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths. ConfigMap volumes support ownership management and SELinux relabeling.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ConfigMapVolumeSource {
    /// Optional: mode bits to use on created files by default. Must be a value between 0 and 0777. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(rename = "defaultMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,

    /// If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<::api::core::v1::KeyToPath>>,

    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Specify whether the ConfigMap or it's keys must be defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
