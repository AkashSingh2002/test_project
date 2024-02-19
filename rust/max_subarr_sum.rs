fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = i32::MIN;
    let mut current_sum = 0;
    for num in arr {
        current_sum = std::cmp::max(*num, current_sum + *num);
        max_sum = std::cmp::max(max_sum, current_sum);
    }
    max_sum
}
