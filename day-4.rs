use std::cmp::min;

fn get_digit(x: u32, n: u32) -> u32 {
    let m = u32::pow(10, n);
    let d = u32::pow(10, n - 1);
    return x % m / d;
}

fn to_digits(x: u32) -> [u32; 6] {
    return [get_digit(x, 6), get_digit(x, 5), get_digit(x, 4), get_digit(x, 3), get_digit(x, 2), get_digit(x, 1)];
}

fn get_chain_length(digits: [u32; 6]) -> u32 {
    let mut previous: u32 = 0;
    let mut chain_length = 1;
    let mut shortest = 100;

    for current in digits.iter() {
        if current < &previous {
            return 0;
        }
        if previous == *current {
            chain_length += 1;
        } else if chain_length > 1 {
            shortest = min(chain_length, shortest);
            chain_length = 1;
        }
        previous = *current;
    }
    if chain_length > 1 {
        shortest = min(chain_length, shortest);
    }
    if shortest == 100 {
        return 0;
    }
    return shortest;
}

fn main() {
    let mut solution_1 = 0;
    let mut solution_2 = 0;
    for password in 347312..805915 {
        let digits = to_digits(password);
        let length = get_chain_length(digits);

        if length > 1 {
            solution_1 += 1;
            println!("A {}", password);
        }
        if length == 2 {
            solution_2 += 1;
            println!("B {}", password);
        }
    }
    println!("Found {} solutions to first part, {} to second part.", solution_1, solution_2)
}