fn incorrectanswer() {
  const MSG: &str = "That is incorrect.";

  println!("{}", MSG);
}

fn main() {
  let number = 3;
  
  if number == 3 {
    println!("Correct!");
  }
  else {
    incorrectanswer();
  } 
}
