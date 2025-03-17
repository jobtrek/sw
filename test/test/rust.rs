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
        center_x: f32,
        center_y: f32,
        radius: f32,
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
    pub token_type: TokenType,
    pub value: String,
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
        tokens.push(match c {
            '"' => {
                i += 1;
                while input.chars().nth(i).unwrap() != '"' {
                    current_token.push(input.chars().nth(i).unwrap());
                    i += 1;
                }
                Token {
                    token_type: TokenType::String,
                    value: current_token,
                }
            }
            '0'..='9' => {
                while input.chars().nth(i).unwrap().is_digit(10)
                    || input.chars().nth(i).unwrap() == '.'
                {
                    current_token.push(input.chars().nth(i).unwrap());
                    i += 1;
                }
                i -= 1;
                Token {
                    token_type: TokenType::Number,
                    value: current_token,
                }
            }
            '(' | ')' => Token {
                token_type: TokenType::Parenthesis,
                value: c.to_string(),
            },
            '+' | '-' | '*' | '/' => Token {
                token_type: TokenType::Operator,
                value: c.to_string(),
            },
            ';' => Token {
                token_type: TokenType::SemiColon,
                value: c.to_string(),
            },
            ' ' => {
                i += 1;
                continue;
            }
            _ => {
                panic!("Invalid character: {}", c);
            }
        });
        i += 1;
    }
    tokens
}