mod longest_word;
use std::io;
use std::collections::HashMap;

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    // iterate over the characters in the sentence
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
            _ => continue,
        }
    }

    // iterate over the charecters in the sentence to count each vowel separately and store in a hashmap
    let mut vowels = HashMap::new();
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                let count = vowels.entry(c).or_insert(0); // returns a mutable reference to the value
                *count += 1; // dereference the value and increment: dereference operator * is used to access the value
            }
            _ => continue,
        }
    }
    println!("{:?}", vowels);
    // Split and collect into a vector
    // let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    // Call the longest_word function from the longest_word module
    println!("Enter a sentence:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    println!("{}", longest_word::longest_word(&input));
}
