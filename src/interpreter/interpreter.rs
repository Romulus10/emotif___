/**
 * @author Sean Batzel
 * Generate a compiled vector of intermediate code and run that sucker.
 */

use std::io;
use std::io::Read;
use std::io::Write;

use core::types::Emotifuck;

pub struct State {
    pub program: Vec<i32>,
    pub stack: Vec<i32>,
}

pub fn compile(instruction_vector: Vec<Emotifuck>) -> State {
    let mut program = Vec::new();
    let mut stack = Vec::new();
    let mut pc = 0;
    for i in instruction_vector {
        match i {
            Emotifuck::MoveRight => program.push(1),
            Emotifuck::MoveLeft => program.push(2),
            Emotifuck::Increment => program.push(3),
            Emotifuck::Decrement => program.push(4),
            Emotifuck::JumpForward => {
                program.push(5);
                stack.push(pc)
            } 
            Emotifuck::JumpBackward => {
                program.push(8 + stack.pop().unwrap());
            }
            Emotifuck::Input => program.push(7),
            Emotifuck::Output => program.push(6)
        }
        pc += 1;
    }
    program.push(0);
    State { 
        program: program,
        stack: stack,
    }
}

pub fn interpret(state: State) {
    let mut pc: usize = 0;
    let mut ptr: usize = 0;
    let program = state.program.as_slice();
    let mut data = [0; 1024];
    while program[pc] != 0 {
        match program[pc] {
            1 => ptr += 1,
            2 => ptr -= 1,
            3 => data[ptr] += 1,
            4 => data[ptr] += 1,
            5 => {
                if data[ptr] == 0 {
                    pc = (program[pc] - 8) as usize;
                }
            },
            6 => data[ptr] = io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as i32)
                .unwrap(),
            7 => { io::stdout().write(&[data[ptr] as u8]); },
            _ => {
                if data[ptr] == 0 {
                    pc = (program[pc] - 8) as usize;
                }
            }
        }
    }
}
