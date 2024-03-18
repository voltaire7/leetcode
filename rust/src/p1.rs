#[allow(unused)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, n1) in nums.iter().enumerate() {
        for (j, n2) in nums.iter().enumerate() {
            if i != j && n1 + n2 == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

pub fn test() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    assert_eq!(two_sum(vec![1, 2, 3, 4, 5], 9), vec![3, 4]);
}
