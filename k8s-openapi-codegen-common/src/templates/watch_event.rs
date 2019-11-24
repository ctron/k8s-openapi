pub(crate) fn generate(
	mut writer: impl std::io::Write,
	type_name: &str,
	has_bookmark_event_type: bool,
	error_status_rust_type: &str,
	error_other_rust_type: &str,
) -> Result<(), crate::Error> {
	let (
		bookmark_def,
		bookmark_event_type,
		bookmark_type_match_arm,
		bookmark_type_value,
		bookmark_value_match_arm,
		bookmark_serialize,
	) =
		if has_bookmark_event_type {
			(
				"Bookmark(T),\n    ",
				"Bookmark,\n            ",
				"\"BOOKMARK\" => WatchEventType::Bookmark,\n                            ",
				"\"BOOKMARK\", ",
				{
					use std::fmt::Write;

					let mut result = String::new();
					writeln!(result, "                    WatchEventType::Bookmark => {{")?;
					writeln!(result, "                        let value_object = serde_value::ValueDeserializer::new(value_object);")?;
					writeln!(result, "                        {}::Bookmark(serde::Deserialize::deserialize(value_object)?)", type_name)?;
					writeln!(result, "                    }},")?;
					result
				},
				{
					use std::fmt::Write;

					let mut result = String::new();
					writeln!(result, "{}::Bookmark(object) => {{", type_name)?;
					writeln!(result, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "BOOKMARK")?;"#)?;
					writeln!(result, r#"                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;"#)?;
					writeln!(result, "            }},")?;
					write!(result, "            ")?;
					result
				},
			)
		}
		else {
			(
				"",
				"",
				"",
				"",
				String::new(),
				String::new(),
			)
		};

	writeln!(
		writer,
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/watch_event.rs")),
		type_name = type_name,
		bookmark_def = bookmark_def,
		error_status_rust_type = error_status_rust_type,
		error_other_rust_type = error_other_rust_type,
		bookmark_event_type = bookmark_event_type,
		bookmark_type_match_arm = bookmark_type_match_arm,
		bookmark_type_value = bookmark_type_value,
		bookmark_value_match_arm = bookmark_value_match_arm,
		bookmark_serialize = bookmark_serialize,
	)?;

	Ok(())
}
