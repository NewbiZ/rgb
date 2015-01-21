extern crate rgb;

fn main() {
    let mut c = rgb::Cpu::new();


    c.a = 6;
    c.e = 250;

    println!("Cpu = {}", c);
    c.instr_0x84();
    println!("Cpu = {}", c);
}
