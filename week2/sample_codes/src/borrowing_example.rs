fn own_vec(mut vector: Vec<i32>) {
    vector.push(10);
    println!("{:?}", vector);
}

fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: String) {
    println!("{}", s);
}

fn borrow_vec(mut vector: &Vec<i32>){
    println!("{:?}", vector);
}

fn borrow_string(s: &String) {
    let new_s = String::from(s) + "ABC";
    println!("{}",new_s);
}

// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function 
// or another part of your program without actually transferring ownership of the variable. 
// When you borrow a variable, you're essentially saying 
// "I want to use this variable for a little while, but I promise I won't modify it."
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_string = String::from("Hello, world!");

    // this compiles no problem!
    own_integer(my_int);
    println!("{}", my_int);

    // With ownership we have errors
    // own_string(my_string); // take ownership of my_string
    // this is using my_string which has also moved and is invalid
    // println!("{:?}", my_string); // this will not compile!
    // Using borrowing instead:
    borrow_string(&my_string);
    println!("{:?}", my_string);



    // own_vec(my_vec);
    // but this is using my_vec which was borrowed (moved) and yet is now invalid
    // println!("{:?}", my_vec); // this will not compile!
    // Using borrowing instead:
    borrow_vec(&my_vec);
    println!("{:?}", my_vec);

}

// Borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient. 
// By lending ownership of a variable instead of transferring it, Rust ensures that only 
// one part of your program can modify the variable at a time, which helps prevent 
// bugs and makes it easier to reason about your code.