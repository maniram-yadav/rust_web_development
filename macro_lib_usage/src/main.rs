use custom_macro_lib::AnswerFn;
use custom_macro_lib::log_execution;

#[log_execution]
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

#[derive(AnswerFn)]
struct Answer;

fn main() {
    println!("Answer is: {}", Answer::answer());
     greet("World"); 
}
