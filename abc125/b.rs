fn main() {
  let n = read_num();
  let v = read_nums();
  let c = read_nums();

  let mut result = 0;
  for i in 0..n as usize {
    let d = v[i] - c[i];
    if d > 0 {
      result += d;
    }
  }
  println!("{}", result)
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