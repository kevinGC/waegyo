use std::collections::TreeMap;
use std::io::File;
use std::str::*;
use serialize::json;


enum Terrain {
	Land,
	Sea
}

pub struct Loc {
	adjacent    : TreeMap<String, &Loc>,
	terrain     : Terrain,
	// piece      : Option<Piece>, TODO
	display_name: String
}

impl Loc {
	// TODO this is enefficient, but probably doesn't matter
	// pub fn from_file(&json_obj: &json::Json) -> TreeMap<String, Box<Loc>> {
	pub fn from_file(filename: &str) -> TreeMap<String, Box<Loc>> {
		let file_contents = File::open(&Path::new(filename)).read_to_end();
		let contents      = from_utf8_owned(file_contents.unwrap()).unwrap();
		let json_obj      = json::from_str(contents.as_slice()).unwrap();
		let parent_obj    = get_json_Object_or_fail(&json_obj);
		let mut locs: TreeMap<String, Box<Loc>> = TreeMap::new();

		// insert blank locations
		for (key, sub_json_obj) in parent_obj.iter() {
			locs.insert(key.clone(), box Loc::new());
		}

		// configure each location
		for (key, sub_json_obj) in parent_obj.iter() {
			let obj = get_json_Object_or_fail(sub_json_obj);
			let mut loc = locs.find(key).unwrap();

			// configure properties
			let terrain_string = get_json_String_or_fail(obj, "terrain");
			loc.terrain =
				if terrain_string == String::from_str("Land") {
					Land
				} else {
					Sea
				};
			loc.display_name = get_json_String_or_fail(obj, "display_name");

			// TODO this is horrid
			// add adjacent locations
			let adjacents = match obj.find(&String::from_str("adjacent")).unwrap() {
				&json::List(list) => list.iter().map(|json_obj| match json_obj {
					&json::String(ref val) => val.clone(),
					_                      => fail!("not a string")
				}),
				_                 => fail!("not an array")
			};
			for adjacent in adjacents {
				let (_, adjacent_loc) = locs.find(adjacent);
				loc.adjacent.insert(adjacent, adjacent);
			}
		}
		locs
	}

	fn new() -> Loc {
		Loc {
			adjacent    : TreeMap::new(),
			terrain     : Land,
			display_name: String::new() // TODO inefficient
		}
	}

}

	fn get_json_Object_or_fail(json_obj: &json::Json) -> &json::Object {
		match json_obj {
			&json::Object(ref obj) => obj,
			_                      => fail!("not an object")
		};
	}

	fn get_json_String_or_fail(object: &json::Object, key: &str) -> String {
		match object.find(&String::from_str(key)).unwrap() {
			&json::String(ref val) => val.clone(),
			_                      => fail!("not a string")
		}
	}
