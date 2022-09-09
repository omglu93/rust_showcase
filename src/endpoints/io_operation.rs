use rocket::serde::Serialize;
use rocket::serde::json::Json;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;

static LOREM_IPSUM: &str =
"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

#[derive(Serialize)]
pub struct JsonIoOperationResult {
    data: String
}

#[get("/io-operation")]
pub fn get_io_operation() -> Json<JsonIoOperationResult> {

    let x = match read_and_write() {
        Err(_why) => panic!("Error while preforming IO operation"),
        Ok(file) => file,
    };
     
    //read_and_write().expect("Error while preforming IO operation");
    let result = JsonIoOperationResult{
        data: x
    };

    Json(result)
}

fn read_and_write() -> Result<String, &'static str> {

    let path = Path::new("../rust_rest/src/data/foo.txt");
    // Create File
    match File::create(&path) {
        Err(_why) => panic!("couldn't create file"),
        Ok(_file) => (),
    };

    let mut f = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path) {
                Err(_why) => panic!("couldn't adjust options on the file"),
                Ok(file) => file,
            };

    // Write File
    match f.write_all(LOREM_IPSUM.as_bytes()) {
        Err(_why) => panic!("couldn't write in file"),
        Ok(file) => file,
    };

    // Read File
    let buf_reader = BufReader::new(File::open(&path)
        .expect("Error handeling file"));

    let n = buf_reader.lines()
        .next()
        .unwrap()
        .expect("Error while reading line");


    // Delete File
    fs::remove_file(&path).expect("Error deleting file");
    
    Ok(n)
}