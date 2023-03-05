use std::default::Default;
use std::fs::File;
use std::io::BufReader;
use std::iter::repeat;
use std::string::String;

#[macro_use]
extern crate html5ever;

use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

const PAGE: &str = include_str!("../wikipage.html");
fn main() {
    let page = File::open("./wikipage.html").unwrap(); //get_wiki_page().unwrap();
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

    walk(0, &dom.document);

    if !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.iter() {
            println!("    {}", err);
        }
    }
}

fn get_wiki_page() -> Result<String, ureq::Error> {
    let wiki = "https://ja.wikipedia.org/wiki/%E7%89%B9%E5%88%A5:%E3%81%8A%E3%81%BE%E3%81%8B%E3%81%9B%E8%A1%A8%E7%A4%BA";
    let url = wiki; //"https://github.com/algesten/ureq/raw/main/README.md";
    let html: String = ureq::get(wiki).call()?.into_string()?;

    Ok(html)
}

fn walk(indent: usize, handle: &Handle) {
    let node = handle;
    // FIXME: don't allocate
    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.data {
        NodeData::Document => (),
        NodeData::Doctype { .. } => (),
        NodeData::Comment { .. } => (),

        NodeData::Text { ref contents } => {
            let text = collapse_whitespace(contents.borrow().to_string());
            if let Some(text) = text {
                println!("\"{text}\"")
            }
        }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }

        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(indent + 1, child);
    }
}

fn collapse_whitespace(string: String) -> Option<String> {
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
