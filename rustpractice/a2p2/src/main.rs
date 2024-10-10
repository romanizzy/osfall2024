
fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut word_count: Vec<(&str, usize)> = Vec::new();
    
    for word in words{
        let mut found = false;
        for (w, count) in &mut word_count {
            if word == *w {
                *count += 1;
                found = true;
                break;
            }
        }
        if found != true{
            word_count.push((word, 1));
        }
    }

    let mut max_c = 0;
    let mut max_w = "";
    for (w, c) in word_count{
        if c > max_c{
            max_c = c;
            max_w = w;
        }
    }
    let max_word = max_w.to_string();
    let max_count = max_c;
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

