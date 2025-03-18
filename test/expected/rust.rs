pub fn first2_or_third(first: bool, second: bool, third: bool) -> bool {
    /*
     * Add boolean operators between the inputs to make the test pass, keep the parameters in the same order as the function signature
     * Write your logic here
     */
    first && second || third
}

pub enum Shape {
    Circle {
        // Write your logic here
        todo!();
    },
    Rectangle {
        // Write your logic here
        top_left_x: f32,
        top_left_y: f32,
        bottom_right_x: f32,
        bottom_right_y: f32,
    },
}

pub struct Token {
    // Write your logic here
    todo!();
}

/// Greet the person referred to in the `text` parameter.
///
/// ```
/// assert_eq!(rust_ex::hello("world"), "Hello, world!");
/// assert_eq!(rust_ex::hello("Rust"), "Hello, Rust!");
/// ```
/// Write your logic here
pub fn hello(text: &str) -> String {
    // Write here
    format!("Hello, {}!", text)
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    // Write your code here
    let mut i = 0;
    while i < input.len() {
        let mut current_token = String::new();
        let c = input.chars().nth(i).unwrap();
        todo!();
        i += 1;
    }
    tokens
}