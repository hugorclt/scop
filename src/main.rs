mod reader;
mod checker;
mod parser;

use std::env;
use crate::reader::*;
use crate::checker::*;
use crate::parser::Data;

fn main()
{
    let mut data: Data = Data::default();
    let args: Vec<String> = env::args().collect();

    check_args(&args);
    check_exists(&args[1]);
    check_obj_extensions(&args[1]);

    let buffer = get_buffers::<&str>(&args[1]);

    for line in buffer {
        match line {
            Ok(line) => data.line_type_choice(&line),
            Err(e) => panic!("Error while retrieving the line {e}"),
        }
    }


}
