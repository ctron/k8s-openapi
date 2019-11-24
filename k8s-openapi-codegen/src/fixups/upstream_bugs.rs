#![deny(unused)]

//! These fixups correspond to bugs in the upstream swagger spec.

// Path operation annotated with a "x-kubernetes-group-version-kind" that references a type that doesn't exist in the schema.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/66807
#[allow(clippy::if_same_then_else)]
pub(crate) fn connect_options_gvk(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let mut found = false;

	for operation in &mut spec.operations {
		if let Some(kubernetes_group_kind_version) = &mut operation.kubernetes_group_kind_version {
			if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "NodeProxyOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Node".to_string();
				found = true;
			}
			else if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "PodAttachOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Pod".to_string();
				found = true;
			}
			else if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "PodExecOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Pod".to_string();
				found = true;
			}
			else if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "PodPortForwardOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Pod".to_string();
				found = true;
			}
			else if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "PodProxyOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Pod".to_string();
				found = true;
			}
			else if kubernetes_group_kind_version.group == "" && kubernetes_group_kind_version.kind == "ServiceProxyOptions" && kubernetes_group_kind_version.version == "v1" {
				kubernetes_group_kind_version.kind = "Service".to_string();
				found = true;
			}
		}
	}

	if found {
		Ok(())
	}
	else {
		Err("never applied connect options kubernetes_group_kind_version override".into())
	}
}

// The spec says that `createAppsV1beta1NamespacedDeploymentRollback` returns `DeploymentRollback`, but it returns `Status`.
//
// Ref: https://github.com/kubernetes/kubernetes/pull/63837
pub(crate) fn deployment_rollback_create_response_type(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let mut found = false;

	if let Some(operation) = spec.operations.iter_mut().find(|o| o.id == "createAppsV1beta1NamespacedDeploymentRollback") {
		for response in operation.responses.values_mut() {
			if let crate::swagger20::Schema { kind: crate::swagger20::SchemaKind::Ref(crate::swagger20::RefPath { path, .. }), .. } = response {
				if path == "io.k8s.api.apps.v1beta1.DeploymentRollback" {
					std::mem::replace(path, "io.k8s.apimachinery.pkg.apis.meta.v1.Status".to_owned());
					found = true;
				}
			}
		}
	}

	if found {
		Ok(())
	}
	else {
		Err("never applied createAppsV1beta1NamespacedDeploymentRollback response type override".into())
	}
}

// Type is not annotated with "x-kubernetes-group-version-kind", which would make its associated functions end up in the mod root.
//
// Ref: https://github.com/kubernetes/kubernetes/issues/49465
// Ref: https://github.com/kubernetes/kubernetes/pull/64174
pub(crate) mod gvk {
	pub(crate) fn api_service_list_v1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.kube-aggregator.pkg.apis.apiregistration.v1.APIServiceList".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiregistration.k8s.io".to_string(),
					kind: "APIServiceList".to_string(),
					version: "v1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied APIServiceList v1 kubernetes_group_kind_version override".into())
	}

	pub(crate) fn api_service_list_v1beta1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIServiceList".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiregistration.k8s.io".to_string(),
					kind: "APIServiceList".to_string(),
					version: "v1beta1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied APIServiceList v1beta1 kubernetes_group_kind_version override".into())
	}

	pub(crate) fn api_service_v1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.kube-aggregator.pkg.apis.apiregistration.v1.APIService".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiregistration.k8s.io".to_string(),
					kind: "APIService".to_string(),
					version: "v1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied APIService v1 kubernetes_group_kind_version override".into())
	}

	pub(crate) fn api_service_v1beta1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIService".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiregistration.k8s.io".to_string(),
					kind: "APIService".to_string(),
					version: "v1beta1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied APIService v1beta1 kubernetes_group_kind_version override".into())
	}

	pub(crate) fn crd_v1beta1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinition".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiextensions.k8s.io".to_string(),
					kind: "CustomResourceDefinition".to_string(),
					version: "v1beta1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied CustomResourceDefinition kubernetes_group_kind_version override".into())
	}

	pub(crate) fn crd_list_v1beta1(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionList".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if definition.kubernetes_group_kind_versions.is_none() {
				definition.kubernetes_group_kind_versions = Some(vec![crate::swagger20::KubernetesGroupKindVersion {
					group: "apiextensions.k8s.io".to_string(),
					kind: "CustomResourceDefinitionList".to_string(),
					version: "v1beta1".to_string(),
				}]);
				return Ok(());
			}
		}

		Err("never applied CustomResourceDefinitionList v1beta1 kubernetes_group_kind_version override".into())
	}
}

