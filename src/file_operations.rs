use crate::app::Document;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{Read, Write};

pub fn get_md_files<'a>(path: String) -> HashMap<String, Document> {
    if let Ok(entries) = read_dir(path) {
        let mut sorted_entries: Vec<_> = entries.map(|r| r.unwrap()).collect();
        sorted_entries.sort_by_key(|dir| dir.path());

        let mut hm = HashMap::new();
        for entry in sorted_entries {
            // Basic metadata about file
            let full_path = String::from(entry.path().to_string_lossy());
            let mut file_title = String::from(entry.file_name().to_string_lossy());
            if !file_title.ends_with(".md") {
                continue;
            }

            let mut file =
                File::open(full_path.clone()).expect("Existant file suddenly non-existant!");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Could not read file!");
            if let Some(title) = get_md_file_title(&contents) {
                file_title = title;
            }

            hm.insert(
                file_title.clone(),
                Document {
                    path: full_path,
                    title: file_title,
                },
            );
        }
        hm
    } else {
        println!("Can't open directory!");
        HashMap::new()
    }
}

pub fn get_md_file_title(contents: &String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^\# (.+)$|^title: "?([^"]+)"?$"#).unwrap();
    }
    for line in contents.split("\n") {
        if let Some(caps) = RE.captures(&line) {
            return caps
                .get(1)
                .or(caps.get(2))
                .map(|x| String::from(x.as_str()));
        }
    }
    None
}

pub fn get_file_contents(path: &String) -> Option<String> {
    let mut file = File::open(path).expect("Existant file suddenly non-existant!");
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(e) => panic!("{:?}", e),
    }
}

pub fn save_file_contents(path: &String, buf: String) {
    println!("Saving to file: {:?}", path);
    let mut file = File::create(path).expect("Existant file suddenly non-existant!");
    file.write_all(buf.as_bytes())
        .expect("Cannot write to file (read-only?)");
}
