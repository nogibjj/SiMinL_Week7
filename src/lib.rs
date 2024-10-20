
//takes interval [a,b], number of segments n, and function f as input
// any function that matches Fn(f64) -> 64 works
pub fn trapezoidal_rule<F>(a: f64, b: f64, n: usize, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let mut integral = 0.5 * (f(a) + f(b)); // Starting with the endpoints
    // Sum up the intermediate points
    for i in 1..n {
        let x = a + i as f64 * h;
        integral += f(x);
    }
    integral * h // Multiply by the width of each segment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trapezoidal_rule() {
        // Test with a simple function f(x) = x^2 over the interval [0, 1]
        let result = trapezoidal_rule(0.0, 1.0, 100, |x| x * x);
        let expected = 1.0 / 3.0; // Exact integral of x^2 over [0, 1] is 1/3
        assert!((result - expected).abs() < 0.001); // Allow small error
    }
}

