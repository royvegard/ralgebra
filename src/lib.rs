//! # Roy's algebra library
//! A very simple library made mostly
//! to learn how to publish stuff on crates.io.

/// Adds two numbers together.
/// 
/// Two lonely numbers where so very sad.
/// One lonely number said to the other:
/// Lets add.
/// 
/// # Examples
/// ```
/// let result = ralgebra::add(4, 5);
/// assert_eq!(result, 9);
/// ```
pub fn add(a : i32, b : i32) -> i32 {
    a + b
}

/// Subtracts the second number from the first one.
/// 
/// Two numbers and a minus sign walks into a bar.
/// 
/// # Examples
/// ```
/// let result = ralgebra::sub(9, 4);
/// assert_eq!(result, 5);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two numbers.
/// 
/// Go forth and multiply, feeble numbers.
/// 
/// # Examples
/// ```
/// let result = ralgebra::mul(3, 6);
/// assert_eq!(result, 18);
/// ```
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides the first number by the second number.
/// 
/// # Examples
/// 
/// ```
/// let result = ralgebra::div(8, 2);
/// assert_eq!(result, 4);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    a / b
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(4, 2), 2);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(2, 3), 6);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(36, 6), 6);
    }
}
