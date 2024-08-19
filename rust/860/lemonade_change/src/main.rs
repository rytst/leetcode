fn main() {

    let bills: Vec<i32> = vec![5, 5, 5, 10, 20];
    // let bills: Vec<i32> = vec![5, 5, 10, 10, 20];

    let res = Solution::lemonade_change(bills);

    println!("{}", res)
}



// we don't need to track of 20 dollar bills
struct LemonadeStand {
    fives: i32,
    tens: i32,
}

impl LemonadeStand {
    fn new() -> Self {
        LemonadeStand { fives: 0, tens: 0 }
    }

    fn can_provide_change(&mut self, bills: Vec<i32>) -> bool {
        for bill in bills {
            if !self.process_transaction(bill) {
                return false;
            }
        }
        true
    }

    fn process_transaction(&mut self, bill: i32) -> bool {
        match bill {
            5 => {
                self.fives += 1;
                true
            }
            10 => {
                if self.fives == 0 {
                    return false;
                }
                self.fives -= 1;
                self.tens += 1;
                true
            }
            20 => self.provide_change_for_20(),
            _ => false, 
        }
    }

    fn provide_change_for_20(&mut self) -> bool {
        if self.tens > 0 && self.fives > 0 {
            self.tens -= 1;
            self.fives -= 1;
        } else if self.fives >= 3 {
            self.fives -= 3;
        } else {
            return false;
        }
        true
    }
}



struct Solution {}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut lemonade = LemonadeStand::new();
        lemonade.can_provide_change(bills)
    }
}
