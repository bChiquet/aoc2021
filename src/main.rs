mod parse;
mod solve;
mod error;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::parse;
    use crate:: solve;
    use crate::error::{to_module_error};

    #[test]
    fn day1_ex1() {
        let depth_increases = fs::read_to_string("examples/1-1")
            .map_err(to_module_error)
            .and_then(parse::p1_1)
            .map(solve::p1_1);
        assert_eq!(depth_increases, Ok(7))
    }

    #[test]
    fn day1_ex2() {
        let depth_increases = fs::read_to_string("examples/1-1")
            .map_err(to_module_error)
            .and_then(parse::p1_1)
            .map(solve::p1_2);
        assert_eq!(depth_increases, Ok(5))
    }

    #[test]
    fn day2_ex1() {
        let final_pos_x_depth = fs::read_to_string("examples/2-1")
            .map_err(to_module_error)
            .and_then(parse::p2_1)
            .map(solve::p2_1);
        assert_eq!(final_pos_x_depth, Ok(15*10))
    }

    #[test]
    fn day2_ex2() {
        let final_pos_x_depth = fs::read_to_string("examples/2-1")
            .map_err(to_module_error)
            .and_then(parse::p2_1)
            .map(solve::p2_2);
        assert_eq!(final_pos_x_depth, Ok(15*60))
    }
}
