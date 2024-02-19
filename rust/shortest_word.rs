fn shortest_word(string: &str) -> &str {
    let mut shortest = string.split_whitespace().next().unwrap();
    for word in string.split_whitespace() {
      if word.len() < shortest.len() {
        shortest = word;
      }
    }
    shortest
  }
  