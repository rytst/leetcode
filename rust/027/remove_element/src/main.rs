fn main() {
    let mut nums: Vec<i32> = vec![0,1,2,2,3,0,4,2];

    let val = 2;

    let ans = Solution::remove_element(&mut nums, val);

    println!("{}", ans);
    println!("{:?}", nums);
}


struct Solution {}


impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {

        let mut index = 0;
        while index < nums.len() {
            if nums[index] == val {
                nums.remove(index);
            } else {
                index += 1;
            }
        }

        nums.len() as i32
    }
}
