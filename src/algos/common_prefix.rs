use std::str::Chars;

pub fn get_common_prefix_igni(words: Vec<&str>) -> String {
    fn advance_prefix<'a>(mut common_prefix: String, mut all_chars: Vec<Chars<'a>>) -> String {
        let mut next_letters = all_chars.iter_mut().map(|cs| cs.next());

        if let Some(Some(l)) = next_letters.next() {
            if next_letters.all(|maybe_l| maybe_l == Some(l)) {
                common_prefix.push(l);
                return advance_prefix(common_prefix, all_chars);
            }
        }
        common_prefix
    }

    advance_prefix(
        String::with_capacity(32),
        words.iter().map(|word| word.chars()).collect(),
    )
}

pub fn get_common_prefix_myo(words: Vec<&str>) -> String {
    let base_len = words[0].len();

    for i in (0..=base_len).rev() {
        if words.iter().all(|word| {
            let word_leng = {
                if i <= word.len() {
                    i
                } else {
                    word.len()
                }
            };
            word[0..word_leng] == words[0][0..i]
        }) == true
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
