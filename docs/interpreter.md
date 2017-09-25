# The Emotif**k Interpreter
## Structures/Constants
The interpreter includes two `structs` and 9 `const` values which are used for internal representation.

### Constants
| Constant  |
| :------:  |
| MOVR      | 
| MOVL      |
| INC       |
| DEC       |
| JMP_F     |
| OUT       |
| IN        |
| JMP_BK    |
| DATA_SIZE |

Each of these constants is used as an opcode in the `Instruction` structure and is used to determine what operation to carry out. The `DATA_SIZE` constant is used to define how large the 'memory tape' is supposed to be.

### Instruction
#### Definition
```
pub struct Instruction {
    pub op_code: i32,
    pub operand: i32
}
```
#### What it Does
The instruction structure consists of two components, an opcode and an operand. The operand is only used for the `JMP_F` and `JMP_BK` operations and determines which location the jump operation should resume execution at.

### Instruction
#### Definition
```
pub struct State {
    pub program: Vec<Instruction>,
    pub stack: Vec<i32>
}
```
#### What it Does
The `State` struct has two vectors. One is the full emotif\*\*k program rendered down to Instruction structures. The other is the stack used for determining the location that JMP commands should point to. When `JMP_F` commands are encountered, the current location in the program is pushed to the internal stack. When a `JMP_BK` operation is encountered, the last value on the stack is popped and the two JMP commands are matched by setting their operands equal to their counterpart's location. This allows the interpreter to jump conditionally between two points.

## Functions

### Compile
#### Definition
`pub fn compile(instruction_vector: Vec<Emotifuck>) -> State`
#### What it Does
This function travels along the vector of parsed instructions and renders them into `Instruction` objects. Each intruction is matched with an opcode until the end of the program is reached, at which point an `Instruction { op_code 0, operand: 0 }` is inserted to inform the interpreter that it has reached the end of the program. The resulting `State` object is then returned and passed into the `interpret` function, which actually executes the program.

### Interpret
#### Definition
`pub fn interpret(state: State)`
#### What it Does
This takes a program `State` consisting of a `Vec` of `Instructions` and a stack. It travels along `state.program` and operates on an array `data` of `usize` with length `DATA_SIZE`. It moves along the array incrementing/decrementing the value at the current point in the `data` array until it reaches an `Instruction` with `op_code=0`.
