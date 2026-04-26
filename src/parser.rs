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

fn sanitize_string(repr: &str) -> String {
    repr.replace("**", "^")
}

fn tokenize(repr: &str) -> Vec<String> {
    // 5+(12/3)*sin(3)
    let functions = [
        "sin", "cos", "tan", "arcsin", "arccos", "arctan", "log10", "ln",
    ];
    let mut sanitized = sanitize_string(repr);
    for function in functions {
        if sanitized.contains(function) {}
    }
    let mut grammar = Vec::new();
    for item in sanitized.split_ascii_whitespace() {
        if functions.contains(item) {
            grammar.push(value);
        }
    }
}

fn build_rpn(tokens: Vec<Token>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenize_brackets() {
        tokenize("(5+7)*(3/2)")
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
