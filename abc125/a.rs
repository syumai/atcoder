fn main() {
  let nums = read_nums();
  let a = nums[0];
  let b = nums[1];
  let t = nums[2];
  println!("{}", t / a * b);
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn read_nums() -> Vec<i32> {
    read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}