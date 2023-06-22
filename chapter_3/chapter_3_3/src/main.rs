fn main() {
    print_labeled_measurement(5, "he");
}

fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The measurement is: {value}{unit_label}");
}