use custom_macro_lib::AnswerFn;

#[derive(AnswerFn)]
struct Answer;

fn main() {
    println!("Answer is: {}", Answer::answer());
}
