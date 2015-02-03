#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(os)]
#![feature(io)]
#![feature(path)]
#![feature(collections)]

extern crate rgb;
extern crate getopts;

use std::os;
use std::old_io::{stderr};
use getopts::{Options, HasArg, Occur};

fn disclaimer() {
    //! Print author, version, license... at the start of
    //! the program
    println!("RGB emulator {}", rgb::VERSION);
    println!("Copyright (C) 2015 {}", rgb::AUTHORS.connect(", "));
    println!("License MIT: http://opensource.org/licenses/MIT");
}

fn usage(opts: Options) {
    println!("{}", opts.usage("Usage: rgbemu <FILE> [-h]"));
}

fn run<'a>(filename: &'a String) {
    println!("Running {}", filename);
    let c = rgb::Cartridge::from_file(&Path::new(filename));
}

fn main() {
    let args: Vec<String> = std::os::args();
    let mut options = Options::new();

    options.opt("h", "help", "display this help message", "",
                HasArg::No, Occur::Optional);

    let matches = match options.parse(args.tail()) {
        Ok(m)  => m,
        Err(e) => {
            let _ = writeln!(&mut stderr(), "error: {}", e.to_string());
            usage(options);
            os::set_exit_status(1);
            return;
        },
    };

    if matches.opt_present("h") {
        usage(options);
        return;
    }

    if matches.free.len()!=1 {
        let _ = writeln!(&mut stderr(), "error: Need a single an input file.");
        usage(options);
        os::set_exit_status(1);
        return;
    }

    disclaimer();

    run(&matches.free[0]);
}
