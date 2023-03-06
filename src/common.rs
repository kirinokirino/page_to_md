use std::io::BufReader;
use std::{default::Default, io::Read};

use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::RcDom;

pub fn build_dom(page: Box<dyn Read>) -> RcDom {
    let mut page = BufReader::new(page);
    let options = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            scripting_enabled: false,
            exact_errors: true,
            ..Default::default()
        },
        tokenizer: TokenizerOpts {
            exact_errors: true,
            ..Default::default()
        },
    };
    let dom = parse_document(RcDom::default(), options)
        .from_utf8()
        .read_from(&mut page)
        .unwrap();
    dom
}

pub fn get_page(url: &str) -> Result<Box<dyn Read + Send + Sync>, ureq::Error> {
    let html = ureq::get(url).call()?.into_reader();
    Ok(html)
}

pub fn collapse_whitespace(string: String) -> Option<String> {
    let mut last_was_whitespace = false;
    let result = string
        .chars()
        .filter_map(|ch| {
            if ch.is_whitespace() && last_was_whitespace {
                last_was_whitespace = true;
                None
            } else if ch.is_whitespace() {
                last_was_whitespace = true;
                Some(' ')
            } else {
                last_was_whitespace = false;
                Some(ch)
            }
        })
        .collect();
    if &result == " " {
        None
    } else {
        Some(result)
    }
}
