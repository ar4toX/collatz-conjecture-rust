use std::io;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut input_line = String::new();

    println!("Input whatever natural number you'd like: ");

    io::stdin().read_line(&mut input_line).expect("oh no");

    let number: i64 = input_line
        .trim().parse().expect("i said natural number you brick");

    is_num_even(number);
}

fn is_num_even(mut num: i64) {
    sleep(Duration::new(0, 5_000_000));
    if num % 2 == 0{
        num=num/2;
        println!("      {}", num);
        is_num_even(num);
    }
    else {
        num=(num*3)+1;
        println!("{}", num);
        is_num_even(num);
    }
}