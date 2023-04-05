#![warn(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf
)]

use std::io;
use std::io::Read;
use std::usize;

fn brainfuck(data: &str) -> Vec<u8> {
    const CELL_SIZE: usize = 30000;
    let commands: Vec<u8> = data.as_bytes().to_vec();
    let mut instruction_pointer = 0;

    let mut cells = [0u8; CELL_SIZE]; // `wrapping_add/sub breaks if I change this to `0`?
    let mut data_pointer = 0;
    let mut stack: Vec<usize> = Vec::new();

    let mut input = [0u8; 1];
    let mut output: Vec<u8> = Vec::new();

    while instruction_pointer < commands.len() {
        match commands.get(instruction_pointer) {
            Some(instruction) => match instruction {
                b'>' => data_pointer += 1,
                b'<' => data_pointer -= 1,
                b'+' => {
                    if let Some(cell) = cells.get_mut(data_pointer) {
                        *cell = cell.wrapping_add(1);
                    }
                }
                b'-' => {
                    if let Some(cell) = cells.get_mut(data_pointer) {
                        *cell = cell.wrapping_sub(1);
                    }
                }
                b'[' => match cells.get(data_pointer) {
                    Some(0) => instruction_pointer = bracket_match(instruction_pointer, &commands),
                    None => panic!("Data pointer [{data_pointer}] outside of cells"),
                    _ => stack.push(instruction_pointer),
                },
                b']' => match cells.get(data_pointer) {
                    Some(0) => _ = stack.pop(),
                    None => panic!("Data pointer [{data_pointer}] outside of cells"),
                    _ => match stack.last() {
                        Some(next_pointer) => instruction_pointer = *next_pointer,
                        None => panic!("No matching `[` found to jump to"),
                    },
                },
                // user input doesn't work right now?
                b',' => match io::stdin().read_exact(&mut input) {
                    Ok(_) => {
                        cells[data_pointer] = input[0];
                    }
                    _ => panic!("Failed to read user input"),
                },
                b'.' => {
                    if let Some(cell_content) = cells.get(data_pointer) {
                        output.push(*cell_content);
                    }
                }
                _ => (),
            },
            None => panic!("Instruction pointer [{instruction_pointer}] outside of commands"),
        }

        instruction_pointer += 1;
    }

    output
}

fn bracket_match(instruction_pointer: usize, commands: &[u8]) -> usize {
    let mut depth = 1;
    let mut instruction_pointer = instruction_pointer;
    while depth > 0 {
        instruction_pointer += 1;
        match commands.get(instruction_pointer) {
            Some(instruction) => match instruction {
                b'[' => depth += 1,
                b']' => depth -= 1,
                _ => (),
            },
            None => panic!("instruction pointer [{instruction_pointer}] outside of commands"),
        }
    }
    instruction_pointer
}

fn main() {
    let hello_world = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    let output = brainfuck(hello_world);

    for o in &output {
        if let Ok(output_string) = String::from_utf8(vec![*o]) {
            print!("{output_string}");
        };
    }
}
