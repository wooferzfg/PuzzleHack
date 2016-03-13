mod dol;
mod assembler;

use std::io::prelude::*;
use std::fs::File;
use assembler::Assembler;

fn main() {
    let mut asm = String::new();
    let _ = File::open("../src/src/patch.asm")
                .expect("Couldn't find \"src/src/patch.asm\". If you don't need to patch the dol, just create an empty file.")
                .read_to_string(&mut asm);

    let lines = &asm.lines().collect::<Vec<_>>();

    let mut assembler = Assembler::new("../build/intermediate.elf");
    let instructions = &assembler.assemble_all_lines(lines);

    let mut original = Vec::new();
    let _ = File::open("../game/original.dol")
                .expect("Couldn't find \"game/original.dol\". You need to copy the game's main.dol there.")
                .read_to_end(&mut original);

    let mut intermediate = Vec::new();
    let _ = File::open("../build/intermediate.dol")
                .expect("Couldn't find \"build/intermediate.dol\". Did you build the project correctly using \"make\"?")
                .read_to_end(&mut intermediate);

    let mut original = dol::DolFile::new(&original);
    let intermediate = dol::DolFile::new(&intermediate);
    original.append(intermediate);

    // println!("{:#?}", tww);

    original.patch(instructions);

    let data = original.to_bytes();
    let mut file = File::create("../game/sys/main.dol")
                       .expect("Couldn't create \"game/sys/main.dol\". You might need to provide higher privileges.");

    let _ = file.write(&data);
}
