use crate::app::Document;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader, Error, Read};

pub fn get_md_files(path: String) -> HashMap<String, Document> {
    if let Ok(paths) = read_dir(path) {
        let mut hm = HashMap::new();
        for path in paths {
            let p = path.unwrap();

            // Basic metadata about file
            let full_path = String::from(p.path().to_string_lossy());
            let mut file_title = String::from(p.file_name().to_string_lossy());

            // We're going to have to read the file anyway to get the title, might as well just store the contents now too.
            let mut file =
                File::open(full_path.clone()).expect("Existant file suddenly non-existant!");
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            if let Some(title) = get_md_file_title(&contents) {
                file_title = title;
            }

            hm.insert(
                full_path.clone(),
                Document {
                    path: full_path,
                    title: file_title,
                    contents: Some(contents),
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
