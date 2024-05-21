fn main(){
    // Tuples:
    // A tuple is a collection of values of different types. 
    // The data type is based on the data types of its elements, 
    // and the length is fixed based on the number of elements. (you can't add or remove elements from a tuple)
    // Tuple of length 3
    let tuple_e = ('E', 5i32, true);
    // You can access its elements of a Tuple using the dot notation and the index of the element
    println!("tuple_e: ({}, {}, {})", tuple_e.0, tuple_e.1, tuple_e.2);


    // Structs:
    // Classic struct with named fields
    struct Student { name: String, level: u8, remote: bool }

    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);

    // Unit struct
    struct Unit;

    // Instantiate classic struct, specify fields in random order, or in specified order
    // Using String::from() to create a String from a string literal &str
    // An alternative is to use to_string() method on a string literal
    let user_1 = Student { name: String::from("Constance Sharma"), remote: true, level: 2 };
    let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);

    // Enums: are types of any one of several variants

    enum WebEventBasic {
        // An enum variant can be like a unit struct without fields or data types
        WELoad,
        // An enum variant can be like a tuple struct with data types but no named fields
        WEKeys(String, char),
        // An enum variant can be like a classic struct with named fields and their data types
        WEClick { x: i64, y: i64 }
    }

    // Defining Enums with Struct
    // Define a tuple struct
    struct KeyPress(String, char);

    // Define a classic struct
    struct MouseClick { x: i64, y: i64 }

    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

    // Instantiate an enum:
    // Simple Variant:
    let we_load = WebEvent::WELoad(true);

    // Struct Variant:
    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };

    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);

    // Tuple Struct Variant:
    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);
}