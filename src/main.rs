use std::io;

fn main() {
    let mut second_num_state: bool = true;

    println!("Rust Calculator");
    println!("Addition -> +");
    println!("Minus -> -");
    println!("Product -> *");
    println!("Division -> /");
    println!("Modulus -> %");
    println!("Square -> s");
    println!("Cube -> c");
    println!("Power -> p");
    println!("Factorial -> f");
    println!("Operator :: ");

    let mut user_in_operator = String::new();

    io::stdin()
        .read_line(&mut user_in_operator)
        .expect("Failed to Read operator");

    let user_in_operator: char = match user_in_operator.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("InValid Input operator");
            return;
        }
    };

    println!("Enter First Number :: ");

    let mut first_num = String::new();

    io::stdin()
        .read_line(&mut first_num)
        .expect("Failed to Read first num");

    let first_num: f64 = match first_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("InValid Input first num");
            return;
        }
    };

    println!("Enter Second Number :: ");

    let mut second_num = String::new();

    if user_in_operator == 's' || user_in_operator == 'c' {
        second_num_state = false;
    }

    if second_num_state {
        io::stdin()
            .read_line(&mut second_num)
            .expect("Failed to Read second number");

        let second_num: f64 = match second_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("InValid Input second number");
                return;
            }
        };
        calculation(first_num, second_num, user_in_operator);
    } else {
        calculation(first_num, 0.0, user_in_operator);
    }
}

fn calculation(first_num: f64, second_num: f64, user_in_operator: char) {
    let mut result: f64 = 0.0;
    match user_in_operator {
        '+' => result = first_num + second_num,
        '-' => result = first_num - second_num,
        '*' => result = first_num * second_num,
        '/' => result = first_num / second_num,
        '%' => result = first_num % second_num,
        's' => result = first_num * first_num,
        'c' => result = first_num * first_num * first_num,
        'p' => result = first_num.powf(second_num),
        _ => println!("Wrong Operator"),
    }
    println!("Result -> {}", result);
}
