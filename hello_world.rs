mod reader;
mod checker;
mod parser;
mod parser_mtl;

use std::env;
use parser_mtl::Material;

use crate::reader::*;
use crate::checker::*;
use crate::parser::Data;

fn start_parsing(args: Vec<String>, data: &mut Data) {
    let mut material_list: Vec<Material> = Vec::new();
    let mut current_material: Vec<Material> = Vec::new();
    let buffer = get_buffers::<&str>(&args[1]);

    for line in buffer {
        match line {
            Ok(line) => data.line_type_choice(&line, &args[1], &mut current_material, &mut material_list),
            Err(e) => panic!("Error while retrieving the line {e}"),
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    let mut data: Data = Data::default();

    check_args(&args);
    check_exists(&args[1]);
    check_obj_extensions(&args[1], "obj");

    start_parsing(args, &mut data);
}