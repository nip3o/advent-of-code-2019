fn run(mut memory: [i32; 157], noun: i32, verb: i32) -> i32 {
    let mut i = 0;
    memory[1] = noun;
    memory[2] = verb;

    loop {
        match memory[i] {
            1 => {
                memory[memory[i + 3] as usize] = memory[memory[i + 1] as usize] + memory[memory[i + 2] as usize];
            },
            2 => {
                memory[memory[i + 3] as usize] = memory[memory[i + 1] as usize] * memory[memory[i + 2] as usize];
            },
            99 => {
                break;
            },
            _ => {
                println!("Unknown opcode {}", memory[i]);
                break;
            }
        }
        i += 4;
    }
    return memory[0];
}

fn main() {
    static PROGRAM: [i32; 157] = [
        1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,1,6,23,27,2,27,9,31,2,6,31,35,1,5,35,
        39,1,10,39,43,1,43,13,47,1,47,9,51,1,51,9,55,1,55,9,59,2,9,59,63,2,9,63,67,1,5,67,71,2,13,
        71,75,1,6,75,79,1,10,79,83,2,6,83,87,1,87,5,91,1,91,9,95,1,95,10,99,2,9,99,103,1,5,103,
        107,1,5,107,111,2,111,10,115,1,6,115,119,2,10,119,123,1,6,123,127,1,127,5,131,2,9,131,135,
        1,5,135,139,1,139,10,143,1,143,2,147,1,147,5,0,99,2,0,14,0
    ];

    println!("Answer to part one: {}", run(PROGRAM.clone(), 12, 2));

    for noun in 1..100 {
        for verb in 1..100 {
            let result = run(PROGRAM.clone(), noun, verb);
            if result == 19690720 {
                println!("Found noun {} and verb {} => Answer to part two: {}", noun, verb, 100 * noun + verb);
                break;
            }
        }
    }
}