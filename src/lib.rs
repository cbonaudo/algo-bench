use std::str::Chars;

pub fn get_common_prefix_igni(words: Vec<&str>) -> String {
    let mut result = "".to_string();

    let mut allchars: Vec<Chars> = words.iter().map(|word| word.chars()).collect();

    while let Some(next_char) = allchars
        .iter_mut()
        .map(|chars| chars.next())
        .reduce(|prev, curr| {
            if prev == curr {
                prev
            } else {
                None
            }
        })
    {
        if next_char.is_some() {
            result.push(next_char.unwrap());
        } else {
            break;
        }
    }
    
    result
}

pub fn get_common_prefix_myo(words: Vec<&str>) -> String {
    let base_len = words[0].len();
    
    for i in (0..=base_len).rev() {
        if words
            .iter()
            .all(|word| word[0..i] == words[0][0..i])
            == true
        {
            return words[0][0..i].to_string();
        }
    }

    String::new()
}




pub fn get_common_prefix_if(words: Vec<&str>) -> String {
    let mut result = "".to_string();

    for i in 0..words[0].len() {
        if words.iter().all(|word| word.chars().nth(i).unwrap() == words[0].chars().nth(i).unwrap()) {
            result.push(words[0].chars().nth(i).unwrap());
        } else {
            break;
        }
    }

    result
}
