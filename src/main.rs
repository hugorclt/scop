mod reader;
mod checker;
use std::env;
use crate::reader::Reader;
use crate::checker::Checker;

// pub enum Line_Types {
//     Vertex,
//     Textures,
//     Normals,
//     Links,
//     Name,
//     Comments,
// }

// struct Data {
//     name: String,
//     vertices: Vec<[f64; 3]>,
//     textures: Vec<[f64; 2]>,
//     normals: Vec<[f64; 3]>,
//     links: Vec<[[u32; 3]; 3]>,
// }

fn line_type_choice(line: &String) {
    match line.as_str() {
        line if line.starts_with("#") => {println!("#")},
        line if line.starts_with("o") => {println!("o")},
        line if line.starts_with("v") => {println!("v")},
        line if line.starts_with("f") => {println!("f")},
        line if line.starts_with("vt") => {println!("vt")},
        line if line.starts_with("vn") => {println!("vn")},
        line if line.starts_with("vf") => {println!("vf")},
        line if line.starts_with("mtllib")  => {println!("mttlib")},
        line if line.starts_with("usemtl")  => {println!("usemtl")},
        line if line.starts_with("s")  => {println!("s")},
        _ => panic!("Error in the .obj file at line '{}'", line),
    };
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    Checker::check_args(&args);
    Checker::check_exists(&args[1]);
    Checker::check_obj_extensions(&args[1]);

    let mut buffer = Reader::get_buffers::<&str>(&args[1]);

    for line in buffer {
        match line {
            Ok(line) => {
                line_type_choice(&line);
            }
            Err(e) => panic!("Error while retrieving the line {}", e)
        }
    }


}