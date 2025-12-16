use custom_macro_lib::AnswerFn;
use custom_macro_lib::log_execution;
use custom_macro_lib::make_map;

#[log_execution]
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

#[derive(AnswerFn)]
struct Answer;

fn main() {
    println!("Answer is: {}", Answer::answer());
     greet("World"); 
     let map = make_map! { "one" => 1, "two" => 2, "three" => 3  };
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

}
