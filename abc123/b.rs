fn main() {
  let mut t = Vec::new();
  for _ in 0..5 {
    t.push(read_num());
  }
  let mut smallest = 10;
  let mut result = 0;
  for time in t {
    if time % 10 == 0 {
      result += time;
      continue;
    }
    result += (time / 10 + 1) * 10;
    if time % 10 < smallest {
      smallest = time%10;
    }
  }
  if smallest != 10 {
    result = result - 10 + smallest;
  }
  println!("{}", result);
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn read_num() -> i32 {
    read_line().parse().unwrap()
}
