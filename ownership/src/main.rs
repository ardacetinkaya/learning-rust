fn main() {
    println!("Hello, world!");

    // ERROR:"cannot assign twice to immutable variable"
    // let message = "hello";
    // message = "world";
    // println!("{}",message);
    //---------------------

    // mutable variables can be assigned twice
    let mut number = 34;
    number = 06;
    println!("{}", number);
    //---------------------

    let mut message = String::from("hello"); // message#1
    message.push_str(", world!");
    println!("{}", message);

    {
        let message = String::from("Arda"); // message#2
    } // message#2 is destroyed
      // message#1
    println!("{}", message);

    //---------------------

    let x = 5;
    let y = x;
    println!("x={}, y={}", x, y);
    //---------------------

    // ERROR:"value borrowed here after move"
    // let message1 = String::from("hello");
    // let message2 = message1;
    // println!("{}, world!", message1);
    //---------------------

    let message1 = String::from("hello");
    let message2 = message1.clone();
    println!("{}, world!, {} mars!", message1, message1);
    //---------------------

    let mut message1 = String::from("hello");
    let message2 = message1.clone();
    message1 = "hi".to_string();
    println!("{}, world!, {} mars!", message1, message1);
    //---------------------

    let message1 = String::from("hello");
    takes_ownership(message1);

    let number = 100;
    makes_copy(number);
    println!("{}", number);

    let message1 = String::from("HELLO WORLD");
    let message2 = takes_and_gives_back(message1);
    println!("{}",message2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(mut some_integer: i32) {
    println!("First value :{}", some_integer);
    some_integer += 100;
    println!("After +=100 :{}", some_integer);
}

fn takes_and_gives_back(mut some_string: String) -> String {
    some_string.push_str("!");
    some_string
}
