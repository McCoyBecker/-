pub use self::TokenType::{
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET
};

#[derive(Debug)]
pub enum TokenType
{
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET
}

#[derive(Debug)]
pub struct Token
{
    tt: TokenType,
    lit: String,
}

struct Lexer
{
    input: String,
    position: usize,
    read_position: usize,
    ch: u8
}

impl Lexer {
    pub fn new(inp: String) -> Lexer
    {
        let mut l = Lexer{ input: inp, position: 0, read_position: 0, ch: 0 };
        l.read_char();
        return l
    }

    pub fn read_char(&mut self) -> ()
    {
        if self.read_position >= self.input.len()
        {
            self.ch = 0
        }
        else
        {
            self.ch = self.input.as_bytes()[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    pub fn next_token(&mut self) -> Token
    {
        let tok = match self.ch
        {
            b'=' => Token { tt: ASSIGN, lit: "=".to_string()},
            b';' => Token { tt: SEMICOLON, lit: ";".to_string()},
            b'(' => Token { tt: LPAREN, lit: "(".to_string()},
            b')' => Token { tt: RPAREN, lit: ")".to_string()},
            b',' => Token { tt: COMMA, lit: ",".to_string()},
            b'+' => Token { tt: PLUS, lit: "+".to_string()},
            b'{' => Token { tt: LBRACE, lit: "{".to_string()},
            b'}' => Token { tt: RBRACE, lit: "}".to_string()},
            _ => Token { tt: EOF, lit: "".to_string()},
        };
        self.read_char();
        return tok
    }
}


#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::lexer::TokenType::{
        ILLEGAL,
        EOF,
        IDENT,
        INT,
        ASSIGN,
        PLUS,
        COMMA,
        SEMICOLON,
        LPAREN,
        RPAREN,
        LBRACE,
        RBRACE,
        FUNCTION,
        LET
    };

    #[test]
    fn test_next_token()
    {
        let input = "=+(){},;";
        let expected = [
            (ASSIGN, "="), 
            (PLUS, "+"),
            (LPAREN, "("),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RBRACE, "}"),
            (COMMA, ","),
            (SEMICOLON, ";"),
            (EOF, "")
        ];

        let mut l = Lexer::new(input.to_string());
        for i in expected.iter()
        {
            let tok = l.next_token();
            print!("Current input: {}\n", i.1);
            print!("Current token: {:?}\n", tok);
            assert_eq!(tok.lit, i.1);
        }
    }

    #[test]
    fn test_next_token_2()
    {
        let input = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
    x + y;
    };
    let result = add(five, ten);";

    let expected = [
        (LET, "let"),
        (IDENT, "five"),
        (ASSIGN, "="),
        (INT, "5"),
        (SEMICOLON, ";"),
        (LET, "let"),
        (IDENT, "ten"),
        (ASSIGN, "="),
        (INT, "10"),
        (SEMICOLON, ";"),
        (LET, "let"),
        (IDENT, "add"),
        (ASSIGN, "="),
        (FUNCTION, "fn"),
        (LPAREN, "("),
        (IDENT, "x"),
        (COMMA, ","),
        (IDENT, "y"),
        (RPAREN, ")"),
        (LBRACE, "{"),
        (IDENT, "x"),
        (PLUS, "+"),
        (IDENT, "y"),
        (SEMICOLON, ";"),
        (RBRACE, "}"),
        (SEMICOLON, ";"),
        (LET, "let"),
        (IDENT, "result"),
        (ASSIGN, "="),
        (IDENT, "add"),
        (LPAREN, "("),
        (IDENT, "five"),
        (COMMA, ","),
        (IDENT, "ten"),
        (RPAREN, ")"),
        (SEMICOLON, ";"),
        (EOF, "")
        ];

        let mut l = Lexer::new(input.to_string());
    for i in expected.iter()
        {
            let tok = l.next_token();
            print!("Current input: {}\n", i.1);
            print!("Current token: {:?}\n", tok);
            assert_eq!(tok.lit, i.1);
        }
    }
}
