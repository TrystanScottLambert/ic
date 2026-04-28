fn parse(repr: &str) -> f64 {
    todo!()
}

enum Functions {
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Log10,
    Ln,
}

enum Grammar {
    Number,
    Function,
    Operation,
    Paren,
}

struct Token<'a> {
    symbol: &'a str,
    precidence: i32,
    grammar: Grammar,
}

impl<'a> Token<'a> {
    fn new(symbol: &'a str) -> Self {
        if ["+", "-"].contains(&symbol) {
            Self {
                symbol,
                precidence: 1,
                grammar: Grammar::Operation,
            }
        } else if ["/", "*"].contains(&symbol) {
            Self {
                symbol,
                precidence: 2,
                grammar: Grammar::Operation,
            }
        } else if symbol == "^" {
            Self {
                symbol,
                precidence: 3,
                grammar: Grammar::Operation,
            }
        } else if ["(", ")"].contains(&symbol) {
            Self {
                symbol,
                precidence: 5,
                grammar: Grammar::Operation,
            }
        } else if [
            "sin", "cos", "tan", "arcsin", "arccos", "arctan", "log10", "ln",
        ]
        .contains(&symbol)
        {
            Self {
                symbol,
                precidence: 4,
                grammar: Grammar::Function,
            }
        } else {
            Self {
                symbol,
                precidence: 0,
                grammar: Grammar::Number,
            }
        }
    }
}

fn sanitize_string(repr: &str) -> String {
    let mut sanitized = repr.to_string();
    sanitized = sanitized.replace("**", "^");
    sanitized = sanitized.replace("arcsin", "ARCSIN");
    sanitized = sanitized.replace("arccos", "ARCCOS");
    sanitized = sanitized.replace("arctan", "ARCTAN");
    sanitized.split_ascii_whitespace().collect()
}

fn tokenize<'a>(repr: &'a str) -> Vec<Token<'a>> {
    // 5+(12/3)*sin(3)
    let mut tokens = Vec::new();
    let functions = [
        "sin", "cos", "tan", "ARCSIN", "ARCCOS", "ARCTAN", "log10", "ln",
    ];
    let mut sanitized = sanitize_string(repr);
    for function in functions {
        if sanitized.contains(function) {
            sanitized = sanitized.replace(function, &format!(" {} ", function));
        }
    }

    let primer = sanitized.split(' ').collect::<Vec<&str>>();
    for prime in primer {
        if functions.contains(&prime) {
            tokens.push(Token::new(&prime.to_lowercase()))
        } else {
            for c in prime.chars() {
                tokens.push(Token::new(&c.to_string()))
            }
        }
    }
    tokens
}

fn build_rpn(tokens: Vec<String>) {
    let functions = [
        "sin", "cos", "tan", "arcsin", "arccos", "arctan", "log10", "ln",
    ];
    let output = Vec::new();
    let operations = Vec::new();
    for token in tokens {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenize_brackets() {
        let result = tokenize("(5+7)*(3/2)");
        dbg!(&result);
        assert_eq!(
            result,
            vec!["(", "5", "+", "7", ")", "*", "(", "3", "/", "2", ")"]
        );
    }

    #[test]
    fn tokenize_functions() {
        let result = tokenize("sin(5+7)*(cos(3)/2)");
        dbg!(&result);
        assert_eq!(
            result,
            vec![
                "sin", "(", "5", "+", "7", ")", "*", "(", "cos", "(", "3", ")", "/", "2", ")"
            ]
        );
    }

    #[test]
    fn tokenize_nested_functions() {
        let result = tokenize("sin(cos(arctan(4)))");
        dbg!(&result);
        assert_eq!(
            result,
            vec!["sin", "(", "cos", "(", "arctan", "(", "4", ")", ")", ")"]
        );
    }

    #[test]
    fn tokenize_exponent() {
        let result = tokenize("5**6");
        assert_eq!(result, vec!["5", "^", "6"]);
    }

    #[test]
    fn simple() {
        let result = parse("1 + 2 / 3 * 4 - 6");
        assert!((result - -2.3333333333333335).abs() < 1e-9);
    }

    #[test]
    fn exponent() {
        let result = parse("8**3");
        assert_eq!(result, 24.);
    }

    #[test]
    fn brackets() {
        let result = parse("(1+3) / (8-10)");
        let more_brackets = parse("((1+3) / (8-10))+0.5");
        assert_eq!(result, 0.5);
        assert_eq!(more_brackets, 1.0);
    }

    #[test]
    fn white_space() {
        assert_eq!(parse("-1*3/10"), parse("-1 * 3 / 10"))
    }
}
