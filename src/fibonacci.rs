pub fn calculate_tribonacci(n: u32) -> Result<u32, &'static str> {
    if !(1..=40).contains(&n) {
        return Err("n must be an integer in the range 1 to 40");
    }

    let mut values = (0, 0, 1);

    match n {
        1 => return Ok(values.0),
        2 => return Ok(values.1),
        3 => return Ok(values.2),
        _ => (),
    }

    for _ in 3..n {
        values = (values.1, values.2, values.0 + values.1 + values.2);
    }

    Ok(values.2)
}

#[cfg(test)]
mod tests {
    use crate::fibonacci::calculate_tribonacci;
    use test_case::test_case;

    #[test]
    fn calculate_tribonacci_given_value_less_than_1_should_return_an_error() {
        assert!(calculate_tribonacci(0).is_err());
    }

    #[test]
    fn calculate_tribonacci_given_value_greater_than_40_should_return_an_error() {
        assert!(calculate_tribonacci(41).is_err());
    }

    #[test_case(1, 0)]
    #[test_case(2, 0)]
    #[test_case(3, 1)]
    #[test_case(4, 1)]
    #[test_case(5, 2)]
    #[test_case(6, 4)]
    #[test_case(7, 7)]
    #[test_case(8, 13)]
    #[test_case(9, 24)]
    #[test_case(10, 44)]
    fn calculate_tribonacci_given_value_greater_than_zero_should_return_correct_result(
        n: u32,
        expected_result: u32,
    ) {
        assert_eq!(expected_result, calculate_tribonacci(n).unwrap());
    }
}
