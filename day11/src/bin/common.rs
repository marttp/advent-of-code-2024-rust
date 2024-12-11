use std::collections::HashMap;

pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn first_rule_producer(input: &Vec<u64>, n: u64) -> Vec<u64> {
    let mut blink_amount = n;
    /*
        If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
        If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
        If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
        No matter how the stones change, their order is preserved
    */
    let mut result: Vec<u64> = input.clone();
    let mut memo: HashMap<u64, Vec<u64>> = HashMap::new();
    while blink_amount > 0 {
        let mut new_result: Vec<u64> = Vec::new();
        for element in result.iter() {
            if element == &0 {
                new_result.push(1);
                continue;
            }
            if let Some(existing) = memo.get(&element) {
                new_result.extend(existing.clone());
            } else {
                let element_result = if let Some(vec_result) = even_number_digit(*element) {
                    vec_result.clone()
                } else {
                    vec![*element * 2024]
                };
                memo.insert(*element, element_result.clone());
                new_result.extend(element_result.clone());
            }
        }
        result = new_result;
        blink_amount -= 1;
        println!("{:?}", blink_amount);
    }
    result
}

fn even_number_digit(n: u64) -> Option<Vec<u64>> {
    let mut digit_count = 0;
    let mut temp = n;
    while temp > 0 {
        digit_count += 1;
        temp /= 10;
    }
    if digit_count % 2 == 0 {
        let divisor = 10_u64.pow((digit_count / 2) as u32);
        let right = n % divisor;
        let left = n / divisor;
        Some(vec![left, right])
    } else {
        None
    }
}
