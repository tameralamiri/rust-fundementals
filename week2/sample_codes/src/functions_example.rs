use std::io;

fn avergage(numbers: &[i32]) -> f32 {
    // let mut result = 0;
    let sum: i32 = numbers.iter().sum();
    if !numbers.is_empty() {
        let length = numbers.len() as f32;
        println!("Sum {}, Length {}", sum, length);
        sum as f32 / length
    } else {
        0.0
    }
}

fn main() {
    // There are no variadic arguments in Rust
    let mut input = String::new();
    println!("Enter the number of numbers you want to get their avergage");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n_inputs: u32 = input.trim().parse().expect("Invalild number");
    let mut numbers = Vec::new();
    
    for i in 1..=n_inputs {
        input.clear();
        println!("Enter the {} number:", i);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let number: i32 = input.trim().parse().expect("Invalid number");
        numbers.push(number);
    }
    let avg = avergage(&numbers);
    println!("The average of your input: {}", avg);
}
