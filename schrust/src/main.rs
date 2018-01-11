fn main() {
  let value1 = -3;
  println!("{}", abs(value1));
}


#[cfg(target_os = "linux")]
fn abs(x: i32) -> i32 {
  if x > 0 {
    x
  } else {
    -x
  }
}

#[cfg(target_os = "android")]
fn abs(x: i32){
  println!("Syyyyke");
}