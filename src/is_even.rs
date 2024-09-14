fn main() {
  let ans: bool = is_even(1);
  println!("{}", ans); // Correct usage with println!
}

fn is_even(num: i32) -> bool {
  if num % 2 == 0 {
      return true;
  } else {
      return false;
  }
}
