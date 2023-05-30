mod rectangle {
    pub fn area(length: i32, width: i32) -> i32 {
        length * width
    }
}
    // testing to see if the values used in the main function are computing the same.
    // also testing other values to see if they compute correctly.
    // if the test succeeds, test result will be "ok".
    #[cfg(test)]
    mod tests {
        use super::rectangle::*;

        #[test]
        fn test_area() {
            assert_eq!(area(7, 8), 56);
            assert_eq!(area(0, 1), 0);
            assert_eq!(area(5, 1), 5);
            assert_eq!(area(-5, 6), -30);
        }
    }

fn main() {
    // testing to see if the length time the width computes.
    let r_length = 7;
    let r_width = 8;
    let shape = "Rectangle";

    let mut measurments = 0;
    while measurments < 10 {
        println!("integers that can be used: {}", measurments);
        measurments += 1;
    }

    for i in 0..10 {
        println!("simple return loop: {}", i);
    }

    println!("{}: length: {}, width: {}", shape, r_length, r_width);
    let str_1 = String::from("Area of ");
    let str_2 = String::from("the rectangle is: ");
    let str_3 = str_1 + &str_2 + &rectangle::area(r_length, r_width).to_string();
    println!("{}", str_3);
}