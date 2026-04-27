pub fn is_armstrong_number(num: u32) -> bool {
    let num_to_str = num.to_string();
    let length = num_to_str.len() as u32;
    num_to_str
        .chars()
        .filter_map(|n| n.to_digit(10))
        .map(|n| (n as u64).pow(length))
        .sum::<u64>()
        == num as u64
}
