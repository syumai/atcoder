fn main() {
  let mut a = Vec::new();
  for _ in 0..5 {
    a.push(read_num());
  }

  let k = read_num();
  for i in 1..5 as usize {
    if a[i] - a[0] > k {
      println!(":(");
      return;
    }
  }
  println!("Yay!");
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn read_num() -> i32 {
    read_line().parse().unwrap()
}