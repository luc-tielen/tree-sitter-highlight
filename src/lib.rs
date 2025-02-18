use lazy_static::lazy_static;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter, HtmlRenderer};

#[napi]
pub enum Language {
    Eclair,
    TS,
    Bash,
    LLVM,
    C,
    Rust,
    Haskell,
    Lua,
}

lazy_static! {
    static ref ECLAIR_CONFIG: (HighlightConfiguration, Vec<String>, Vec<String>,) = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_eclair::language(),
            tree_sitter_eclair::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .unwrap();

        let mut highlight_names = Vec::new();
        add_highlight_names(&mut config, &mut highlight_names);
        config.configure(&highlight_names);
        let (html_attrs, class_names) = get_attrs(&highlight_names);

        (config, html_attrs, class_names)
    };
    static ref TS_CONFIG: (HighlightConfiguration, Vec<String>, Vec<String>,) = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_typescript::language_typescript(),
            tree_sitter_typescript::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();

        let mut highlight_names = Vec::new();
        add_highlight_names(&mut config, &mut highlight_names);
        config.configure(&highlight_names);
        let (html_attrs, class_names) = get_attrs(&highlight_names);

        (config, html_attrs, class_names)
    };
    static ref BASH_CONFIG: (HighlightConfiguration, Vec<String>, Vec<String>,) = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_bash::language(),
            tree_sitter_bash::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();

        let mut highlight_names = Vec::new();
        add_highlight_names(&mut config, &mut highlight_names);
        config.configure(&highlight_names);
        let (html_attrs, class_names) = get_attrs(&highlight_names);

        (config, html_attrs, class_names)
    };
    static ref LLVM_CONFIG: (HighlightConfiguration, Vec<String>, Vec<String>,) = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_llvm::language(),
            tree_sitter_llvm::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .unwrap();

        let mut highlight_names = Vec::new();
        add_highlight_names(&mut config, &mut highlight_names);
        config.configure(&highlight_names);
        let (html_attrs, class_names) = get_attrs(&highlight_names);

        (config, html_attrs, class_names)
    };
    static ref C_CONFIG: (HighlightConfiguration, Vec<String>, Vec<String>,) = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_c::language(),
            tree_sitter_c::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();

        let mut highlight_names = Vec::new();
        add_highlight_names(&mut config, &mut highlight_names);
        config.configure(&highlight_names);
        let (html_attrs, class_names) = get_attrs(&highlight_names);

        (config, html_attrs, class_names)
    };
    static ref RUST_CONFIG: (HighlightConfiguration, Vec<String>, Vec<String>,) = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();

        let mut highlight_names = Vec::new();
        add_highlight_names(&mut config, &mut highlight_names);
        config.configure(&highlight_names);
        let (html_attrs, class_names) = get_attrs(&highlight_names);

        (config, html_attrs, class_names)
    };
    static ref HASKELL_CONFIG: (HighlightConfiguration, Vec<String>, Vec<String>,) = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_haskell::language(),
            tree_sitter_haskell::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .unwrap();

        let mut highlight_names = Vec::new();
        add_highlight_names(&mut config, &mut highlight_names);
        config.configure(&highlight_names);
        let (html_attrs, class_names) = get_attrs(&highlight_names);

        (config, html_attrs, class_names)
    };
    static ref LUA_CONFIG: (HighlightConfiguration, Vec<String>, Vec<String>,) = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_lua::language(),
            tree_sitter_lua::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .unwrap();

        let mut highlight_names = Vec::new();
        add_highlight_names(&mut config, &mut highlight_names);
        config.configure(&highlight_names);
        let (html_attrs, class_names) = get_attrs(&highlight_names);

        (config, html_attrs, class_names)
    };
}

fn add_highlight_names(config: &HighlightConfiguration, highlight_names: &mut Vec<String>) {
    for name in config.query.capture_names() {
        if !highlight_names.contains(name) {
            highlight_names.push(name.clone());
        }
    }
}

fn get_attrs(highlight_names: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let html_attrs: Vec<String> = highlight_names
        .iter()
        .map(|s| format!("class=\"{}\"", s.replace('.', " ")))
        .collect();

    let class_names: Vec<String> = highlight_names
        .iter()
        .map(|s| s.replace('.', " "))
        .collect();

    (html_attrs, class_names)
}

fn load_language<'a>(
    language: Language,
) -> (&'a HighlightConfiguration, &'a Vec<String>, &'a Vec<String>) {
    let (config, html_attrs, class_names) = match language {
        Language::Eclair => &*ECLAIR_CONFIG,
        Language::TS => &*TS_CONFIG,
        Language::Bash => &*BASH_CONFIG,
        Language::C => &*C_CONFIG,
        Language::LLVM => &*LLVM_CONFIG,
        Language::Haskell => &*HASKELL_CONFIG,
        Language::Rust => &*RUST_CONFIG,
        Language::Lua => &*LUA_CONFIG,
    };

    (&config, &html_attrs, &class_names)
}

#[napi]
fn highlight(code: String, language: Language) -> String {
    let (config, html_attrs, _) = load_language(language);
    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(&config, code.as_bytes(), None, |_| None)
        .unwrap();

    let mut renderer = HtmlRenderer::new();
    renderer
        .render(highlights, code.as_bytes(), &|highlight| {
            html_attrs[highlight.0].as_bytes()
        })
        .unwrap();
    unsafe { String::from_utf8_unchecked(renderer.html) }
}

#[derive(Debug)]
#[napi(object)]
struct HastProperties {
    pub class_name: String,
}

#[derive(Debug)]
#[napi(object)]
struct HastNode {
    #[napi(js_name = "type")]
    pub kind: String,
    pub tag_name: String,
    pub properties: HastProperties,
    pub children: Vec<Either<HastNode, HastTextNode>>,
}

#[derive(Debug)]
#[napi(object)]
struct HastTextNode {
    #[napi(js_name = "type")]
    pub kind: String,
    pub value: String,
}

#[napi]
fn highlight_hast(code: String, language: Language) -> HastNode {
    let (config, _, class_names) = load_language(language);
    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(&config, code.as_bytes(), None, |_| None)
        .unwrap();

    let mut stack = Vec::new();
    stack.push(HastNode {
        kind: "element".into(),
        tag_name: "span".into(),
        properties: HastProperties {
            class_name: "source".into(),
        },
        children: Vec::new(),
    });

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::HighlightStart(highlight) => {
                let node = HastNode {
                    kind: "element".into(),
                    tag_name: "span".into(),
                    properties: HastProperties {
                        class_name: class_names[highlight.0].clone(),
                    },
                    children: Vec::new(),
                };
                stack.push(node);
            }
            HighlightEvent::Source { start, end } => {
                let slice = &code[start..end];
                let parent = stack.last_mut().unwrap();
                if let Some(Either::B(text_node)) = parent.children.last_mut() {
                    text_node.value.push_str(slice);
                } else {
                    let text_node = HastTextNode {
                        kind: "text".into(),
                        value: slice.into(),
                    };
                    parent.children.push(Either::B(text_node));
                }
            }
            HighlightEvent::HighlightEnd => {
                let node = stack.pop().unwrap();
                let parent = stack.last_mut().unwrap();
                parent.children.push(Either::A(node));
            }
        }
    }

    stack.pop().unwrap()
}
