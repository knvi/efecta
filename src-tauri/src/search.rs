use std::path::Path;

use rust_search::SearchBuilder;
use strsim::jaro_winkler;

fn file_name_from_path(path: &str) -> String {
    let path = Path::new(path);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    return file_name.to_string();
}

pub fn sort(vector: &mut Vec<String>, input: &str) {
    vector.sort_by(|a, b| {
        let input = input.to_lowercase();
        let a = file_name_from_path(a).to_lowercase();
        let b = file_name_from_path(b).to_lowercase();
        let a = jaro_winkler(a.as_str(), input.as_str());
        let b = jaro_winkler(b.as_str(), input.as_str());
        b.partial_cmp(&a).unwrap()
    });
}

pub fn search(input: &str, search_locations: Vec<&str>, extension: Option<&str>, depth: Option<usize>) -> Vec<String> {
    let (loc, more_loc) = search_locations.split_first().unwrap();
    let mut res: Vec<String> = SearchBuilder::default()
        .search_input(input)
        .location(loc)
        .more_locations(more_loc.to_vec())
        .ext(extension.unwrap_or("*"))
        .depth(depth.unwrap_or(1))
        .limit(5)
        .ignore_case()
        .build()
        .collect();

    sort(&mut res, input);
    res
}