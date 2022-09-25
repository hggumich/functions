fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_parameters(5);

    print_labeled_measurement(5, 'h');
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
