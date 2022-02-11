use std::fs;
use std::collections::HashMap;
use crate::app::Document;
pub fn get_md_files(path: String) -> HashMap<String, Document> {
    if let Ok(paths) = fs::read_dir(path) {
        let mut hm = HashMap::new();
        for path in paths {
            let p = path.unwrap();
            let full_path = String::from(p.path().to_string_lossy());
            let file_name = String::from(p.file_name().to_string_lossy());
            hm.insert(full_path.clone(), Document {
                path: full_path,
                title: file_name,
                contents: None
            });
        }
        hm
    } else {
        println!("Can't open directory!");
        HashMap::new()
    }
}
