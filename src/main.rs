use std::io;

fn fib(numberfib: u32) {
    let mut result: u32 = 0;
    for num in 0..(numberfib+1) { result = result + num; }
    println!("El resultado: {}", result);
}

fn main() {
    println!("Por favor, inserte un numero para calcular su n succession de Fibonacci.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    fib(guess);
}
