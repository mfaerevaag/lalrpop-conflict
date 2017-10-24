pub mod parser;

fn main() {
    println!("{:?}", parser::parse_Prog(r#"

int a;

int foo(int a);

"#));
}
