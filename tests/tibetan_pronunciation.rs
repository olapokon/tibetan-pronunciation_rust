use tibetan_pronunciation_rust::{*, tibetan_data::TibetanSyllable};

#[test]
fn with_subscript_ra() {
    let roots = generate_tibetan_character_map();

    let syllable = TibetanSyllable {
        root: roots.get(&'ཏ').unwrap(),
        prefix: None,
        superscript: None,
        subscript: Some(roots.get(&'ར').unwrap()),
        suffix: None,
        second_suffix: None,
    };

    assert_eq!("ཏྲ", tibetan(&syllable));
    assert_eq!("trá", phonetic(&syllable));
}

#[test]
fn with_subscript_ya() {
    let roots = generate_tibetan_character_map();

    let syllable = TibetanSyllable {
        root: roots.get(&'ག').unwrap(),
        prefix: None,
        superscript: None,
        subscript: Some(roots.get(&'ཡ').unwrap()),
        suffix: None,
        second_suffix: None,
    };

    assert_eq!("གྱ", tibetan(&syllable));
    assert_eq!("khya", phonetic(&syllable));
}

#[test]
fn root_change_with_diairesis() {
    let roots = generate_tibetan_character_map();

    let syllable = TibetanSyllable {
        root: roots.get(&'ག').unwrap(),
        prefix: None,
        superscript: Some(roots.get(&'ས').unwrap()),
        subscript: Some(roots.get(&'ར').unwrap()),
        suffix: Some(roots.get(&'ལ').unwrap()),
        second_suffix: None,
    };

    assert_eq!("སྒྲལ", tibetan(&syllable));
    assert_eq!("drä̀l", phonetic(&syllable));
}
