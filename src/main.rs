use std::env;
use std::path::Path;
use std::fmt;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

#[derive(Debug, Clone)]
pub enum GetOBJFileError {
    FileNotFound,
    FileNotSpecified,
    FileInvalid,
}

impl std::fmt::Display for GetOBJFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GetOBJFileError::FileNotFound => write!(f, "Couldn't find this OBJ file"),
            GetOBJFileError::FileNotSpecified => write!(f, "Please specify an OBJ file"),
            GetOBJFileError::FileInvalid => write!(f, "This tool only supports OBJ files, sorry"),
        }
    }
}

fn get_obj_filepath() -> Result<String, GetOBJFileError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(GetOBJFileError::FileNotSpecified)
    }

    let obj_filepath = &args[1];

    if !obj_filepath.ends_with(".obj") {
        return Err(GetOBJFileError::FileInvalid)
    }

    if !Path::new(obj_filepath).exists() {
        return Err(GetOBJFileError::FileNotFound)
    }

    return Ok(obj_filepath.to_string())
}

fn process_obj(obj_filepath: String) -> Result<(), Error> {
     let obj_file = File::open(obj_filepath)?;
     let mut obj_reader = BufReader::new(obj_file);

     let mut faces: Vec<Polygon> = vec![];

     for line in obj_reader.lines() {
         let line_string = line.unwrap();

         if line_string.starts_with("f") {
             let polygon = Polygon::from_string(line_string)
             faces.push(polygon)
         }
     }

     println!("{:?}", faces);

     Ok(())
}

fn doubleside_obj_filepath(obj_filepath: String) {
    match process_obj(obj_filepath) {
        Ok(()) => println!("Double sided OBJ"),
        Err(e) => println!("Error occurred: {}", e)
    }
}

fn main() {
    match get_obj_filepath() {
        Ok(obj_filepath) => doubleside_obj_filepath(obj_filepath),
        Err(e) => println!("{}", e)
    }
}
