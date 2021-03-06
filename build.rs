use std::collections::HashSet;

pub fn main() {
    // collect names
    let mut names = HashSet::new();
    let re = regex::Regex::new(r"name!\(([a-zA-Z0-9_]*)\)").unwrap();

    for entry in walkdir::WalkDir::new("src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {

        let text = std::fs::read_to_string(entry.path()).unwrap();
        for caps in re.captures_iter(&text) {
            names.insert(caps.get(1).map(|m| m.as_str()).unwrap().to_string());
        }
    }

    // parse existig names from names.rs
    let existing_text = std::fs::read_to_string("src/names.rs").unwrap();
    let re = regex::Regex::new(r"([a-zA-Z0-9_]+)(,|[[:space:]]+})").unwrap();
    let mut existing_names = HashSet::new();
    for caps in re.captures_iter(&existing_text[165..]) {
        existing_names.insert(caps.get(1).map(|m| m.as_str()).unwrap().to_string());
    }

    // check for new names
    let mut need_regenerate_list = false;
    for name in names.iter() {
        if !existing_names.contains(name) {
            need_regenerate_list = true;
            break;
        }
    }

    if need_regenerate_list {
        // update names.rs
        let mut namestr = String::new();
        for name in names {
            namestr += &name;
            namestr += ",\n    ";
        }

        let namestr = format!("// don't modify this file by hand, it is automatically generated by build.rs, for make_names.\n\n#[macro_export] macro_rules! name {{ ($i:ident) => {{crate::names::$i}} }}\n\nmake_names::make_names! {{\n    {}\n}}", namestr);
        std::fs::write("src/names.rs", namestr).unwrap();
    }
}