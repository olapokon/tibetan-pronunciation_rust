#![allow(clippy::wildcard_imports)]
mod tibetan_calculator;

use seed::{prelude::*, *};
use tibetan_calculator::{tibetan_data::*, *};

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        tibetan_display: String::from("ཨ"),
        phonetic_display: String::from(""),
        prefix: None,
        superscript: None,
        root: None,
        subscript: None,
        suffix: None,
        second_suffix: None,
    }
}

// `Model` describes our app state.
struct Model {
    tibetan_display: String,
    phonetic_display: String,
    prefix: Option<&'static TibetanCharacter>,
    superscript: Option<&'static TibetanCharacter>,
    root: Option<&'static TibetanCharacter>,
    subscript: Option<&'static TibetanCharacter>,
    suffix: Option<&'static TibetanCharacter>,
    second_suffix: Option<&'static TibetanCharacter>,
}

enum Msg {
    PrefixChanged(String),
    SuperscriptChanged(String),
    RootChanged(String),
    SubscriptChanged(String),
    SuffixChanged(String),
    SecondSuffixChanged(String),
    NoChange,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::PrefixChanged(s) => {
            let c = s.chars().next();
            match c {
                Some(c) => model.prefix = ROOTS.iter().find(|&t| t.tibetan == c),
                None => model.prefix = None,
            }
        }
        Msg::SuperscriptChanged(s) => {
            let c = s.chars().next();
            match c {
                Some(c) => model.superscript = ROOTS.iter().find(|&t| t.tibetan == c),
                None => model.superscript = None,
            }
        }
        Msg::RootChanged(s) => {
            // if empty
            let c = s.chars().next();
            match c {
                Some(c) => model.root = ROOTS.iter().find(|&t| t.tibetan == c),
                None => model.root = None,
            }
        }
        Msg::SubscriptChanged(s) => {
            let c = s.chars().next();
            match c {
                Some(c) => model.subscript = ROOTS.iter().find(|&t| t.tibetan == c),
                None => model.subscript = None,
            }
        }
        Msg::SuffixChanged(s) => {
            let c = s.chars().next();
            match c {
                Some(c) => model.suffix = ROOTS.iter().find(|&t| t.tibetan == c),
                None => model.suffix = None,
            }
        }
        Msg::SecondSuffixChanged(s) => {
            let c = s.chars().next();
            match c {
                Some(c) => model.second_suffix = ROOTS.iter().find(|&t| t.tibetan == c),
                None => model.second_suffix = None,
            }
        }
        Msg::NoChange => (),
    }
    update_displays(model)
}

fn update_displays(model: &mut Model) {
    if model.root.is_none() {
        *model = Model {
            tibetan_display: String::from("ཨ"),
            phonetic_display: String::new(),
            prefix: None,
            superscript: None,
            root: None,
            subscript: None,
            suffix: None,
            second_suffix: None,
        };
        return;
    }
    let syllable = TibetanSyllable {
        root: &model.root.unwrap(),
        prefix: model.prefix,
        superscript: model.superscript,
        subscript: model.subscript,
        suffix: model.suffix,
        second_suffix: model.second_suffix,
    };
    model.tibetan_display = tibetan(&syllable);
    model.phonetic_display = phonetic(&syllable);
}

fn view(model: &Model) -> Node<Msg> {
    let root_chars: Vec<char> = ROOTS.iter().map(|r| r.tibetan).collect();
    let available_subscripts: Vec<char> = {
        if let Some(root) = model.root {
            root.available_subscripts()
        } else {
            vec![]
        }
    };

    div![
        C!["container"],
        div![&model.tibetan_display, C!["display--tibetan"],],
        div![&model.phonetic_display, C!["display--transliteration"],],
        div![
            C!["options"],
            view_character_menu(model.root.is_none(), "prefix", "Prefix", &PREFIXES),
            view_character_menu(
                model.root.is_none(),
                "superscript",
                "Superscript",
                &SUPERSCRIPTS
            ),
            view_character_menu(false, "root", "Root character", &root_chars[..]),
            view_character_menu(
                available_subscripts.len() == 0,
                "subscript",
                "Subscripts",
                &available_subscripts[..]
            ),
            view_character_menu(model.root.is_none(), "suffix", "Suffix 1", &SUFFIXES),
            view_character_menu(
                model.suffix.is_none(),
                "second_suffix",
                "Suffix 2",
                &SECOND_SUFFIXES
            ),
        ],
    ]
}

fn view_character_menu(disabled: bool, identifier: &str, label: &str, options: &[char]) -> Node<Msg> {
    let msg = match identifier {
        "prefix" => Msg::PrefixChanged,
        "superscript" => Msg::SuperscriptChanged,
        "root" => Msg::RootChanged,
        "subscript" => Msg::SubscriptChanged,
        "suffix" => Msg::SuffixChanged,
        "second_suffix" => Msg::SecondSuffixChanged,
        _ => |_| Msg::NoChange,
    };
    div![
        C!["option"],
        div![
            label,
            C!["option__text", IF!(disabled => "option__text--inactive")],
        ],
        select![
            id!(identifier),
            C!["option__select", IF!(disabled => "option__select--inactive")],
            input_ev(Ev::Change, msg),
            IF!(disabled => attrs! {At::Disabled => true,}),
            option![],
            options.iter().enumerate().map(|(i, opt)| {
                option![id!(i.to_string() + "_" + &opt.to_string()), opt.to_string(),]
            })
        ],
    ]
}

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
