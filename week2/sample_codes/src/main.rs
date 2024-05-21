use std::io;

fn main() {
    let mut message = String::from("Hello, world!");
    message.clear();
    let mut weight = 215.5;
    println!("message: {}, weight:{}", message, weight);
    weight = 210.0;

    let kilos = weight / 2.2;

    println!("message: {}, weight:{} pounds, kilos:{:.2}", message, weight, kilos);

    let proceed = true;

    if proceed {
        println!("We should proceed");
    }
    else {
        println!("We should not proceed");
    }

    let mut counter = 5;
    if counter > 10 {
        println!("Counter is greater than 10");
    }
    else {
        println!("Counter {} is less than or equal to 10", counter);
    }
    counter = 11;
    if counter > 10 && !proceed {
        println!("Counter is greater than 10 and we should not proceed");
    }
    else if counter <= 10 || proceed{
        println!("Counter is less than or equal to 10 or we should proceed");
    }
    else {
        println!("Counter is greater than 10 and we should proceed");
    }

    // Handle optional values
    let maybe_number: Option<i32> = Some(4);  // or None in some cases, Option is a generic type that can be used to handle optional values
    if maybe_number == Some(5) {
        println!("Found 4!");
    }
    else if let Some(num) = maybe_number { // assigning the variable num on the fly
        println!("number: {:?}", num);
    }
    else {
        println!("No number found");
    }

    // Looping

    let mut counter = 0;
    loop { // infinite loop until break equivalent to while true
        counter += 1;
        println!("counter in loop: {}", counter);
        if counter == 10 {
            break;
        }
    }

    let mut x = 0;
    while x < 5 {
        x += 1;
        if x == 3 {continue} // skip the rest of the loop
        println!("x: {}", x);
        
    }

    println!("Working with std::io");
    let mut input = String::new();
    while input.trim() != "move" {
        input.clear();
        println!("enter a msg or type 'move' to move to outside this loop");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You typed: {}", input);
        println!("You typed: {} with trim", input.trim());
    }

    println!("Moving to the next section");
    for i in 0..5 { // 0 to 4 to include 5 use 0..=5
        println!("i: {}", i);
    }
    for i in (0..5).rev() { // reverse to include 5 use (0..=5).rev()
        println!("i: {}", i);
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers.iter() { // without iter() it will take ownership of the vector with iter() it borrows the vector
        println!("number: {}", number);
    }

    for i in 0..=10 {
        if i % 2 == 0 {
            continue; // skip the rest of the loop
        }
        println!("i: {}", i);
        if i == 7 {
            break; // break out of the loop
        }
    }

    // Match control flow
    let mut greeting = String::new();
    println!("Enter a greeting: ");
    io::stdin().read_line(&mut greeting).expect("Failed to read line");
    match greeting.trim().to_lowercase().as_str() {
        "hi" => println!("Hello there!"),
        "goodbye" => println!("bye bye!"),
        _ => println!("can't find a greeting!"),
    }
}
