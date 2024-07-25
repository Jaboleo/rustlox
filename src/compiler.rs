use crate::scanner;

pub fn compile(source: String) {
    let mut scanner = scanner::Scanner::new(source);
    let mut line = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            println!("{}", token.line);
            line = token.line
        } else {
            print!("   | ")
        }
        print!("{:?} {} {}", token.ttype, token.length, token.start);
        if token.ttype == scanner::TokenType::Eof {
            break;
        }
    }
}
