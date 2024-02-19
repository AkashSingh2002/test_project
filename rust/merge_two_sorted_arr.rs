fn merge_sorted_arrays(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::new();
    while !arr1.is_empty() && !arr2.is_empty() {
        if arr1[0] < arr2[0] {
            merged.push(arr1.remove(0));
        } else {
            merged.push(arr2.remove(0));
        }
    }
    merged.append(&mut arr1);
    merged.append(&mut arr2);
    merged
}
