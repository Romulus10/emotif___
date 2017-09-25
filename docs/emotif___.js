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
    var stack = new Array();
    for (x in program_vec) {
        switch (x) {
            case '🔥':
                instructions.push([1,0]);
                break;
            case '💯':
                instructions.push([2,0]);
                break;
            case '💩':
                instructions.push([3,0]);
                break;
            case '👍':
                instructions.push([4,0]);
                break;
            case '💞':
                instructions.push([5,0]);
                break;
            case '🙏':
                instructions.push([6,0]);
                break;
            case '🌚':
                instructions.push([7,0]);
                stack.push(pc);
                break;
            case '🐸':
                var jmp_pc = stack.pop();
                instructions.push([8,jmp_pc]);
                program[jmp_pc][1] = pc;
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
                    pc = program[pc][1];
                }
            case 8: //jmp_bk
                if (data[ptr] != 0) {
                    pc = program[pc][1];
                }
        }
        pc++;
    }
    return output;
}
