pub fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[derive(Debug, Clone)]
pub struct GateCombination {
    pub a: String,
    pub b: String,
    pub gate: String,
    pub output: String,
}

impl GateCombination {
    pub fn new(a: String, b: String, gate: String, output: String) -> GateCombination {
        GateCombination { a, b, gate, output }
    }
}
