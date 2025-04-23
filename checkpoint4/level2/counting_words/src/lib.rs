use std::collections::HashMap

fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut count = HashMap::new();
    for word in  words.split_whitespace(){
        let worde = word.trim_matches(|c: char| !c.is_alphanumeric());
        if !word.is_empty(){
            *count.entry(worde.to_lowercase()).or_insert(0)+=1;
        }
    }
    count
}