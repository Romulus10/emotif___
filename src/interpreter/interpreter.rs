/**
 * @author Sean Batzel
 * Generate a compiled vector of intermediate code.
 */

use std::io;

use stack::Stack;
use super::types::Emotifuck;

pub struct State {
    pub program: Vec<i32>,
    pub stack: Stack,
    pub pointer: i32,
    pub data: Vec<i32>
}

pub fn compile(instruction_vector: Vec<Emotifuck>) -> Vec<i32>{
    let mut program = Vec::new();
    let mut stack = Stack::new();
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
                program.push(8 + stack.pop());
            }
            Emotifuck::Input => program.push(7),
            Emotifuck::Output => program.push(6)
        }
        pc += 1;
    }
    program.push(0)
    State { 
        program: program,
        stack: stack,
        pointer: 0,
        data: Vec::new()
    }
}

pub fn interpret(state: State) {
    let mut pc = 0;
    while state.program[pc] != 0 {
        match state.program[pc] {
            1 => state.pointer += 1,
            2 => state.pointer -= 1,
            3 => state.data[state.pointer] += 1,
            4 => state.data[state.pointer] += 1,
            5 => {
                if state.data[state.pointer] == 0 {
                    pc = state.program[pc] - 8;
                }
            },
            6 => state.data[state.pointer] = io::stdin()
                                                 .bytes()
                                                 .next()
                                                 .and_then(|result| result.ok())
                                                 .map(|byte| byte as i32),
            7 => io::stdout().write(state.data[state.pointer])?,
            _ => {
                if state.data[state.pointer] == 0 {
                    pc = state.program[pc] - 8;
                }
            }
        }
    }
}
