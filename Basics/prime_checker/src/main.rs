fn main() {}

fn check_if_prime(n: u32) -> bool {
    // todo!("Implementar check_if_prime aqui")

    // special cases
    if n == 0 || n == 1 {
        return false;
    }

    let mut is_prime = true;
    let mut i : u32 = 2;
    while i < n {  // exclude check division by 1 and itself
        if n % i == 0 {
            is_prime = false;
            break;
        }
        i += 1;
    }
    is_prime
}

#[cfg(test)]
mod prime_checker_test {

    #[test]
    fn test_check_if_prime() {
        assert_eq!(super::check_if_prime(0), false);
        assert_eq!(super::check_if_prime(1), false);
        assert_eq!(super::check_if_prime(2), true);
        assert_eq!(super::check_if_prime(3), true);
        assert_eq!(super::check_if_prime(4), false);
        assert_eq!(super::check_if_prime(5), true);
        assert_eq!(super::check_if_prime(6), false);
        assert_eq!(super::check_if_prime(7), true);
        assert_eq!(super::check_if_prime(8), false);
        assert_eq!(super::check_if_prime(9), false);
        assert_eq!(super::check_if_prime(10), false);
        assert_eq!(super::check_if_prime(11), true);
        assert_eq!(super::check_if_prime(12), false);
        assert_eq!(super::check_if_prime(13), true);
        assert_eq!(super::check_if_prime(14), false);
        assert_eq!(super::check_if_prime(15), false);
        assert_eq!(super::check_if_prime(16), false);
        assert_eq!(super::check_if_prime(17), true);
        assert_eq!(super::check_if_prime(18), false);
        assert_eq!(super::check_if_prime(19), true);
        assert_eq!(super::check_if_prime(20), false);
        assert_eq!(super::check_if_prime(21), false);
        assert_eq!(super::check_if_prime(22), false);
        assert_eq!(super::check_if_prime(23), true);
        assert_eq!(super::check_if_prime(24), false);
        assert_eq!(super::check_if_prime(25), false);
        assert_eq!(super::check_if_prime(26), false);
        assert_eq!(super::check_if_prime(27), false);
        assert_eq!(super::check_if_prime(28), false);
        assert_eq!(super::check_if_prime(29), true);
        assert_eq!(super::check_if_prime(30), false);
        assert_eq!(super::check_if_prime(31), true);
        assert_eq!(super::check_if_prime(32), false);
        assert_eq!(super::check_if_prime(33), false);
        assert_eq!(super::check_if_prime(34), false);
        assert_eq!(super::check_if_prime(35), false);
    }
}