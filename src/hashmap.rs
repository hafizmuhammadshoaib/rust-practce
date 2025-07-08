use std::collections::HashMap;

pub fn main() {
    word_frequency_count()
}

fn word_frequency_count() {
    let str = "hello world hello";
    let words = str.split(" ");
    let mut count_by_words: HashMap<String, u32> = HashMap::new();

    for word in words {
        let count = count_by_words.get(word);
        if count.is_none() {
            count_by_words.insert(String::from(word), 1);
        } else {
            let new_count = *count.unwrap() + 1;
            count_by_words.insert(String::from(word), new_count);
        }
    }

    for (_, value) in count_by_words.iter().enumerate() {
        println!("key: {}, frequency: {}", value.0, value.1);
    }
}
