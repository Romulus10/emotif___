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
    pub program: Vec<Instruction>,
    pub stack: Vec<i32>,
}
const MOVR: i32 = 1;
const MOVL: i32 = 2;
const INC: i32 = 3;
const DEC: i32 = 4;
const JMP_F: i32 = 5;
const OUT: i32 = 6;
const IN: i32 = 7;
const JMP_BK: i32 = 8;
const DATA_SIZE: usize = 1024;

pub fn compile(instruction_vector: Vec<Emotifuck>) -> State {
    let mut program = Vec::new();
    let mut stack = Vec::new();
    let mut pc = 0;
    for i in instruction_vector {
        match i {
            Emotifuck::MoveRight => program.push(Instruction { op_code: MOVR, operand: 0 }),
            Emotifuck::MoveLeft => program.push(Instruction { op_code: MOVL, operand: 0 }),
            Emotifuck::Increment => program.push(Instruction { op_code: INC, operand: 0 }),
            Emotifuck::Decrement => program.push(Instruction { op_code: DEC, operand: 0 }),
            Emotifuck::JumpForward => {
                program.push(Instruction { op_code: JMP_F, operand: 0 });
                stack.push(pc)
            },
            Emotifuck::Output => program.push(Instruction { op_code: OUT, operand: 0 }),
            Emotifuck::Input => program.push(Instruction { op_code: IN, operand: 0 }),
            Emotifuck::JumpBackward => {
                if let Some(x) = stack.pop() {
                    program.push(Instruction { op_code: JMP_BK, operand: x });
                    program[x as usize].operand = pc;
                }
            },
            _ => pc -= 1,
        }
        pc += 1;
    }
    program.push(Instruction { op_code: 0, operand: 0 });
    State { 
        program: program,
        stack: stack,
    }
}

pub fn interpret(state: State) {
    let mut pc: usize = 0;
    let mut ptr: usize = 0;
    let program = state.program.as_slice();
    let mut data = [0; DATA_SIZE];
    'prog: loop {
        if pc >= DATA_SIZE { break 'prog }
        match program[pc].op_code {
            0 => {break 'prog},
            MOVR => ptr += 1,
            MOVL => ptr -= 1,
            DEC => { data[ptr] -= 1 },
            INC => { data[ptr] += 1 },
            OUT => { 
                //println!("DATA[PTR] {}", data[ptr]);
                io::stdout().write(&[data[ptr] as u8]); 
            },
            IN => data[ptr] = io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as i32)
                .unwrap(),
            JMP_F => {
                if data[ptr] != 0 {
                    pc = program[pc].operand as usize;
                }
            },
            JMP_BK => {
                if data[ptr] == 0 {
                    pc = program[pc].operand as usize;
                }
            },
            _ => {}
        }
        pc += 1;
    }
}
