fn incorrectanswer() {
  const MSG: &str = "That is incorrect."; // It tells you that the number is incorrect

  println!("{}", MSG);
}

fn main() {
  let number = 3; // Type a number anything you want
  
  if number == 3 {
    println!("Correct!"); // It will asked you that the answer is correct
  }
  else {
    incorrectanswer(); // It will asked you that the answer is incorrect
  } 
}
