pub fn raindrops(n: u32) -> String {
    const FACTORS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let result: String = FACTORS
        .iter()
        .filter(|(factor, _)| n % factor == 0)
        .flat_map(|(_, sound)| sound.chars())
        .collect();

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
