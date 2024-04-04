use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod scanner;
use scanner::Scanner;

fn main() {
    println!("Rana v1");
    let src = String::from("let x\"hello\"1.3");
    let mut sc = Scanner::new(src);
    let token = sc.next_token();
    println!("{:?}", token);

    loop {
        let mut src = String::new();
        print!(">> ");
        stdout().flush().expect("unable to flush the console");
        stdin().read_line(&mut src).expect("Cant read from console");
        print!("{}", src);
        println!("{:?}", scanner::Tt::STRING as u8);
    }
}
