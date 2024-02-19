fn longest_common_prefix(strings: &[&str]) -> String {
    let mut prefix = "";
    for i in 0..strings[0].len() {
      let char = strings[0].chars().nth(i).unwrap();
      for string in strings {
        if string.chars().nth(i).unwrap() != char {
          return prefix.to_string();
        }
      }
      prefix = format!("{}{}", prefix, char);
    }
    prefix.to_string()
  }
  