use std::error;
use std::io;
use std::io::Read;
use std::io::Write;

pub fn run(prog: &Vec<u8>) -> Result<(), Box<dyn error::Error>> {
    let mut pc = 0; // PC
    let mut cells = vec![0u8; 30_000]; // Memory Cells
    let mut cc = 0; // Cell Counter

    while pc < prog.len() {
        match prog[pc] as char {
            '<' => cc -= 1,
            '>' => cc += 1,
            '+' => cells[cc] = cells[cc].wrapping_add(1),
            '-' => cells[cc] = cells[cc].wrapping_sub(1),
            '[' if cells[cc] == 0 => {
                let mut level = 1;
                while level > 0 {
                    pc += 1;
                    if prog[pc] as char == '[' {
                        level += 1;
                    } else if prog[pc] as char == ']' {
                        level -= 1;
                    }
                }
            }
            ']' if cells[cc] != 0 => {
                let mut level = 1;
                while level > 0 {
                    pc -= 1;
                    if prog[pc] as char == ']' {
                        level += 1;
                    } else if prog[pc] as char == '[' {
                        level -= 1;
                    }
                }
            },
            '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
            ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => {}
        }

        pc += 1;
    }
    Ok(())
}
