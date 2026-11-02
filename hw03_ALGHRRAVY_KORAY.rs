use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    
    for word in text.split_whitespace() {
        let lower_word = word.to_lowercase();
        let count = counts.entry(lower_word).or_insert(0);
        *count += 1;
    }
    
    counts
}

fn count_of(counts: &HashMap<String, u32>, word: &str) -> u32 {
    let lower_word = word.to_lowercase();
    *counts.get(&lower_word).unwrap_or(&0)
}

fn total_words(counts: &HashMap<String, u32>) -> u32 {
    counts.values().sum()
}

fn sorted_words(counts: &HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut sorted_vec: Vec<(String, u32)> = counts
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();

    sorted_vec.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    sorted_vec
}

fn main() {
    let text1 = "the cat The dog THE end";
    let text2 = "Rust is fun and Rust is fast";
    let text3 = "";

    let counts1 = count_words(text1);
    println!("Text 1: '{}'", text1);
    println!("Total words: {}", total_words(&counts1));
    println!("'the' count: {}", count_of(&counts1, "the"));
    println!("'horse' count: {}", count_of(&counts1, "horse"));
    println!("Sorted: {:?}\n", sorted_words(&counts1)); 

    let counts2 = count_words(text2);
    println!("Text 2: '{}'", text2);
    println!("Total words: {}", total_words(&counts2));
    println!("Sorted: {:?}\n", sorted_words(&counts2));

    let counts3 = count_words(text3);
    println!("Text 3: (empty)");
    println!("Total words: {}", total_words(&counts3));
    println!("Sorted: {:?}", sorted_words(&counts3));
}
