// TODO figure out how to make this work
pub fn cmp_str_string(str_: &str) -> |String| -> Ordering {
	let local = str_.clone();
	|string: String| {
		local.cmp(&string.as_slice())
	}
}
