fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = vec[0]; // Correct: Copying the value
    vec.push(3);
    println!("Value of x: {}", x);

    //Alternative solution using iterators
    let mut vec2 = Vec::new();
    vec2.push(10);
    vec2.push(20);
    for i in &vec2 {
        println!("Value: {}",i);
    }
    vec2.push(30);
} 