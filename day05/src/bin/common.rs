pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn extract_rules_updates(input: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut rules: Vec<&str> = Vec::new();
    let mut updates: Vec<&str> = Vec::new();
    let mut is_received_update: bool = false;
    for line in input {
        match line.is_empty() {
            true => is_received_update = true,
            _ => {
                if is_received_update {
                    updates.push(line);
                } else {
                    rules.push(line);
                }
            }
        }
    }
    (rules, updates)
}
