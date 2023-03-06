use markup5ever_rcdom::{Handle, NodeData};

use crate::common::{build_dom, get_page};

pub fn collect_links() {
    let wiki_random_japanese_article_url = "https://ja.wikipedia.org/wiki/%E7%89%B9%E5%88%A5:%E3%81%8A%E3%81%BE%E3%81%8B%E3%81%9B%E8%A1%A8%E7%A4%BA";
    let page = get_page(wiki_random_japanese_article_url).unwrap();
    let dom = build_dom(page);
    let mut links = Vec::new();
    collect_links_walk(&dom.document, &mut links);
    for link in links {
        println!("{link}");
    }
}

fn collect_links_walk(handle: &Handle, links: &mut Vec<String>) {
    let mut after_walk = String::new();
    let node = handle;
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => match name.local {
            local_name!("a") => {
                if let Some(href) = attrs
                    .borrow()
                    .iter()
                    .find(|attr| attr.name.local == local_name!("href"))
                {
                    return links.push(href.value.to_string());
                }
            }
            _ => (),
        },
        NodeData::ProcessingInstruction { .. } => unreachable!(),
        _ => (),
    }
    for child in node.children.borrow().iter() {
        collect_links_walk(child, links);
    }
}
