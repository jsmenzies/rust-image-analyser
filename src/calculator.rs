use std::env::{args, Args};

fn _calculate() {
    let mut args: Args = args();

    let _my_str: &str = "Hello I'm stored on the stack";
    let _my_heap_string_obj: String = String::from("Hello I'm stored on the heap");

    let _my_string = String::from("The quick brown fox");
    let _my_str: &str = &_my_string[4..9]; // "quick"

    let first = args.nth(1).unwrap();
    let operator = args.next().unwrap();
    // let operator = args.next().unwrap().chars().next().unwrap();
    let second = args.next().unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    let operand = operator.parse::<char>().unwrap();

    let result = operate(operand, first_number, second_number);

    println!("{}", output(first_number, operand, second_number, result));
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}

fn operate(operand: char, first_number: f32, second_number: f32) -> f32 {
    match operand {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operand used"),
    }
}
