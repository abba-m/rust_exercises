#![allow(unused_variables)]
#![allow(dead_code)]

// mod ex_1_luhn;
mod ex_2_counter;

/* if let */
fn if_let_example() {
    let arg = std::env::args().next();

    if let Some(val) = arg {
        println!("Hello, {val}");
    } else {
        println!("Missing argument?");
    }
}

fn while_let_example() { 
    let mut iter = std::env::args().into_iter().enumerate();

    while let Some((i, val)) = iter.next()  {
        println!("Arg {i} => {val}");
    }
}

fn pattern_matching_example() {
    match std::env::args().nth(1).as_deref() {
        Some("cat") => println!("Will do cat things"),
        Some("ls")  => println!("Will ls some files"),
        Some("mv")  => println!("Let's move some files"),
        Some("rm")  => println!("Uh, dangerous!"),
        None        => println!("Hmm, no program name?"),
        _           => println!("Unknown program name!"),
    }
}



fn main() {
    // if_let_example();
    // while_let_example();
    // pattern_matching_example();   
    // let is_valid = ex_1_luhn::luhn("4263 9826 4026 9299");

    ex_2_counter::counter();
}