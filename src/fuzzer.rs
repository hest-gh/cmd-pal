pub fn fuzz(commands: &Vec<String>, search_input: &str) -> Vec<(String, f32)> {
    let fuzz = search_input.to_lowercase();
    let mut results = Vec::new();

    for choice in commands {
        let score = calculate_similarity(&fuzz, choice);
        if score > 0.0 {
            results.push((choice.clone(), score));
        }
    }

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    results
}

fn calculate_similarity(a: &str, b: &str) -> f32 {
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();

    let matching_chars = a_lower.chars().filter(|c| b_lower.contains(*c)).count();
    let total_chars = a_lower.len().max(b_lower.len());

    matching_chars as f32 / total_chars as f32
}
