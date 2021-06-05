use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}


fn main(){
    println!("Simple Rust Calculator!!");
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operatorlol = String::new();

    print!("Write The First Number: ");
    read(&mut num1);

    print!("Write The Second Number: ");
    read(&mut num2);

    print!("Please Choose an Operator?\n[+, -, /, *]");
    read(&mut operatorlol);
    let num1:f64 = num1.trim().parse().unwrap();
    let num2:f64 = num2.trim().parse().unwrap();
    let operatorlol:char = operatorlol.trim().chars().next().unwrap();
    
    let operators = String::from("+-*/");
    if !operators.contains(operatorlol) {
        println!("Unkonw Operator");
        return;
    }

    let result = match operatorlol {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("Incorrect Operator, Make Sure You Input Correct Operator!")
    };

    println!("Final Result Of {} {} {} = {}", num1, operatorlol, num2, result);
}