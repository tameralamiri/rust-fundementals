pub fn longest_word(sentence: &str) -> String {
    let words = sentence.split(' ');
    let mut longest = "";
    for word in words {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    longest.to_string()
}