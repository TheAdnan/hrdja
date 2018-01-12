fn main() {
  let value1: &'static str = "Selam";
  let mut value2 = value1.to_string();
  value2.push_str(" alejk");
  let value3 = value2.replace("Selam alejk", "merhaba");
  let value4 = replace_with_merhaba(&value2);
  println!("{}, {}", value2, value3);
}


#[cfg(target_os = "linux")]
fn abs(x: i32) -> i32 {
  if x > 0 {
    x
  } else {
    -x
  }
}

#[cfg(target_os = "linux")]
fn replace_with_merhaba(s: &str) -> String {
  s.replace(s, "Merhaba")
}

#[cfg(target_os = "android")]
fn abs(x: i32){
  println!("Syyyyke");
}