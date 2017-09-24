/**
 * @author Sean Batzel
 * Generate a compiled vector of intermediate code and run that sucker.
 */

use std::io;
use std::io::Read;
use std::io::Write;

use core::types::Emotifuck;

pub struct Instruction {
    pub op_code: i32,
    pub operand: i32
}

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
            Emotifuck::MoveRight => program.push(Instruction { op_code: 1, operand: 0 }),
            Emotifuck::MoveLeft => program.push(Instruction { op_code: 2, operand: 0 }),
            Emotifuck::Increment => program.push(Instruction { op_code: 3, operand: 0 }),
            Emotifuck::Decrement => program.push(Instruction { op_code: 4, operand: 0 }),
            Emotifuck::JumpForward => {
                program.push(Instruction { op_code: 5, operand: 0 });
                stack.push(pc)
            } 
            Emotifuck::JumpBackward => {
                if let Some(x) = stack.pop() {
                    program.push(Instruction { op_code: 8, operand: x });
                    program[x as usize] = pc;
                }
            }
            Emotifuck::Input => program.push(Instruction { op_code: 7, operand: 0 }),
            Emotifuck::Output => program.push(Instruction { op_code: 6, operand: 0 })
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
    'prog: loop {
        match program[pc].op_code {
            0 => {println!("Reached 0"); break 'prog},
            1 => ptr += 1,
            2 => ptr -= 1,
            3 => data[ptr] += 1,
            4 => data[ptr] += 1,
            5 => {
                if data[ptr] == 0 {
                    pc = program[pc].operand;
                }
            },
            6 => data[ptr] = io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as i32)
                .unwrap(),
            7 => { io::stdout().write(&[data[ptr] as u8]); },
            8 => {
                if data[ptr] == 0 {
                    pc = program[pc].operand;
                }
            }
        }   
    }
}
