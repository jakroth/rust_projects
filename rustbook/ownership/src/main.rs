fn main() {
    // s is not valid here, it’s not yet declared
    let s = "hello"; // s is valid from this point forward
    let s = String::from("hello");
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`

    // do stuff with s

    // assignment of heap variables is always a move
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{s1}, world!");

    // reassignment destroys the original value
    let mut s = String::from("hello");
    s = String::from("ahoy");
    println!("{s}, world!");

    // deep copies preserve the original
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // assignment of stack values with the Copy trait preserve the original
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function and so is no longer valid here
    //println!("{}", s); // invalid

    let x = 5; // x comes into scope
    makes_copy(x); // because i32 implements the Copy trait, x does NOT move into the function,
    println!("{}", x); // so it's okay to use x afterward

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into
    // takes_and_gives_back, which also moves its return value into s3

    // pass by reference to avoid ownership issues
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    // mutable references
    let mut s = String::from("hello");
    change(&mut s);

    // slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what it refers to, the value is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
