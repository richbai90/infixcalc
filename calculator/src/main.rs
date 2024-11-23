use clap::Parser;
use shunting_yard::{tokenize, evaluate};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The mathematical expression to evaluate
    expression: String,
}


fn main() {
    let args = Args::parse();
    
    match calculate(&args.expression) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

fn calculate(expression: &str) -> Result<f32, String> {
    let tokens = tokenize(expression);
    evaluate(tokens).map_err(|e| e.to_string())
}
