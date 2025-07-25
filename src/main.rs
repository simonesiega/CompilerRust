mod parser;
mod ast;
mod operators;

fn main() {
    let ops = operators::load_system_operators();

    println!("Operatori caricati:");
    for op in ops {
        println!("- {} -> {}", op.name, op.return_type);
        println!("  parametri: {:?}", op.params);
        println!("  body: {}", op.body);
        println!();
    }
}
