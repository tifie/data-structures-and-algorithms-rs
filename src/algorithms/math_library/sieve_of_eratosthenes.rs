// エラトステネスの篩
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut ary = vec![true; n + 1];
    for x in (0..).take_while(|&i| i * i <= n) {
        if x < 2 {
            ary[x] = false;
        }
        if ary[x] {
            for mult in (2..).take_while(|&j| j * x <= n) {
                ary[mult * x] = false;
            }
        }
    }
    let mut prime: Vec<usize> = Vec::new();
    for (i, x) in ary.into_iter().enumerate() {
        if x {
            prime.push(i);
        }
    }

    prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes() {
        assert_eq!(sieve_of_eratosthenes(5), vec![2, 3, 5]);

        assert_eq!(sieve_of_eratosthenes(1), vec![]);
    }

    #[test]
    fn summation() {
        assert_eq!(
            sieve_of_eratosthenes(2000000).iter().sum::<usize>(),
            142913828922
        );
    }
}
