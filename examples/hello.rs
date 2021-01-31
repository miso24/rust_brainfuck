use brainfuck::interpreter::{Interpreter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut interp = Interpreter::new(10);

    interp.run(code)?;
    Ok(())
}
