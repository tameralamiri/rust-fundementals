enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

fn main() {
    let shapes = vec![
        Shape::Circle(3.0),
        Shape::Square(4.0),
        Shape::Triangle(3.0, 4.0),
    ];
    let total_area: f64 = shapes.iter()
        .map( |shape| match shape { // map takes a closure with one argument and returns a new iterator
            Shape::Circle(radius) => 3.14 * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Triangle(base, height) => 0.5 * base * height,
        })
        .sum();
    let total_area2: f64 = shapes.iter().fold(0.0, |acc, shape| { // fold takes an initial value and a closure with two arguments: the accumulator and the current value
        match shape {
            Shape::Circle(radius) => acc + 3.14 * radius * radius, // acc is the accumulator and is updated with the current value
            Shape::Square(side) => acc + side * side,
            Shape::Triangle(base, height) => acc + 0.5 * base * height,
        }
    });
    println!("Total area: {}", total_area);
    println!("Total area2: {}", total_area2);
}