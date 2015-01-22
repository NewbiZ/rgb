#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rgb;

fn main() {
    println!("This is rgbemu");

    let mut mmu = rgb::Mmu::new();
    let cpu = rgb::Cpu::new(&mut mmu);
}
