fn main(){
    // Arrays: [T; size] fixed size, elements of the same type T
    // Declare array, initialize all values, compiler infers length = 7
    // When accessing an array, use square brackets and the index number
    // Indexing starts at 0
    // if index is out of bounds, the program will throw an error.
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    
    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];

    // Vectors: <vector><T> 
    // As with arrays, vectors store multiple values that have the same data type. 
    // Unlike arrays, the size or length of a vector can grow or shrink at any time.
    // Declare vector, initialize with three values
    // Compiler infers type of vector from the first value
    // Vec<T> is a generic type, so it can store any type of value
    // Use square brackets to access elements of a vector
    // Indexing starts at 0
    // if index is out of bounds, the program will panic.
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);  
    
    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();

    // Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    // Pop off value at end of vector
    // Call pop() method from inside println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    // Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    // Add 5 to the value at index 1, which is 5 + 3 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);
}