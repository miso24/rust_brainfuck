use brainfuck::interpreter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = "+[>.,<]";
    let mut interp = interpreter::Interpreter::new(2);

    interp.run(code)?;
    Ok(())
}
