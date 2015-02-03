#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(os)]
#![feature(io)]
#![feature(core)]
#![feature(path)]
#![feature(collections)]

extern crate rgb;
extern crate getopts;

use std::os;
use std::old_io::{stderr, stdout, File, Writer, LineBufferedWriter};
use getopts::{Options, HasArg, Occur};

fn disclaimer() {
    //! Print author, version, license... at the start of
    //! the program
    println!("RGB assembler {}", rgb::VERSION);
    println!("Copyright (C) 2015 {}", rgb::AUTHORS.connect(", "));
    println!("License MIT: http://opensource.org/licenses/MIT");
}

fn usage(opts: Options) {
    println!("{}", opts.usage("Usage: rgbas -i <FILE> [-o <FILE>] [-d] [-h]"));
}

fn assemble<'a>(input: &'a String, output: Option<String>) {
    println!("Assembling {} in {}", input, output.unwrap());
}

fn disassemble<'a>(input: &'a String, output: Option<String>) {
    let mut cpu = rgb::Cpu::new();
    let mut size: usize;

    // Load input file
    {
        let mut file = match File::open(&Path::new(input)) {
            Ok(f) => f,
            Err(e) => {
                let _ = writeln!(&mut stderr(),
                    "error: Failed to open input file ({})", e.desc);
                os::set_exit_status(1);
                return;
            },
        };

        size = match file.read(&mut cpu.mmu.memory) {
            Ok(s)  => s,
            Err(e) => {
                let _ = writeln!(&mut stderr(),
                    "error: Failed to load input file ({})", e.desc);
                os::set_exit_status(1);
                return;
            },
        };
    }

    // Prepare output file
    let mut out = match output {
        Some(o) => match File::create(&Path::new(o)) {
            Ok(f)  => Box::new(LineBufferedWriter::new(f)) as Box<Writer>,
            Err(e) => {
                let _ = writeln!(&mut stderr(),
                    "error: Cannot create output file ({})", e.desc);
                os::set_exit_status(1);
                return;
            },
        },
        None => Box::new(stdout()) as Box<Writer>,
    };


    println!("\nDisassembling {}.\n", input);

    // Write to output file
    while cpu.pc < size as u16 {
        let (instruction, size) = cpu.state();
        let _ = out.write_line(instruction.as_slice());
        cpu.pc += size as u16;
    }
}

fn main() {

    let args: Vec<String> = std::os::args();
    let mut options = Options::new();

    options.opt("o", "output", "output file name", "NAME",
                HasArg::Yes, Occur::Optional);
    options.opt("i", "input", "input file name", "NAME",
                HasArg::Yes, Occur::Req);
    options.opt("d", "disassemble", "switch to disassemble mode", "",
                HasArg::No, Occur::Optional);
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

    let input = matches.opt_str("i").unwrap();
    let output = matches.opt_str("o");

    if !matches.free.is_empty() {
        let _ = writeln!(&mut stderr(), "error: Too many arguments provided.");
        usage(options);
        os::set_exit_status(1);
        return;
    }

    disclaimer();

    if matches.opt_present("d") {
        disassemble(&input, output);
    } else {
        assemble(&input, output);
    }
}
