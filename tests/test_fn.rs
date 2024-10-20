//[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trapezoidal_rule_with_square() {
        let result = trapezoidal_rule(0.0, 1.0, 100, |x| x * x);
        let expected = 1.0 / 3.0; // Exact result of the integral of x^2 from 0 to 1
        assert!((result - expected).abs() < 0.001); // Allow small numerical error
    }
}
