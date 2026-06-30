pub fn reverse(input: &str) -> String {

    let letters: String = input.chars().rev().collect();

    return letters;
}
