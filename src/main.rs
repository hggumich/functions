fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_parameters(5);

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(5);
    println!("The value of y: {y}");
}

fn another_function() {
    println!("Another Function");
}

fn another_function_with_parameters(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(y: i32) -> i32 {
    y + 1
}
