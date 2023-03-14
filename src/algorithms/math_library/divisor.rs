// 約数の列挙を返す
fn divisor(n: usize) -> Vec<usize> {
    let mut ary: Vec<usize> = Vec::new();
    for i in (1..).take_while(|&x| x * x <= n) {
        if n % i == 0 {
            ary.push(i);
            ary.push(n / i);
        }
    }
    ary.sort();

    ary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisors() {
        assert_eq!(divisor(60), vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]);
    }
}
