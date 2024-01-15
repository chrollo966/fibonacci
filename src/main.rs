use std::io;

fn main() {
    println!("What term of the Fibonacci sequence do you want to find?");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line.");
    let n: u32 = n.trim().parse().expect("Input a natural number.");

    let fib = fibonacci(n);
    println!("{}-th term of the Fibonacci sequence is: {}", n, fib);
}

fn fibonacci(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 1..n {
        let next = a + b;
        a = b;
        b = next;
    }

    a
}