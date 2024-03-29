enum AddressMode {
    Position,
    Immediate,
}

fn get_digits(x: u32, n: u32, len: u32) -> u32 {
    let m = u32::pow(10, n);
    let d = u32::pow(10, n - len);
    return x % m / d;
}

fn get_mode(instruction: i32, param_index: usize) -> AddressMode {
    let value = get_digits(instruction as u32, 2 + param_index as u32, 1);
    match value {
        0 => return AddressMode::Position,
        1 => return AddressMode::Immediate,
        _ => panic!("Unknown address mode"),
    }
}

fn get_opcode(instruction: u32) -> u32 {
    return get_digits(instruction, 2, 2);
}

fn get_param_value(ip: usize, memory: [i32; 678], param_index: usize) -> i32 {
    match get_mode(memory[ip as usize], param_index) {
        AddressMode::Position => memory[memory[ip + param_index] as usize],
        AddressMode::Immediate => memory[ip + param_index],
    }
}

fn run(mut memory: [i32; 678], input: i32) -> i32 {
    let mut ip: usize = 0;
    loop {
        let opcode = get_opcode(memory[ip] as u32);
        // println!("ip = {}, opcode = {}", ip + 1, opcode);

        match opcode {
            1 => {
                memory[memory[ip + 3] as usize] = get_param_value(ip, memory, 1) + get_param_value(ip, memory, 2);
                ip += 4;
            },
            2 => {
                memory[memory[ip + 3] as usize] = get_param_value(ip, memory, 1) * get_param_value(ip, memory, 2);
                ip += 4;
            },
            3 => {
                memory[memory[ip + 1] as usize] = input;
                ip += 2;
            },
            4 => {
                println!("Output: {}", get_param_value(ip, memory, 1));
                ip += 2;
            },
            5 => {
                if get_param_value(ip, memory, 1) != 0 {
                    ip = get_param_value(ip, memory, 2) as usize;
                } else {
                    ip += 3;
                }
            },
            6 => {
                if get_param_value(ip, memory, 1) == 0 {
                    ip = get_param_value(ip, memory, 2) as usize;
                } else {
                    ip += 3;
                }
            },
            7 => {
                if get_param_value(ip, memory, 1) < get_param_value(ip, memory, 2) {
                    memory[memory[ip + 3] as usize] = 1;
                } else {
                    memory[memory[ip + 3] as usize] = 0;
                }
                ip += 4;
            },
            8 => {
                if get_param_value(ip, memory, 1) == get_param_value(ip, memory, 2) {
                    memory[memory[ip + 3] as usize] = 1;
                } else {
                    memory[memory[ip + 3] as usize] = 0;
                }
                ip += 4;
            },
            99 => {
                break;
            },
            _ => {
                panic!("Unknown opcode {}", opcode);
            }
        }
    }
    return memory[0];
}

fn main() {
    static PROGRAM: [i32; 678] = [
        3,225,1,225,6,6,1100,1,238,225,104,0,1002,114,19,224,1001,224,-646,224,4,224,102,8,223,223,1001,224,7,224,1,223,224,223,1101,40,62,225,1101,60,38,225,1101,30,29,225,2,195,148,224,1001,224,-40,224,4,224,1002,223,8,223,101,2,224,224,1,224,223,223,1001,143,40,224,101,-125,224,224,4,224,1002,223,8,223,1001,224,3,224,1,224,223,223,101,29,139,224,1001,224,-99,224,4,224,1002,223,8,223,1001,224,2,224,1,224,223,223,1101,14,34,225,102,57,39,224,101,-3420,224,224,4,224,102,8,223,223,1001,224,7,224,1,223,224,223,1101,70,40,225,1102,85,69,225,1102,94,5,225,1,36,43,224,101,-92,224,224,4,224,1002,223,8,223,101,1,224,224,1,224,223,223,1102,94,24,224,1001,224,-2256,224,4,224,102,8,223,223,1001,224,1,224,1,223,224,223,1102,8,13,225,1101,36,65,224,1001,224,-101,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,677,226,224,1002,223,2,223,1006,224,329,1001,223,1,223,1108,226,226,224,1002,223,2,223,1005,224,344,101,1,223,223,1108,226,677,224,1002,223,2,223,1006,224,359,101,1,223,223,107,226,226,224,1002,223,2,223,1005,224,374,101,1,223,223,1107,226,226,224,1002,223,2,223,1005,224,389,101,1,223,223,107,677,677,224,102,2,223,223,1006,224,404,101,1,223,223,1008,226,226,224,1002,223,2,223,1006,224,419,101,1,223,223,108,677,226,224,1002,223,2,223,1006,224,434,101,1,223,223,1108,677,226,224,102,2,223,223,1005,224,449,101,1,223,223,1008,677,226,224,102,2,223,223,1006,224,464,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,479,101,1,223,223,7,677,677,224,102,2,223,223,1005,224,494,1001,223,1,223,8,226,677,224,102,2,223,223,1006,224,509,101,1,223,223,107,677,226,224,1002,223,2,223,1005,224,524,1001,223,1,223,7,677,226,224,1002,223,2,223,1005,224,539,1001,223,1,223,1007,226,677,224,1002,223,2,223,1005,224,554,1001,223,1,223,8,677,677,224,102,2,223,223,1006,224,569,101,1,223,223,7,226,677,224,102,2,223,223,1006,224,584,1001,223,1,223,1008,677,677,224,102,2,223,223,1005,224,599,101,1,223,223,1007,677,677,224,1002,223,2,223,1006,224,614,101,1,223,223,1107,677,226,224,1002,223,2,223,1006,224,629,101,1,223,223,1107,226,677,224,1002,223,2,223,1006,224,644,101,1,223,223,1007,226,226,224,102,2,223,223,1005,224,659,1001,223,1,223,108,226,226,224,102,2,223,223,1006,224,674,101,1,223,223,4,223,99,226
    ];
    println!("Running program with input 1.");
    run(PROGRAM.clone(), 1);

    println!("Running program with input 5.");
    run(PROGRAM.clone(), 5);
}
