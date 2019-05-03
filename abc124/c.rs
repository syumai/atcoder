fn main() {
    let s = read_line();
    let mut r1 = 0;
    let mut r2 = 0;
    for (i, c) in s.char_indices() {
        match (i%2, c) {
            (0, '0') | (1, '1') => r1 += 1,
            (1, '0') | (0, '1') => r2 += 1,
            _ => (),
        }
    }
    println!("{}", std::cmp::min(r1, r2));
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}