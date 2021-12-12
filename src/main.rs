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

    #[test]
    fn day3_ex1() {
        let gamma_x_epsilon = fs::read_to_string("examples/3-1")
            .map_err(to_module_error)
            .and_then(parse::p3_1)
            .map(solve::p3_1);
        assert_eq!(gamma_x_epsilon, Ok(22*9))
    }

    #[test]
    fn day3_ex2() {
        let oxygen_x_co2 = fs::read_to_string("examples/3-1")
            .map_err(to_module_error)
            .and_then(parse::p3_1)
            .map(solve::p3_2);
        assert_eq!(oxygen_x_co2, Ok(23*10))
    }

    #[test]
    fn day4_ex1() {
        let bingo_score = fs::read_to_string("examples/4-1")
            .map_err(to_module_error)
            .and_then(parse::p4_1)
            .map(solve::p4_1);
        assert_eq!(bingo_score, Ok(4512))
    }

    #[test]
    fn day4_ex2() {
        let bingo_worst_score = fs::read_to_string("examples/4-1")
            .map_err(to_module_error)
            .and_then(parse::p4_1)
            .map(solve::p4_2);
        assert_eq!(bingo_worst_score, Ok(1924))
    }

    #[test]
    fn day5_ex1() {
        let nb_overlaps = fs::read_to_string("examples/5-1")
            .map_err(to_module_error)
            .and_then(parse::p5_1)
            .map(solve::p5_1);
        assert_eq!(nb_overlaps, Ok(5))
    }

    #[test]
    fn day5_ex2() {
        let nb_overlaps = fs::read_to_string("examples/5-1")
            .map_err(to_module_error)
            .and_then(parse::p5_1)
            .map(solve::p5_2);
        assert_eq!(nb_overlaps, Ok(12))
    }

    #[test]
    fn day6_ex1() {
        let parsed_input = fs::read_to_string("examples/6-1")
            .map_err(to_module_error)
            .and_then(parse::p6_1);
        assert_eq!(
            parsed_input.as_ref().map(|data| solve::p6_1(18, data.clone())),
            Ok(26));
        assert_eq!(
            parsed_input.map(|data| solve::p6_1(80, data)),
            Ok(5934))
    }

    #[test]
    fn day6_ex2() {
        let fishcount = fs::read_to_string("examples/6-1")
            .map_err(to_module_error)
            .and_then(parse::p6_1)
            .map(|data| solve::p6_2(256, data));
        assert_eq!(fishcount, Ok(26984457539));
    }

    #[test]
    fn day7_ex1() {
        let fuel = fs::read_to_string("examples/7-1")
            .map_err(to_module_error)
            .and_then(parse::p6_1)
            .map(solve::p7_1);
        assert_eq!(fuel, Ok(37));
    }

    #[test]
    fn day7_ex2() {
        let fuel = fs::read_to_string("examples/7-1")
            .map_err(to_module_error)
            .and_then(parse::p6_1)
            .map(solve::p7_2);
        assert_eq!(fuel, Ok(168));
    }

    #[test]
    fn day7_ex2_test2() {
        let brute_fuel = fs::read_to_string("data/7-1")
            .map_err(to_module_error)
            .and_then(parse::p6_1)
            .map(solve::_p7_2_brute);
        let fuel = fs::read_to_string("data/7-1")
            .map_err(to_module_error)
            .and_then(parse::p6_1)
            .map(solve::p7_2);
        assert_eq!(fuel, brute_fuel);
    }

    #[test]
    fn day8_ex1() {
        let nb_special_digits = fs::read_to_string("examples/8-1")
            .map_err(to_module_error)
            .and_then(parse::p8_1)
            .map(solve::p8_1);
        assert_eq!(nb_special_digits, Ok(26));
    }

    #[test]
    fn day8_ex2() {
        let nb_special_digits = fs::read_to_string("examples/8-1")
            .map_err(to_module_error)
            .and_then(parse::p8_2)
            .map(solve::p8_2);
        assert_eq!(nb_special_digits, Ok(61229));
    }

    #[test]
    fn day9_ex1() {
        let low_point_risk_level = fs::read_to_string("examples/9-1")
            .map_err(to_module_error)
            .and_then(parse::p9_1)
            .map(solve::p9_1);
        assert_eq!(low_point_risk_level, Ok(15));
    }

    #[test]
    fn day9_ex2() {
        let low_point_risk_level = fs::read_to_string("examples/9-1")
            .map_err(to_module_error)
            .and_then(parse::p9_1)
            .map(solve::p9_2);
        assert_eq!(low_point_risk_level, Ok(1134));
    }

    #[test]
    fn day10_ex1() {
        let low_point_risk_level = fs::read_to_string("examples/10-1")
            .map_err(to_module_error)
            .and_then(parse::p10_1)
            .map(solve::p10_1);
        assert_eq!(low_point_risk_level, Ok(26397));
    }

    #[test]
    fn day10_ex2() {
        let low_point_risk_level = fs::read_to_string("examples/10-1")
            .map_err(to_module_error)
            .and_then(parse::p10_1)
            .map(solve::p10_2);
        assert_eq!(low_point_risk_level, Ok(288957));
    }
}
