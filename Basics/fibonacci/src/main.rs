fn fibonacci(n: u32) -> u32 {
    // todo!("Implementar fibonacci aqui")

    if n == 0 {
        return 0;
    }

    let mut prev : u32 = 0;
    let mut curr : u32 = 1;

    let mut i : u32 = 2;
    while i <= n {
        let next = prev + curr;
        prev = curr;
        curr = next;
        i += 1;
    }
    curr
}


#[cfg(test)]
mod fibonacci_test {

    #[test]
    fn test_fibonacci() {
        assert_eq!(super::fibonacci(0), 0);
        assert_eq!(super::fibonacci(1), 1);
        assert_eq!(super::fibonacci(2), 1);
        assert_eq!(super::fibonacci(3), 2);
        assert_eq!(super::fibonacci(4), 3);
        assert_eq!(super::fibonacci(5), 5);
        assert_eq!(super::fibonacci(6), 8);
        assert_eq!(super::fibonacci(7), 13);
        assert_eq!(super::fibonacci(8), 21);
        assert_eq!(super::fibonacci(9), 34);
        assert_eq!(super::fibonacci(10), 55);
    }

}

fn main() {}