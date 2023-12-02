use serde_json::json;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn get_filesize(path: &str) -> u64 {
    match fs::metadata(path) {
        Ok(metadata) => metadata.len(),
        Err(_) => 0,
    }
}

// pub fn append_to_json_file(dest_path: &str, new_data: &Vec<Entry>) {
//     let path = Path::new(dest_path);
//     fs::create_dir_all(path.parent().unwrap()).unwrap();

//     let mut data = match fs::read_to_string(path) {
//         Ok(file) => {
//             let mut json: Vec<RequestIdResponse> = serde_json::from_str(&file).unwrap();
//             json.extend(new_data.clone());
//             json
//         },
//         Err(_) => new_data.clone(),
//     };

//     let file = fs::File::create(path).unwrap();
//     let mut file = std::io::BufWriter::new(file);
//     write!(file, "{}", json!(data)).unwrap();
// }
