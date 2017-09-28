function main() {
    var in_program = document.getElementById('in_program').value;
    var intermediate = compile(in_program);
    var output = interpret(intermediate);
    document.getElementById('output').value = output;
}

function parse_vector(program) {
    program_vec = Array();
    for (var i = 0, len = program.length; i < len; i++) {
        var emoji = program[i].codePointAt(0);
        switch (emoji) {
            case '>'.codePointAt(0):
                program_vec.push(1);
                break;
            case '<'.codePointAt(0):
                program_vec.push(2);
                break;
            case '-'.codePointAt(0):
                program_vec.push(3);
                break;
            case '+'.codePointAt(0):
                program_vec.push(4);
                break;
            case '.'.codePointAt(0):
                program_vec.push(5);
                break;
            case ','.codePointAt(0):
                program_vec.push(6);
                break;
            case '['.codePointAt(0):
                program_vec.push(7);
                break;
            case ']'.codePointAt(0):
                program_vec.push(8);
                break;
        }
    }
    return program_vec;
}

function compile(program) {
    var pc = 0;
    var program_vec = parse_vector(program);
    var instructions = new Array();
    var operands = new Array();
    var stack = new Array();
    for (x in program_vec) {
        switch (program_vec[x]) {
            case 1:
                instructions.push(1);
                operands.push(0);
                break;
            case 2:
                instructions.push(2);
                operands.push(0);
                break;
            case 3:
                instructions.push(3);
                operands.push(0);
                break;
            case 4:
                instructions.push(4);
                operands.push(0);
                break;
            case 5:
                instructions.push(5);
                operands.push(0);
                break;
            case 6:
                instructions.push(6);
                operands.push(0);
                break;
            case 7:
                instructions.push(7);
                operands.push(0);
                stack.push(pc);
                break;
            case 8:
                var jmp_pc = stack.pop();
                instructions.push(8);
                operands.push(jmp_pc);
                operands[jmp_pc] = pc;
            default:
                pc--;
                break;
        }
        pc++;
    }
    instructions.push([0,0]);
    return [instructions, operands];
}

function interpret(state) {
    var ptr = 0;
    var data = new Array(1024);
    for (var i = 0; i < 1024; i++) {
        data[i] = 0;
    }
    var output = "";
    for (var x = 0; x < state[0].length; x++) {
        switch (state[0][x]) {
            case 0:
                break;
            case 1: //right
                ++ptr;
                break;
            case 2: //left
                --ptr;
                break;
            case 3: //decrement
                --data[ptr];
                break;
            case 4: //increment
                ++data[ptr];
                break;
            case 5: //output
                output = output + String.fromCodePoint((data[ptr].toString()));
                break;
            case 6: //input
                break;
            case 7: //jmp_f
                if (data[ptr] == 0) {
                    x = state[1][x];
                }
                break;
            case 8: //jmp_bk
                if (data[ptr] != 0) {
                    x = state[1][x];
                }
                break;
        }
    }
    return output;
}
