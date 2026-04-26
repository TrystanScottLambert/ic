fn parse(repr: String) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let result = parse("1 + 2 / 3 * 4 - 6".to_string());

        assert!((result - -2.3333333333333335).abs() < 1e-9);
    }
}
