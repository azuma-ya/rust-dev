use std::io;

fn fibonacci(n: u32) -> u32 {
    if n == 0 {return 1}
    else if n == 1 {return 1}
    else {fibonacci(n-1) + fibonacci(n-2)}
}

fn main() {
    // println!("Please input celsius.");

    // let mut celsius = String::new();
    
    // io::stdin().read_line(&mut celsius).expect("Failed to read line");

    // let celsius: f64 = celsius.trim().parse().expect("Failed to convert");
    // let fahrenheit = celsius * 9.0 / 5.0 + 32.0;

    // println!("celsius {} is fahrenheit {}", celsius, fahrenheit);

    println!("Please input n.");

    let mut n = String::new();
    
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Failed to convert");

    let a: u32 = fibonacci(n);

    println!("fibonacci({}) = {}",n ,a);
    
}
