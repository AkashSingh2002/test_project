fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..(n as f64).sqrt() as u32 + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
