use std::fs;
use std::collections::HashMap;
fn main() {
    //let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //let even_numbers: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).copied().collect();
    //let even_number_10: Vec<i32> = even_numbers.iter().map(|&x| x * 10).collect();
    //let sum: i32 = numbers.into_iter().filter(|&x| x > 5).sum();

    let text = fs::read_to_string("poem.txt").expect("Error");
    let contents: Vec<String> = text.split_whitespace()
        .map(|word| 
            word.chars()
            .filter(|c| !c.is_ascii_punctuation())
            .flat_map(|c| c.to_lowercase())
            .collect()
        )
        .collect();
    let quantity_word = contents.len();
    let mut word_count: HashMap<String, u16> = HashMap::new();
    for word in contents {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    dbg!(word_count);
}