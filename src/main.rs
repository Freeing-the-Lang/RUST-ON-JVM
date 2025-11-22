mod parser;
mod semantic;
mod jvm_backend;

fn main() {
    let src = r#"
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    "#;

    println!("RUST-ON-JVM Prototype\n");

    // 1. Parse
    let ast = parser::parse(src);
    println!("AST = {:#?}", ast);

    // 2. Semantic meaning
    let ir = semantic::analyze(ast);
    println!("IR = {:#?}", ir);

    // 3. Generate JVM bytecode
    let bytecode = jvm_backend::generate(ir);
    println!("JVM Bytecode = {:?}", bytecode);
}
