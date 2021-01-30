use tibetan_pronunciation_rust::{*, tibetan_data::TibetanSyllable};

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
