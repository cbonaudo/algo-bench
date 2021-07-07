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

pub fn get_common_prefix_sha(words: Vec<&str>) -> String {
    let mut strs = words.iter();
    let mut longest_prefix = *strs.next().unwrap();

    for candidate in strs {
        let difference_index = candidate
            .char_indices()
            .zip(longest_prefix.chars())
            .find_map(
                |((idx, candidate), prefix)| if candidate != prefix { Some(idx) } else { None },
            );

        match difference_index {
            None if longest_prefix.len() > candidate.len() => {
                longest_prefix = longest_prefix.split_at(candidate.len()).0
            }

            None => {}

            Some(idx) => longest_prefix = longest_prefix.split_at(idx).0,
        }
    }

    longest_prefix.to_string()
}
