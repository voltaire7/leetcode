pub fn call() {
    dbg!(two_sum(vec![2, 7, 11, 15], 9) == vec![0, 1]);
    dbg!(two_sum(vec![3, 2, 4], 6) == vec![1, 2]);
    dbg!(two_sum(vec![3, 3], 6) == vec![0, 1]);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, n1) in nums.iter().enumerate() {
        for (j, n2) in nums.iter().enumerate() {
            if i != j && n1 + n2 == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}
