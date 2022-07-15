mod tokenizer;
mod codegen;

fn main() {
    let test = r#""
    @vstart 0x00

    @p _main
    intr 0x01
    mov_ans r0
    intr 0x01
    mov_ans r1
    add r0, r1
    mov_ans r0
    intr 0x00
    jmp 0x7c05
    ""#;
    let tlist = tokenizer::Token::tokenizer(test);
    println!("{:?}", tlist);
    let a = codegen::gen_code_from(tlist);
    for i in a {
        println!("0x{:02X}", i);
    }
}
