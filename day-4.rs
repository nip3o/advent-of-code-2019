fn get_digit(x: u32, n: u32) -> u32 {
    let m = u32::pow(10, n);
    let d = u32::pow(10, n - 1);
    return x % m / d;
}

fn to_digits(x: u32) -> (u32, u32, u32, u32, u32, u32) {
    return (get_digit(x, 6), get_digit(x, 5), get_digit(x, 4), get_digit(x, 3), get_digit(x, 2), get_digit(x, 1));
}

fn main() {
    let mut solutions = 0;
    for password in 347312..805915 {
        let (a, b, c, d, e, f) = to_digits(password);
        if a <= b && b <=c && c <=d && d <= e && e <= f {
            if a == b || b == c || c == d || d == e || e == f {
                println!("{}", password);
                solutions += 1;
            }
        }
    }
    println!("Found {} solutions", solutions)
}