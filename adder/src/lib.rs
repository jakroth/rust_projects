pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {}.",
//                 value
//             );
//         } else if value > 100 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {}.",
//                 value
//             );
//         }

//         Guess { value }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn it_works2() -> Result<(), String> {
    //     if 2 + 2 == 4 {
    //         Ok(())
    //     } else {
    //         Err(String::from("two plus two does not equal four"))
    //     }
    // }

    // // #[test]
    // // fn it_fails() {
    // //     panic!("make this test fail!");
    // // }

    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };

    //     assert!(larger.can_hold(&smaller));
    // }

    // #[test]
    // #[should_panic(expected = "less than or equal to 100")]
    // fn greater_than_100() {
    //     Guess::new(200);
    // }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
