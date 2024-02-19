use rand::Rng;

fn kth_smallest(arr: &mut [i32], k: usize) -> i32 {
  let mut rng = rand::thread_rng();
  let pivot_index = partition(arr, &mut rng);
  if pivot_index == k - 1 {
    arr[pivot_index]
  } else if pivot_index > k - 1 {
    kth_smallest(&mut arr[..pivot_index], k)
  } else {
    kth_smallest(&mut arr[pivot_index + 1..], k - pivot_index - 1)
  }
}

fn partition(arr: &mut [i32], rng: &mut Rng) -> usize {
  let pivot_index = rng.gen_range(0, arr.len());
  arr.swap(pivot_index, 0);
  let pivot = arr[0];
  let mut i = 1;
  for j in 1..arr.len() {
    if arr[j] < pivot {
      arr.swap(i, j);
      i += 1;
    }
  }
  arr.swap(0, i - 1);
  i - 1
}
