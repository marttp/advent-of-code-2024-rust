pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn extract_input(input: Vec<&str>) -> (Vec<u64>, Vec<Vec<u64>>) {
    let (eq_results, numbers): (Vec<&str>, Vec<&str>) = input
        .iter()
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .map(|pair| (pair[0], pair[1]))
        .unzip();
    let eq_results = eq_results
        .iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let numbers = numbers
        .iter()
        .map(|list| {
            list.trim()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (eq_results, numbers)
}


pub fn validate_equation(equation_result: u64, num_list: &Vec<u64>, idx: usize, current: u64, is_third_operator: bool) -> bool {
    if num_list.len() == idx {
        return equation_result == current;
    }
    if current > equation_result && !is_third_operator {
        return false;
    }
    let result = validate_equation(equation_result, num_list, idx + 1, current + num_list[idx], is_third_operator) ||
        validate_equation(equation_result, num_list, idx + 1, current * num_list[idx], is_third_operator);
    if result {
        return true;
    }

    if is_third_operator {
        // Concatenation here, Doesn't mean always single digit
        let mut digits = num_list[idx];
        let mut multiplier = 1;
        while digits > 0 {
            digits /= 10;
            multiplier *= 10;
        }
        let new_value = current * multiplier + num_list[idx];
        if validate_equation(equation_result, num_list, idx + 1, new_value, is_third_operator) {
            return true;
        }
    }
    false
}
