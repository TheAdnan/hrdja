fn main() {
  let value1: &'static str = "Selam";
  let mut value2 = value1.to_string();
  value2.push_str(" alejk");
  let value3 = value2.replace("Selam alejk", "merhaba");
  let value4 = replace_with_merhaba(&value2);
  

//Arrays and vectors
  let someArray = ["This", "is", "an", "array"];

  let mut aVector: Vec<&str> = vec!["This", "is", "a", "Vector"];


  aVector.pop();
  aVector.push("also a Vector");


  for a in aVector{
    println!("{:?}", a);
  }

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