// Fixes to the JSON types used in CRD validation
//
// Ref: https://github.com/kubernetes/kubernetes/pull/65256
pub(crate) mod json_ty {
	// The spec says that `JSON` is an object with a property `Raw` that's a byte-formatted string.
	// While the golang type is indeed a struct with a `Raw []byte` field, the type is serialized by just emitting the value of that field.
	// The value of that field is itself a JSON-serialized value.
	//
	// Thus `JSON` is really an arbitrary JSON value, and should be represented by `serde_json::Value`
	pub(crate) fn json(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let mut found = false;

		for &definition_path in &[
			"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSON",
			"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSON",
		] {
			let definition_path = crate::swagger20::DefinitionPath(definition_path.to_owned());
			if let Some(definition) = spec.definitions.get_mut(&definition_path) {
				if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Any) = definition.kind {
				}
				else {
					definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Any);
					found = true;
				}
			}
		}

		if found {
			Ok(())
		}
		else {
			Err("never applied JSON override".into())
		}
	}

	// The spec says that `JSONSchemaPropsOrArray` is an object with properties `JSONSchemas` and `Schema`.
	// In fact this type is either a `JSONSchemaProps` or an array of `JSONSchemaProps`.
	pub(crate) fn json_schema_props_or_array(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let mut found = false;

		for &definition_path in &[
			"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrArray",
			"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrArray",
		] {
			let definition_path = crate::swagger20::DefinitionPath(definition_path.to_owned());
			if let Some(definition) = spec.definitions.get_mut(&definition_path) {
				if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrArray) = definition.kind {
				}
				else {
					definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrArray);
					found = true;
				}
			}
		}

		if found {
			Ok(())
		}
		else {
			Err("never applied JSONSchemaPropsOrArray override".into())
		}
	}

	// The spec says that `JSONSchemaPropsOrBool` is an object with properties `Allows` and `Schema`.
	// In fact this type is either a `JSONSchemaProps` or a `bool`.
	pub(crate) fn json_schema_props_or_bool(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let mut found = false;

		for &definition_path in &[
			"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrBool",
			"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrBool",
		] {
			let definition_path = crate::swagger20::DefinitionPath(definition_path.to_owned());
			if let Some(definition) = spec.definitions.get_mut(&definition_path) {
				if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrBool) = definition.kind {
				}
				else {
					definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrBool);
					found = true;
				}
			}
		}

		if found {
			Ok(())
		}
		else {
			Err("never applied JSONSchemaPropsOrBool override".into())
		}
	}

	// The spec says that `JSONSchemaPropsOrStringArray` is an object with properties `Property` and `Schema`.
	// In fact this type is either a `JSONSchemaProps` or a `Vec<String>`.
	pub(crate) fn json_schema_props_or_string_array(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let mut found = false;

		for &definition_path in &[
			"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaPropsOrStringArray",
			"io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.JSONSchemaPropsOrStringArray",
		] {
			let definition_path = crate::swagger20::DefinitionPath(definition_path.to_owned());
			if let Some(definition) = spec.definitions.get_mut(&definition_path) {
				if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrStringArray) = definition.kind {
				}
				else {
					definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::JSONSchemaPropsOrStringArray);
					found = true;
				}
			}
		}

		if found {
			Ok(())
		}
		else {
			Err("never applied JSONSchemaPropsOrStringArray override".into())
		}
	}
}

