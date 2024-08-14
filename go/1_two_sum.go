package main


import "fmt"


// func twoSum(nums []int, target int) []int {
//     for i := 0; i < len(nums); i++ {
//         for j := i+1; j < len(nums); j++ {
//             if i == j {
//                 continue
//             }
//             if nums[i] + nums[j] == target {
//                 return []int{i, j}
//             }
//         }
//     }
//     return nil
// }



func twoSum(nums []int, target int) []int {
    tracking := make(map[int]int)
    res := make([]int, 2)
    for idx, num := range nums {
        if val, ok := tracking[target-num]; ok { // if `ok` is true, there is a number `val` such that val + num = target
             res[0] = val
             res[1] = idx
             return res
        }
        tracking[num] = idx
    }
	return nil
}


func main() {
    nums := []int{2, 7, 11, 15}
    target := 9

    res := twoSum(nums, target)

    fmt.Println(res)
}
