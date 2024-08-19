
fn main() {
    let mut nums = vec![0,1,2,2,3,0,4,2];
    let val = 2;
    let res = Solution::remove_element(&mut nums, val);
    println!("{}", res);
    println!("{:?}", nums);
}


struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut begin = 0;
        let mut end = nums.len();

        while begin < end {
            if nums[begin] == val {
                end -= 1;
                nums[begin] = nums[end];
            } else {
                begin += 1;
            }
        }
        end as i32
    }
}
