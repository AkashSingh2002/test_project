fn is_prime(number: u32) -> bool {
    if number <= 1 {
      return false;
    }
    for i in 2..((number as f64).sqrt() as u32 + 1) {
      if number % i == 0 {
        return false;
      }
    }
    true
  }
  