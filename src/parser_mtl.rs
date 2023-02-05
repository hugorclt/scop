
use crate::{checker::{check_obj_extensions, check_exists}, reader::get_buffers};
use std::{path::Path, io::BufReader};
use std::fs::File;
use std::io::{Lines};

#[derive(Debug, Clone)]
pub struct Material {
    name: String,
    specular_exponent: f32,
    ambiant_reflectivity: [f32; 3],
    diffuse_reflectivity: [f32; 3],
    specular_reflectivity: [f32; 3],
    optical_density: f32,
    factor: f32,
    illumination: f32,
}

impl Material{
    pub fn default() -> Self {
        Material {
            name: "".to_string(),
            ambiant_reflectivity: [-1.0, -1.0, -1.0],
            diffuse_reflectivity: [-1.0, -1.0, -1.0],
            specular_reflectivity: [-1.0, -1.0, -1.0],
            illumination: -1.0,
            factor: -1.0,
            specular_exponent: -1.0,
            optical_density: -1.0,
        }
    }
}


pub fn open_material(line: &str, obj_path: &str) -> Lines<BufReader<File>>
{
    let vec_line: Vec<&str> = line.split_whitespace().collect();

    if vec_line.len() != 2 {
        panic!("Error on the line {line}");
    }

    check_obj_extensions(vec_line[1], "mtl");
    let path_mtl = Path::new(obj_path).parent();
    let full_path;
    if path_mtl.is_some() && path_mtl.unwrap() == Path::new("") {
        full_path = vec_line[1].to_string();
    }
    else if path_mtl.is_some() {
        let temp = path_mtl.unwrap()
        .to_str()
        .expect("Expected to convert a path to a string but failed");
        let mut string_temp = temp.to_string();
        string_temp.push_str("/");
        string_temp.push_str(vec_line[1]);
        full_path = string_temp;
    }
    else {
        full_path = vec_line[1].to_string();
    }
    check_exists(&full_path);

    get_buffers::<&str>(&full_path)
}

pub fn parse_reflectivity(line: String) -> [f32; 3]
{
    let vec_line : Vec<&str> = line.split_whitespace().collect();

    if vec_line.len() != 4 {
        panic!("error at line: {line}");
    }

    [
        vec_line[1].parse().expect("Expected to convert str to float"),
        vec_line[2].parse().expect("Expected to convert str to float"),
        vec_line[3].parse().expect("Expected to convert str to float"),
    ]
}

pub fn mtl_parse_name(line: String) -> String {
    let vec_line : Vec<&str> = line.split_whitespace().collect();

    if vec_line.len() != 2 {
        panic!("error at line: {line}");
    }

    vec_line[1].to_string()
}

pub fn mtl_parse_float(line: String) -> f32 {
    let vec_line : Vec<&str> = line.split_whitespace().collect();

    if vec_line.len() != 2 {
        panic!("error at line: {line}");
    }

    vec_line[1].parse().expect("Expected to convert str to float but failed")
}

pub fn parse_material(line: &str, vec_material: &mut Vec<Material>, obj_path: &str)
{
    let buffers = open_material(line, obj_path);
    let mut material: Material = Material::default();

    for line in buffers {
        match line {
            Ok(line) => {
                match line.as_str() {
                    line if line.len() == 0 => {},
                    line if line.starts_with("newmtl") => material.name = mtl_parse_name(line.to_string()),
                    line if line.starts_with("illum") => material.illumination = mtl_parse_float(line.to_string()),
                    line if line.starts_with("Ns") => material.specular_exponent = mtl_parse_float(line.to_string()),
                    line if line.starts_with("Ka")  => {material.ambiant_reflectivity = parse_reflectivity(line.to_string())}
                    line if line.starts_with("Kd")  => {material.diffuse_reflectivity = parse_reflectivity(line.to_string())}
                    line if line.starts_with("Ks")  => {material.specular_reflectivity = parse_reflectivity(line.to_string())}
                    line if line.starts_with("Ni") => material.optical_density = mtl_parse_float(line.to_string()),
                    line if line.starts_with('d') => material.factor = mtl_parse_float(line.to_string()),
                    line if line.starts_with('#') => {},
                    _ => panic!("Error in the .mtl file at line '{line}'"),
                }
            }
            Err(e) => panic!("Error while reading mtl file {e}")
        }
    }
    vec_material.push(material);
}

pub fn find_material(line: String, vec_material: &Vec<Material>,  current_material: &mut Vec<Material>) {
    let vec_line : Vec<&str> = line.split_whitespace().collect();

    if vec_line.len() != 2 {
        panic!("error at line: {line}");
    }

    for i in 0..vec_material.len() {
        if vec_material[i].name == vec_line[1] {
            current_material.push(vec_material[i].clone());
            return ;
        }
    }
    panic!("Error material not found at line {line}");

}
