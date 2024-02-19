fn median(arr: &[i32]) -> f32 {
    let n = arr.len();
    let mid = n / 2;
    if n % 2 == 0 {
      (arr[mid - 1] as f32 + arr[mid] as f32) / 2.0
    } else {
      arr[mid] as f32
    }
  }
  