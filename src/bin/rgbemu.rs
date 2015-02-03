#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(path)]

extern crate rgb;

fn main() {
    let c = rgb::Cartridge::from_file(&Path::new("tetris.gb"));
}
