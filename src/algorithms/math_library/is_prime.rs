// 素数判定
fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }

    for i in (2..).take_while(|&x| x * x <= n) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(is_prime(2));
    }

    #[test]
    fn case2() {
        assert!(!is_prime(1));
    }

    #[test]
    fn case3() {
        assert!(!is_prime(4));
    }
}
