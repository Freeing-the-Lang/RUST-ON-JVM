mod parser;
mod semantic;
mod ir;
mod jvm_backend;
mod classfile;
mod constant_pool;
mod type_descriptor;

fn main() {
    println!("[RUST-ON-JVM] Prototype Compiler");

    let src = r#"
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    "#;

    // 1. Parse
    let ast = parser::parse(src);
    println!("AST = {:#?}", ast);

    // 2. Semantic → IR
    let ir = semantic::analyze(&ast);
    println!("IR = {:#?}", ir);

    // 3. JVM ClassFile 생성
    jvm_backend::emit_classfile(&ir, "Add.class");
    println!("Wrote Add.class");
}
