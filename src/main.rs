use std::io;

fn main() {
    println!("What number do you like?");
    println!("Hint: it's 17");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
       .expect("Your terminal is fucked. You should do something about that.");
    println!("You like {}, eh? Interesting. Wrong, but interesting.", guess);
}
