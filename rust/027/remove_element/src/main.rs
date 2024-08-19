fn main() {
    let mut nums: Vec<i32> = vec![3, 2, 2, 3];

    let val = 3;

    let ans = Solution::remove_element(&mut nums, val);

    println!("{}", ans);
    println!("{:?}", nums);
}


struct Solution {}


impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;

        let mut index = 0;
        while index != nums.len() {
            if nums[index] == val {
                nums.remove(index);
                count += 1;
            } else {
                index += 1;
            }
        }

        count as i32
    }
}
