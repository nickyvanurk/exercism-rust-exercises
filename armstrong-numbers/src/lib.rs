pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits_from_num(num);
    let sum = digits.iter().map(|d| d.pow(digits.len() as u32)).sum::<u32>();

    num == sum
}

fn get_digits_from_num(num: u32) -> Vec<u32> {
    num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
}
