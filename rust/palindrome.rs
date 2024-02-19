fn is_palindrome(string: &str) -> bool {
    let mut left = 0;
    let mut right = string.len() - 1;
  
    while left < right {
      if string.chars().nth(left) != string.chars().nth(right) {
        return false;
      }
      left += 1;
      right -= 1;
    }
  
    true
  }
  