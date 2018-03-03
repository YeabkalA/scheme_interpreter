
pub enum Keyword {
    Define,
    If,
}

#[derive(Clone)]
pub enum Operator {
    Plus,
    Mult,
    Greater,
    Less,
    Equal,
}

pub enum Token {
    LParen,
    RParen,
    Oper(Operator),
    Constant(String),
    Keyword(Keyword),
}

enum TokenizationState {
    Starting,
    Accumulating,
}

fn is_single_character_token(c: char) -> bool {
    match c {
        '('   => true,
        ')'   => true,
        '+'   => true,
        '*'   => true,
        '>'   => true,
        '<'   => true,
        '='   => true,
        _     => false,
    }
}

fn get_single_character_token(c: char) -> Token {
    match c {
        '('   => Token::LParen,
        ')'   => Token::RParen,
        '+'   => Token::Oper(Operator::Plus),
        '*'   => Token::Oper(Operator::Mult),
        '>'   => Token::Oper(Operator::Greater),
        '<'   => Token::Oper(Operator::Less),
        '='   => Token::Oper(Operator::Equal),
        _     => panic!("Unable to match single character token!"),
    }
}

fn get_constant_or_keyword(v: &Vec<char>) -> Token {
    let s = v.iter().cloned().collect::<String>();
    match s.as_str() {
        "define" => Token::Keyword(Keyword::Define),
        "if"     => Token::Keyword(Keyword::If),
        _                      => Token::Constant(s),
    }
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

pub fn tokenize(s: &str) -> Vec<Token> {
    
    let mut state: TokenizationState = TokenizationState::Starting;
    let mut tokens: Vec<Token> = Vec::<Token>::new();
    let mut accumulation: Vec<char> = Vec::<char>::new();

    for c in s.chars() {
        match state {
            TokenizationState::Starting => {
                if is_single_character_token(c) {
                    tokens.push(get_single_character_token(c));
                } else if is_whitespace(c) {
                    // ignore
                } else {
                    accumulation.push(c);
                    state = TokenizationState::Accumulating;
                }
            },
            TokenizationState::Accumulating => {
                if is_single_character_token(c) {
                    state = TokenizationState::Starting;
                    tokens.push(get_constant_or_keyword(&accumulation));
                    accumulation.clear();
                    tokens.push(get_single_character_token(c));
                } else if is_whitespace(c) {
                    state = TokenizationState::Starting;
                    tokens.push(get_constant_or_keyword(&accumulation));
                    accumulation.clear();
                } else {
                    accumulation.push(c);
                }
            },
        }
    }
    // handle end token, if needed
    if accumulation.len() > 0 {
        tokens.push(get_constant_or_keyword(&accumulation));
    }

    return tokens;
}
