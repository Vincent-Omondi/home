pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let mut new_arr = Vec::new();
    let mut sum = 0;

    for num in arr.iter() {
        sum += num;
        new_arr.push(sum);
    }

    new_arr.reverse();
    new_arr.push(0);
    return new_arr
}