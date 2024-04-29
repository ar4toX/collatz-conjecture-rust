use std::io;
use std::time::Duration;
use std::thread::sleep;

/*
    By ar4toX on 28, April, 2024 after watching a Veritasium video on YouTube.
    Done for fun and just because Rust makes me feel good.
*/

fn main() {
    let mut input_line = String::new();

    println!("Input whatever integer you'd like: ");

    io::stdin().read_line(&mut input_line).expect("Invalid Entry");

    let number: i64 = input_line
        .trim().parse().expect("That number is out of bounds");

    collatz_time(number);
}

fn collatz_time(mut num: i64) {
    sleep(Duration::new(0, 5_000_000)); 
    //Sleep so that the program doesn't crash immediately

    if num % 2 == 0{ //Check if number is even
        num=num/2;
        println!("      {}", num); //Spacing for the sake of spacing
        collatz_time(num); //1
    }
    
    else {
        if num==1 {
            println!("Reached 1"); //This tells you when num reaches
                                   //1, giving the opportunity to
                                   //test with different arithmetic
                                   //operations to prove the famous
                                   //conjecture.
        }
        num=(num*3)+1;
        println!("{}", num);
        collatz_time(num); //1
    }
    //1: A bit of recursion is always fun
}