fn read_nums() -> Vec<i32> {
  let mut buf = String::new();
  std::io::stdin().read_line(&mut buf);
  buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() {
  let nums = read_nums();
  let a = nums[0];
  let b = nums[1];
  let mut r = 0;
  if a == b {
    r = a * 2;
  } else if a > b {
    r = a * 2 - 1;
  } else {
    r = b * 2 - 1;
  }
  println!("{}", r);
}