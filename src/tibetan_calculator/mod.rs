pub mod tibetan_data;

use std::collections::HashMap;

use tibetan_data::*;

/// Returns an entire Tibetan syllable as a String.
pub fn tibetan(syllable: &TibetanSyllable) -> String {
    let mut result = String::new();
    if let Some(prefix) = syllable.prefix {
        result.push(prefix.unicode_code_point);
    }
    if let Some(superscript) = syllable.superscript {
        result.push(superscript.unicode_code_point);
        result.push(syllable.root.unicode_code_point_as_subscript);
    } else {
        result.push(syllable.root.unicode_code_point);
    }
    if let Some(subscript) = syllable.subscript {
        result.push(subscript.unicode_code_point_as_subscript);
    }
    if let Some(suffix) = syllable.suffix {
        result.push(suffix.unicode_code_point);
    }
    if let Some(second_suffix) = syllable.second_suffix {
        result.push(second_suffix.unicode_code_point);
    }
    return result;
}

/// Returns a phonetic representation of a Tibetan syllable in Latin characters.
pub fn phonetic(syllable: &TibetanSyllable) -> String {
    let mut root_phonetic = String::from(syllable.root.phonetic);
    let mut diairesis = false;
    let column = &syllable.root.column;
    let mut tone = Tone::NONE;
    let mut suffix_phonetic: &str = "";

    // Calculate any change in the root, due to the presence of a prefix.
    // The root change of the subscript overrides the root change for a third column root with prefix.
    if syllable.prefix.is_some() || syllable.superscript.is_some() {
        if column == &Column::THIRD {
            if !syllable.root.phonetic_modified_third_column.is_empty() {
                root_phonetic = syllable.root.phonetic_modified_third_column.to_owned();
            }
        }
        if column == &Column::FOURTH {
            tone = Tone::HIGH;
        }
    }

    // Calculate any change in the root, due to the presence of a subscript.
    // This may overwrite changes due to prefix, above.
    if syllable.subscript.is_some() {
        let (changed_root, changed_tone) =
            subscript_phonetic_change(syllable, syllable.subscript.unwrap(), root_phonetic, tone);
        root_phonetic = changed_root;
        tone = changed_tone;
    }

    // Calculate any change in the root, due to the presence of a suffix.
    if syllable.suffix.is_some() {
        if SUFFIXES_THAT_CAUSE_VOWEL_CHANGE.contains(&syllable.suffix.unwrap().tibetan) {
            diairesis = true;
        }
        suffix_phonetic = syllable.suffix.unwrap().phonetic_as_suffix;
    }

    // Combine the appropriate unicode code points to form the final string.
    let mut phonetic = String::from(root_phonetic);
    if diairesis {
        phonetic.push(DIAIRESIS_UNICODE_CODE_POINT);
    }
    if tone == Tone::HIGH {
        phonetic.push(HIGH_TONE_UNICODE_CODE_POINT);
    }
    if tone == Tone::LOW {
        phonetic.push(LOW_TONE_UNICODE_CODE_POINT);
    }
    phonetic.push_str(suffix_phonetic);

    phonetic
}

fn subscript_phonetic_change(
    syllable: &TibetanSyllable,
    subscript: &TibetanCharacter,
    root_phonetic: String,
    tone: Tone,
) -> (String, Tone) {
    // There are three possible subscripts: ra, ya, & la.
    // For each of them, go through all possible combinations with a root.
    return match subscript.tibetan {
        // ra subscript
        'ར' => match syllable.root.tibetan {
            'ཀ' | 'ཏ' | 'པ' => ("tra".to_owned(), Tone::HIGH),
            'ཁ' | 'ཐ' | 'ཕ' => ("thra".to_owned(), Tone::HIGH),
            'ག' | 'ད' | 'བ' => {
                if syllable.superscript.is_some() && syllable.superscript.unwrap().tibetan == 'ས'
                {
                    ("dra".to_owned(), Tone::LOW)
                } else {
                    ("thra".to_owned(), Tone::LOW)
                }
            }
            'ཧ' => ("hra".to_owned(), tone),
            _ => (root_phonetic, tone),
        },
        // la subscript
        'ལ' => match syllable.root.tibetan {
            'ཟ' => ("da".to_owned(), Tone::LOW),
            _ => ("la".to_owned(), Tone::HIGH),
        },
        // ya subscript
        'ཡ' => match syllable.root.tibetan {
            'མ' => ("nya".to_owned(), Tone::LOW),
            'པ' => ("ca".to_owned(), Tone::HIGH),
            'ཕ' => ("cha".to_owned(), Tone::HIGH),
            'བ' => ("cha".to_owned(), Tone::LOW),
            _ => (add_phonetic_ya_subscript(root_phonetic), tone),
        },
        _ => (root_phonetic, tone),
    };
}

// Insert a "y" character before the final "a" character of the phonetic
// representation of the character.
fn add_phonetic_ya_subscript<'a>(root_phonetic: String) -> String {
    let mut r = root_phonetic.split_at(root_phonetic.len() - 1).0.to_owned();
    r.push_str("ya");
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_subscript_ra() {
        let syllable = TibetanSyllable {
            root: ROOTS.iter().find(|&t| t.tibetan == 'ཏ').unwrap(),
            prefix: None,
            superscript: None,
            subscript: ROOTS.iter().find(|&t| t.tibetan == 'ར'),
            suffix: None,
            second_suffix: None,
        };

        assert_eq!("ཏྲ", tibetan(&syllable));
        assert_eq!("trá", phonetic(&syllable));
    }

    #[test]
    fn with_subscript_ya() {
        let syllable = TibetanSyllable {
            root: ROOTS.iter().find(|&t| t.tibetan == 'ག').unwrap(),
            prefix: None,
            superscript: None,
            subscript: ROOTS.iter().find(|&t| t.tibetan == 'ཡ'),
            suffix: None,
            second_suffix: None,
        };

        assert_eq!("གྱ", tibetan(&syllable));
        assert_eq!("khya", phonetic(&syllable));
    }

    #[test]
    fn root_change_with_diairesis() {
        let syllable = TibetanSyllable {
            root: ROOTS.iter().find(|&t| t.tibetan == 'ག').unwrap(),
            prefix: None,
            superscript: ROOTS.iter().find(|&t| t.tibetan == 'ས'),
            subscript: ROOTS.iter().find(|&t| t.tibetan == 'ར'),
            suffix: ROOTS.iter().find(|&t| t.tibetan == 'ལ'),
            second_suffix: None,
        };

        assert_eq!("སྒྲལ", tibetan(&syllable));
        assert_eq!("drä̀l", phonetic(&syllable));
    }
}
