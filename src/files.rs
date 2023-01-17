use std::{collections::HashMap, path::Path, fs};

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RouteFile {
    pub url: String,
    pub file: String
}

impl Default for RouteFile {
    fn default() -> RouteFile {
        RouteFile { url: "".to_string(), file: "".to_string() }
    }    
}

pub struct RouteFiles {
    pub files: HashMap<String, String>
}

impl RouteFiles {
    pub fn load(file: String, folder: String, map: &mut HashMap<String, String>) {
        let file_str: String = std::fs::read_to_string(Path::new(&folder).join(file)).unwrap();

        let file_settings: RouteFile = serde_json::from_str::<RouteFile>(&file_str).unwrap_or_default();

        if file_settings.url != "" && file_settings.file != "" {
            map.insert(file_settings.url, file_settings.file);
        }
    }

    pub fn load_all(&mut self, folder: String) {
        let mut new_files: HashMap<String, String> = HashMap::new();

        let files = fs::read_dir(folder.clone()).unwrap();

        for file in files {
            RouteFiles::load(
                String::from(file.unwrap().file_name().to_str().unwrap()),
                folder.clone(),
                &mut new_files
            );
        }
        
        self.files = new_files;
    }
}

impl Default for RouteFiles {
    fn default() -> RouteFiles {
        RouteFiles{
            files: HashMap::new()
        }
    }
}