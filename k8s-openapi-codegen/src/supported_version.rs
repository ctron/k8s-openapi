pub(crate) const ALL: &[SupportedVersion] = &[
	SupportedVersion::V1_11,
	SupportedVersion::V1_12,
	SupportedVersion::V1_13,
	SupportedVersion::V1_14,
	SupportedVersion::V1_15,
	SupportedVersion::V1_16,
	SupportedVersion::V1_17,
	SupportedVersion::V1_18,
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum SupportedVersion {
	V1_11,
	V1_12,
	V1_13,
	V1_14,
	V1_15,
	V1_16,
	V1_17,
	V1_18,
}

impl SupportedVersion {
	pub(crate) fn mod_root(self) -> &'static str {
		match self {
			SupportedVersion::V1_11 => "v1_11",
			SupportedVersion::V1_12 => "v1_12",
			SupportedVersion::V1_13 => "v1_13",
			SupportedVersion::V1_14 => "v1_14",
			SupportedVersion::V1_15 => "v1_15",
			SupportedVersion::V1_16 => "v1_16",
			SupportedVersion::V1_17 => "v1_17",
			SupportedVersion::V1_18 => "v1_18",
		}
	}

	pub(crate) fn spec_url(self) -> &'static str {
		match self {
			SupportedVersion::V1_11 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.11.10/api/openapi-spec/swagger.json",
			SupportedVersion::V1_12 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.12.10/api/openapi-spec/swagger.json",
			SupportedVersion::V1_13 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.13.12/api/openapi-spec/swagger.json",
			SupportedVersion::V1_14 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.14.10/api/openapi-spec/swagger.json",
			SupportedVersion::V1_15 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.15.12/api/openapi-spec/swagger.json",
			SupportedVersion::V1_16 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.16.9/api/openapi-spec/swagger.json",
			SupportedVersion::V1_17 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.17.5/api/openapi-spec/swagger.json",
			SupportedVersion::V1_18 => "https://raw.githubusercontent.com/kubernetes/kubernetes/v1.18.2/api/openapi-spec/swagger.json",
		}
	}

	pub(crate) fn fixup(self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error> {
		#[allow(clippy::match_same_arms)]
		let upstream_bugs_fixups: &[fn(&mut crate::swagger20::Spec) -> Result<(), crate::Error>] = match self {
			SupportedVersion::V1_11 => &[
				crate::fixups::upstream_bugs::deployment_rollback_create_response_type,
				crate::fixups::upstream_bugs::optional_properties::crdstatus,
				crate::fixups::upstream_bugs::optional_properties::poddisruptionbudgetstatus,
				crate::fixups::upstream_bugs::raw_extension_ty,
				crate::fixups::upstream_bugs::remove_compat_refs,
			],

			SupportedVersion::V1_12 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::crdstatus,
				crate::fixups::upstream_bugs::raw_extension_ty,
				crate::fixups::upstream_bugs::remove_compat_refs,
			],

			SupportedVersion::V1_13 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::crdstatus,
				crate::fixups::upstream_bugs::raw_extension_ty,
				crate::fixups::upstream_bugs::remove_compat_refs,
			],

			SupportedVersion::V1_14 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::crdstatus,
				crate::fixups::upstream_bugs::raw_extension_ty,
			],

			SupportedVersion::V1_15 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
				crate::fixups::upstream_bugs::optional_properties::crdstatus,
				crate::fixups::upstream_bugs::raw_extension_ty,
			],

			SupportedVersion::V1_16 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
			],

			SupportedVersion::V1_17 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
			],

			SupportedVersion::V1_18 => &[
				crate::fixups::upstream_bugs::connect_options_gvk,
			],
		};

		let special_fixups: &[fn(&mut crate::swagger20::Spec) -> Result<(), crate::Error>] = &[
			crate::fixups::special::json_ty::json_schema_props_or_array,
			crate::fixups::special::json_ty::json_schema_props_or_bool,
			crate::fixups::special::json_ty::json_schema_props_or_string_array,
			crate::fixups::special::create_delete_optional,
			crate::fixups::special::create_optionals,
			crate::fixups::special::patch,
			crate::fixups::special::remove_delete_collection_operations_query_parameters,
			crate::fixups::special::remove_delete_operations_query_parameters,
			crate::fixups::special::separate_watch_from_list_operations,
			crate::fixups::special::watch_event,
			crate::fixups::special::list, // Must run after separate_watch_from_list_operations
			crate::fixups::special::response_types,
		];

		for fixup in upstream_bugs_fixups.iter().chain(special_fixups) {
			fixup(spec)?;
		}

		Ok(())
	}
}
