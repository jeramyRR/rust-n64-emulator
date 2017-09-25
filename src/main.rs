mod cpu;
mod coprocessor0;

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use cpu::Cpu;



fn main() {
    // We are starting from 1 because 0 is the program name
    // TODO(jeramy): Come back later and use clap and console
    // crates to add some nicety to the commandline.
    let pif_file_name = args().nth(1).unwrap();
    let rom_file_name = args().nth(2).unwrap();

    let pif = load_rom(pif_file_name);
    let rom = load_rom(rom_file_name);

    let mut cpu = Cpu::new();
    cpu.power_on_reset();
    println!("Cpu after reset: {:#?}", cpu);
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
