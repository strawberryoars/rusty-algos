
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_facotrial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
        assert_eq!(factorial(7), 5040);
        assert_eq!(factorial(8), 40320);
        assert_eq!(factorial(9), 362880);
   }
}

fn factorial(x: i32) -> i32 {
    if x == 1 || x == 0 {
        return 1;
    }

    return x * factorial(x-1);
}

fn main() {
    let result = factorial(4);
    println!("{:?}", result);
}
