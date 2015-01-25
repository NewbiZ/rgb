#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rgb;

fn main() {
    println!("RGB debugger {} by {}.", rgb::VERSION, rgb::AUTHORS.connect(", "));

    let gb = rgb::GameBoy::new();
}

