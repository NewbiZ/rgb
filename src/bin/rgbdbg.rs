#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(io)]
#![feature(path)]
#![feature(libc)]
#![feature(core)]
#![feature(collections)]

extern crate rgb;
extern crate linenoise;
extern crate libc;

use std::old_io::File;
use std::old_io::fs;

fn disclaimer() {
    //! Print author, version, license... at the start of
    //! the program
    println!("RGB debugger {}", rgb::VERSION);
    println!("Copyright (C) 2015 {}", rgb::AUTHORS.connect(", "));
    println!("License MIT: http://opensource.org/licenses/MIT");
}

fn repl_read(prompt: &'static str) -> (String, String) {
    //! Print the prompt, then read user input and split
    //! it between command and arguments
    let val = linenoise::prompt(prompt);
    match val {
        // SIGINT, SIGQUIT
        None => {
            command_quit();
            unreachable!();
        },
        // Input provided
        Some(input) => {
            let mut split = input.splitn(1, ' ');
            let command: String = String::from_str(split.next().unwrap());
            let arguments = split.collect();
            linenoise::history_add(input.as_slice());
            (command, arguments)
        }
    }
}

fn repl_eval<'a>(command: String, arguments: String, cpu: &'a mut rgb::Cpu) {
    //! Match the command string with proper command functions
    match command.as_slice() {
        "h" | "help"  => command_help(),
        "q" | "quit"  => command_quit(),
        "n" | "next"  => command_next(cpu),
        "r" | "run"   => command_run(cpu),
        "l" | "list"  => command_list(cpu),
        "d" | "dump"  => command_dump(cpu),
        "p" | "print" => command_print(cpu),
        "f" | "file"  => command_file(arguments, cpu),
        c => command_unknown(String::from_str(c)),
    }
}

fn complete_path(input: String) -> Vec<String> {
    //! This is a helper method that will try to complete a
    //! partially written path to something existing.
    let mut path: String = input;
    if !path.starts_with("/") && !path.starts_with("./") {
        path = String::from_str("./") + path.as_slice();
    }
    let mut split = path.rsplitn(1, '/');
    let child_path = split.next().unwrap();
    let mut parent_path: String = split.collect();
    if parent_path.is_empty() {
        parent_path = String::from_str("/");
    }

    let mut ret: Vec<String> = Vec::new();
    for p in fs::readdir(&Path::new(parent_path.clone())).unwrap().iter() {
        if p.filename_str().unwrap().starts_with(child_path) {
            let mut result: String = parent_path.clone();
            if !result.ends_with("/") {
                result = result + "/";
            }
            result = result + p.filename_str().unwrap();
            ret.push(String::from_str("file ") + result.as_slice());
        }
    }
    ret
}

fn completion_callback(input: &str) -> Vec<String> {
    //! From an incomplete input stream, return a vector of
    //! possible completions
    if input.starts_with("file ") {
        // Autocomplete filenames
        let mut split = input.splitn(1, ' ');
        split.next();
        let path_str: String = split.collect();
        complete_path(path_str)
    } else {
        match input {
            // Autocomplete simple commands
            "h" | "he" | "hel" | "help" | "?"     => vec![String::from_str("help" )],
            "q" | "qu" | "qui" | "quit"           => vec![String::from_str("quit" )],
            "n" | "ne" | "nex" | "next"           => vec![String::from_str("next" )],
            "r" | "ru" | "run"                    => vec![String::from_str("run"  )],
            "l" | "li" | "lis" | "list"           => vec![String::from_str("list" )],
            "d" | "du" | "dum" | "dump"           => vec![String::from_str("dump" )],
            "p" | "pr" | "pri" | "prin" | "print" => vec![String::from_str("print")],
            "f" | "fi" | "fil" | "file"           => vec![String::from_str("file ")],
            // Unknown command
            unknown => vec![String::from_str(unknown)],
        }
    }
}

fn repl() {
    //! Main REPL of the debugger
    linenoise::set_callback(completion_callback);
    let mut cpu: rgb::Cpu = rgb::Cpu::new();
    loop {
        let (command, arguments) = repl_read("(rgbdbg) ");
        let e = repl_eval(command, arguments, &mut cpu);
    }
}

fn command_unknown(command: String) {
    //! Executed when the command is unknown
    println!("Unknown command \"{}\". Try \"help\"", command);
}

fn command_help() {
    //! Print list of available commands
    println!("Commands:");
    println!("  help |h    List available commands");
    println!("  file |f    Load Z80 executable binary");
    println!("  next |n    Step execute the next instruction");
    println!("  run  |r    Run the program until Cpu is stopped");
    println!("  list |l    List upcoming instructions");
    println!("  dump |d    Dump memory at location");
    println!("  print|p    Print the current CPU state");
    println!("  quit |q    Quit");
}

fn command_quit() {
    //! Exit the program
    println!("Bye.");
    unsafe { libc::exit(libc::EXIT_SUCCESS); }
}

fn command_file<'a>(arguments: String, cpu: &'a mut rgb::Cpu) {
    //! Load an executable Z80 binary file
    println!("Loading: {}.", arguments);
    let mut file = match File::open(&Path::new(arguments)) {
        Ok(f) => f,
        Err(e) => {
            println!("error: failed opening file ({})", e.desc);
            return;
        },
    };
    let size = match file.read(&mut cpu.mmu.memory) {
        Ok(s) => s,
        Err(e) => {
            println!("error: failed to load ({})", e.desc);
            cpu.reset();
            return;
        },
    };
    println!("Successfully loaded {} bytes in memory.", size);
}

fn command_print<'a>(cpu: &'a rgb::Cpu) {
    //! Print the current state of the processor: registers,
    //! flags, program counter, stack counter, ...
    print!("{:?}", cpu);
}

fn command_next<'a>(cpu: &'a mut rgb::Cpu) {
    //! Execute the next instruction, and print it
    let (instruction, _) = cpu.state();
    println!("{}", instruction);
    cpu.step();
}

fn command_run<'a>(cpu: &'a mut rgb::Cpu) {
    //! Run the program until the process is stopped
    cpu.reset();
    cpu.stop = false;
    while !cpu.stopped() {
        // Retrieve current instruction and print it
        let (instruction, _) = cpu.state();
        println!("{}", instruction);
        // Execute current instruction
        cpu.step();
    }
}

fn command_list<'a>(cpu: &'a mut rgb::Cpu) {
    //! List the next upcoming instructions
    let backup_pc = cpu.pc;
    let mut count = 10;
    while count>0 {
        let (instruction, size) = cpu.state();
        if cpu.pc==backup_pc
            { print!("-> "); }
        else
            { print!("   "); }
        println!("{}", instruction);
        cpu.pc += size as u16;
        count -= 1;
    }
    cpu.pc = backup_pc;
}

fn command_dump<'a>(cpu: &'a mut rgb::Cpu) {
    //! Dump memory at location
    let mut address = 0;
    while address<=20 {
        print!("@x{:0>4.4X} ", address);
        let b0: u8 = cpu.mmu.read8(address);
        address += 1;
        let b1: u8 = cpu.mmu.read8(address);
        address += 1;
        let b2: u8 = cpu.mmu.read8(address);
        address += 1;
        let b3: u8 = cpu.mmu.read8(address);
        address += 1;
        println!("| x{:0>2.2X} x{:0>2.2X} x{:0>2.2X} x{:0>2.2X} |",
                 b0, b1, b2, b3);
    }
}

fn main() {
    //! Entry point of the debugger, print the disclaimer then
    //! start the REPL
    disclaimer();
    repl();
}
