use std::{fs::File, io::BufReader};

use byteorder::{BigEndian, ReadBytesExt};
use hardware::vm::{execute_program, VM};

mod hardware;

fn main() {
    let f = File::open("roms/hello-world.obj").expect("Unable to read file");
    let mut f = BufReader::new(f);

    let base_address = f.read_u16::<BigEndian>().expect("Unable te read data") as usize;
    let mut address = base_address;

    let mut vm = VM::new();

    loop {
        match f.read_u16::<BigEndian>() {
            Ok(instruction) => {
                vm.write_memory(address, instruction);
                address += 1
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("OK");
                } else {
                    println!("failed: {}", e)
                }
                break;
            }
        }
    }

    execute_program(&mut vm)
}
