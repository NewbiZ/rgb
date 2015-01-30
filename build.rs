#![allow(unused_variables)]
#![allow(unused_must_use)]
#![feature(collections)]
#![feature(io)]
#![feature(path)]
#![feature(os)]

//extern crate toml;

use std::os;
use std::old_io::File;

fn find_info(path: Path) -> (String, Vec<String>) {
    //! Return a tuple (version, authors) from the Cargo.toml file
    /*
    // Create a path to the desired file
    let display = path.display();

    // Open the path in read-only mode, returns `IoResult<File>`
    let mut file_toml = match File::open(&path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => panic!("error: cannot open {}: {}", display, why.desc),
        Ok(file) => file,
    };

    let str_toml;
    // Read the file contents into a string, returns `IoResult<String>`
    match file_toml.read_to_string() {
        Err(why) => panic!("error: cannot read {}: {}", display, why.desc),
        Ok(string) => str_toml = string,
    }

    // Parse .toml file
    let mut parser = toml::Parser::new(str_toml.as_slice());
    let table_toml = parser.parse();
    assert!(parser.errors.len() == 0, "error: cannot parse {:?}",
            parser.errors.iter().map(|e| {
                (e.desc.clone(), str_toml.slice(e.lo - 5, e.hi + 5))
            }).collect::<Vec<(String, &str)>>());
    assert!(table_toml.is_some());
    let table_toml = toml::Value::Table(table_toml.unwrap());

    // Retrieve [package] author and version from file
    match table_toml {
        toml::Value::Table(table) => {
            let package_toml = table.get("package").unwrap();
            match *package_toml {
                toml::Value::Table(ref package) => {
                    // Retrieve authors and version from [package]
                    let author = package.get("authors").unwrap().as_slice().unwrap();
                    let version = package.get("version").unwrap().as_str().unwrap();
                    println!("Authors: {:?}", author);
                    println!("Version: {:?}", version);
                },
                _ => panic!("oops"),
            };
        },
        _ => panic!("error: malformed toml file {}", display),
    }
    */
    // OH YEAH BABY
    return (String::from_str("0.0.1"), vec![
            String::from_str("Aurelien Vallee")]);
}

fn write_info(path: Path, info: (String, Vec<String>)) {
    //! Write information to a file that will be included in the library
    let mut file = File::create(&path).unwrap();

    let (version, authors) = info;
    file.write_fmt(format_args!("pub static VERSION: &'static str = \"{}\";", version));
    file.write_str("pub static AUTHORS: &'static [&'static str] = & [");
    for author in authors.iter() {
        file.write_fmt(format_args!("    \"{}\",", author));
    }
    file.write_str("];").unwrap();
}

fn main() {
    let path_config = Path::new(os::getenv("OUT_DIR").unwrap()).join("config.rs");
    let path_cargo = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml");

    write_info(path_config, find_info(path_cargo));
}
