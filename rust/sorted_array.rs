fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len() - 1;
  
    while start <= end {
      let mid = (start + end) / 2;
      if arr[mid] == target {
        return Some(mid);
      } else if arr[mid] < target {
        start = mid + 1;
      } else {
        end = mid - 1;
      }
    }
  
    None
  }
  