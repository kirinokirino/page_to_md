use markup5ever_rcdom::{Handle, NodeData};
use std::string::String;

use crate::common::{build_dom, collapse_whitespace, get_page};

pub fn wiki() {
    let wiki_random_japanese_article_url = "https://ja.wikipedia.org/wiki/%E7%89%B9%E5%88%A5:%E3%81%8A%E3%81%BE%E3%81%8B%E3%81%9B%E8%A1%A8%E7%A4%BA";
    let page = get_page(wiki_random_japanese_article_url).unwrap();
    let dom = build_dom(page);

    let mut before = String::new();
    let mut after: Vec<(usize, String)> = Vec::new();
    walk_wiki(0, &mut before, &mut after, &dom.document);
    println!("{before}");
    if !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.iter() {
            println!("    {}", err);
        }
    }
}

fn walk_wiki(
    indent: usize,
    before: &mut String,
    after: &mut Vec<(usize, String)>,
    handle: &Handle,
) {
    let mut after_walk = String::new();
    let node = handle;
    match node.data {
        NodeData::Document => (),
        NodeData::Doctype { .. } => (),
        NodeData::Comment { .. } => (),

        NodeData::Text { ref contents } => {
            let text = collapse_whitespace(contents.borrow().to_string());
            if let Some(text) = text {
                before.push_str(&text);
            }
        }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => match name.local {
            local_name!("style") | local_name!("script") => return,
            local_name!("p") => {
                before.push_str("\n\n");
                after_walk.push_str("\n\n")
            }
            local_name!("br") => before.push('\n'),
            local_name!("h1") => before.push_str("\n\n# "),
            local_name!("h2") => before.push_str("\n\n## "),
            local_name!("h3") => before.push_str("\n\n### "),
            local_name!("h4") => before.push_str("\n\n#### "),
            local_name!("h5") => before.push_str("\n\n##### "),
            local_name!("h6") => before.push_str("\n\n###### "),
            local_name!("blockquote") => before.push_str("\n\n> "),
            local_name!("li") => before.push_str("\n - "),
            local_name!("hr") => before.push_str("\n\n"),
            local_name!("em") | local_name!("i") => {
                before.push('_');
                after_walk.push('_')
            }
            local_name!("b") | local_name!("strong") => {
                before.push_str("**");
                after_walk.push_str("**")
            }
            local_name!("img") => {
                let attrs = attrs.borrow();
                let alt = attrs
                    .iter()
                    .find(|attr| attr.name.local == local_name!("alt"));
                let src = attrs
                    .iter()
                    .find(|attr| attr.name.local == local_name!("src"));
                if alt.is_some() && src.is_some() {
                    before.push_str(&format!(
                        "![{}]({})",
                        alt.unwrap().value,
                        src.unwrap().value
                    ));
                }
            }
            local_name!("a") => {
                if let Some(href) = attrs
                    .borrow()
                    .iter()
                    .find(|attr| attr.name.local == local_name!("href"))
                {
                    before.push('[');
                    after_walk.push_str(&format!("]({})", href.value));
                }
            }
            _ => (),
        },

        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }
    after.push((indent, after_walk));
    for child in node.children.borrow().iter() {
        walk_wiki(indent + 1, before, after, child);
        while after.last().is_some() && after.last().unwrap().0 > indent {
            let (_, text) = after.pop().unwrap();
            before.push_str(&text);
        }
    }
}
