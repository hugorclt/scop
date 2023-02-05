
use std::path::Path;
use std::ffi::OsStr;

pub fn check_args(args: &Vec<String>)
{
    if args.len() != 2 {
        panic!("Wrong numbers of arguments {} provided, format is: {{cargo run ./path_to_file}}", args.len() - 1);
    }
}

pub fn check_exists(args: &str)
{
    if !Path::new(&args).exists() {
        panic!("{args} doesn't exist");
    }
}

fn get_extensions_from_filename(path: &str) -> Option<&str> {
    Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
}

pub fn check_obj_extensions(path: &str, ext: &str) {
    let extensions: Option<&str> = get_extensions_from_filename(path);
    
    if (extensions.is_some() && extensions.unwrap() == ext) == false {
        panic!("Error on line {path} file extensions have to be : {ext}");
    }
}