// The spec says that this property is an array, but it can be null.
//
// Override it to be optional to achieve the same effect.
pub(crate) mod optional_properties {
	// `APIGroup::serverAddressByClientCIDRs`
	//
	// Ref: https://github.com/kubernetes/kubernetes/pull/61963
	pub(crate) fn apigroup(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apimachinery.pkg.apis.meta.v1.APIGroup".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
				if let Some(property) = properties.get_mut(&crate::swagger20::PropertyName("serverAddressByClientCIDRs".to_string())) {
					if property.1 {
						property.1 = false;
						return Ok(());
					}
				}
			}
		}

		Err("never applied APIGroups optional properties override".into())
	}

	// `CustomResourceDefinitionStatus::conditions`
	//
	// Ref: https://github.com/kubernetes/kubernetes/pull/64996
	pub(crate) fn crdstatus(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.CustomResourceDefinitionStatus".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
				if let Some(property) = properties.get_mut(&crate::swagger20::PropertyName("conditions".to_string())) {
					if property.1 {
						property.1 = false;
						return Ok(());
					}
				}
			}
		}

		Err("never applied CustomResourceDefinitionStatus optional properties override".into())
	}

	// `PodDisruptionBudgetStatus::disruptedPods`
	//
	// Ref: https://github.com/kubernetes/kubernetes/pull/65041
	pub(crate) fn poddisruptionbudgetstatus(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		let definition_path = crate::swagger20::DefinitionPath("io.k8s.api.policy.v1beta1.PodDisruptionBudgetStatus".to_owned());
		if let Some(definition) = spec.definitions.get_mut(&definition_path) {
			if let crate::swagger20::SchemaKind::Properties(properties) = &mut definition.kind {
				if let Some(property) = properties.get_mut(&crate::swagger20::PropertyName("disruptedPods".to_string())) {
					if property.1 {
						property.1 = false;
						return Ok(());
					}
				}
			}
		}

		Err("never applied PodDisruptionBudgetStatus optional properties override".into())
	}
}

// The spec says that `RawExtension` is an object with a property `raw` that's a byte-formatted string.
// While the golang type is indeed a struct with a `Raw []byte` field, the type is serialized by just emitting the value of that field.
// The value of that field is itself a JSON-serialized value. For example, a `WatchEvent` of `Pod`s has the `Pod` object serialized as
// the value of the `WatchEvent::object` property.
//
// Thus `RawExtension` is really an arbitrary JSON value, and should be represented by `serde_json::Value`
//
// Ref: https://github.com/kubernetes/kubernetes/issues/55890
//
// https://github.com/kubernetes/kubernetes/pull/56434 will remove RawExtension and replace it with `{ type: "object" }`,
// which would've already been mapped to `Ty(Any)` by `Ty::parse`, so just replicate that for `RawExtension` here.
pub(crate) fn raw_extension_ty(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	let definition_path = crate::swagger20::DefinitionPath("io.k8s.apimachinery.pkg.runtime.RawExtension".to_owned());
	if let Some(definition) = spec.definitions.get_mut(&definition_path) {
		if let crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Any) = definition.kind {
		}
		else {
			definition.kind = crate::swagger20::SchemaKind::Ty(crate::swagger20::Type::Any);
			return Ok(());
		}
	}

	Err("never applied RawExtension override".into())
}

// Remove `$ref`s under `io.k8s.kubernetes.pkg` since these are marked deprecated and point to corresponding definitions under `io.k8s.api`.
// They only exist for backward-compatibility with 1.7's spec.
pub(crate) fn remove_compat_refs(spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
	const COMPAT_NAMESPACE: &[&str] = &["io", "k8s", "kubernetes", "pkg"];

	let mut to_remove = vec![];

	for (definition_path, definition) in &spec.definitions {
		if let crate::swagger20::SchemaKind::Ref(_) = definition.kind {
			let parts: Vec<_> = definition_path.split('.').collect();
			if parts.starts_with(COMPAT_NAMESPACE) {
				to_remove.push(definition_path.clone());
			}
		}
	}

	if to_remove.is_empty() {
		return Err("never removed compat refs".into());
	}

	for to_remove in to_remove {
		spec.definitions.remove(&to_remove);
	}

	Ok(())
}
