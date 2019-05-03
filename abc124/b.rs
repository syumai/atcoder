fn main() {
    let n = read_num();
    let h = read_nums();
    let mut r = 1;
    let mut max_h = h[0];
    for i in 1..n as usize {
        if h[i] >= max_h {
            r += 1;
            max_h = h[i];
        }
    }
    println!("{}", r);
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn read_num() -> i32 {
    read_line().parse().unwrap()
}

fn read_nums() -> Vec<i32> {
    read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}