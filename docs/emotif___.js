function main() {
    console.log('Running program.');
    var in_program = document.getElementById('in_program').value;
    console.log('Program text: ' + in_program);
    var intermediate = compile(in_program);
    console.log('Intermediate: ' + intermediate);
    var output = interpret(intermediate);
    console.log('Output should be: ' + output);
    document.getElementById('output').value = output;
    console.log('Program is done.');
}

function compile(program) {
    var pc = 0;
    var program_vec = program.split('');
    var instructions = new Array();
    var operands = new Array();
    var stack = new Array();
    for (x in program_vec) {
        console.log(program_vec[x].charCodeAt(0));
        switch (program_vec[x].charCodeAt(0)) {
            case '🔥'.charCodeAt(0):
                instructions.push(1);
                operands.push(0);
                break;
            case '💯'.charCodeAt(0):
                instructions.push(2);
                operands.push(0);
                break;
            case '💩'.charCodeAt(0):
                instructions.push(3);
                operands.push(0);
                break;
            case '👍'.charCodeAt(0):
                instructions.push(4);
                operands.push(0);
                break;
            case '💞'.charCodeAt(0):
                instructions.push(5);
                operands.push(0);
                break;
            case '🙏'.charCodeAt(0):
                instructions.push(6);
                operands.push(0);
                break;
            case '🌚'.charCodeAt(0):
                instructions.push(7);
                operands.push(0);
                stack.push(pc);
                break;
            case '🐸'.charCodeAt(0):
                var jmp_pc = stack.pop();
                instructions.push(8);
                operands.push(jmp_pc);
                operands[jmp_pc] = pc;
            default:
                pc -= 1;
                break;
        }
        pc += 1;
    }
    instructions.push([0,0]);
    return [instructions, stack];
}

function interpret(state) {
    var pc = 0;
    var ptr = 0;
    var data = new Array(1024);
    var output = "";
    for (x in state[0]) {
        switch (x) {
            case 0:
                break;
            case 1: //right
                ptr++;
                break;
            case 2: //left
                ptr--;
                break;
            case 3: //decrement
                data[ptr]--;
                break;
            case 4: //increment
                data[ptr]++;
                break;
            case 5: //output
                output = output + data[ptr];
                break;
            case 6: //input
                console.log("Not implemented.");
                break;
            case 7: //jmp_f
                if (data[ptr] == 0) {
                    pc = operands[pc];
                }
            case 8: //jmp_bk
                if (data[ptr] != 0) {
                    pc = operands[pc];
                }
        }
        pc++;
    }
    return output;
}
