use std::{
    fmt::{Debug, Display, Write},
    io,
};

const PAGE: &str = include_str!("../wikipage.html");
const DOCTYPE: &str = "<!DOCTYPE html>";
fn main() {
    let page = PAGE; //get_wiki_page().unwrap();
    let mut tree = FilteredTree::new();
    let mut parser = TagParser::new();
    for (i, ch) in page.chars().skip(DOCTYPE.len()).enumerate() {
        parser.feed_ch(ch);
    }
    for tag in parser.tags {
        tree.feed_tag(tag);
    }
    //tree.page.finish();
    println!("{}", tree.page);
}

struct WikiPage {
    pub title: Option<String>,
    pub tags: Vec<Tag>,
}

impl Display for WikiPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(title) = &self.title {
            f.write_str(title);
            f.write_char('\n');
        }
        for tag in &self.tags {
            if let Some(content) = &tag.content {
                f.write_fmt(format_args!("{}", content));
            }
        }
        Ok(())
    }
}

impl Debug for WikiPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(title) = &self.title {
            f.write_str(title);
            f.write_char('\n');
        }
        for tag in &self.tags {
            if tag.content.is_none() {
                continue;
            }
            let items = if tag.items.is_empty() {
                "".to_string()
            } else {
                format!(" |{:?}|", tag.items)
            };
            let content = if tag.content.is_none() {
                "".to_string()
            } else {
                format!(":{}", tag.content.clone().unwrap())
            };
            let padding = if tag.door == TagDoor::Closing {
                " /".to_string()
            } else {
                format!("\n{}", " ".repeat(tag.depth))
            };
            f.write_fmt(format_args!("{}{}{}{}", padding, tag.name, items, content));
        }
        Ok(())
    }
}

impl WikiPage {
    fn new() -> Self {
        Self {
            title: None,
            tags: Vec::new(),
        }
    }

    fn finish(&mut self) {
        let mut iter = std::mem::take(&mut self.tags).into_iter();
        iter.find(|el| el.content == Some("From Wikipedia, the free encyclopedia".to_string()));

        self.tags = iter.skip(2).collect();
    }
}

#[derive(Debug)]
struct FilteredTree {
    tags: Vec<Tag>,
    is_in_main: bool,
    is_at_footer: bool,
    page: WikiPage,
    depth: usize,
}

const IGNORE_TAGS: [&str; 18] = [
    "meta",
    "script",
    "style",
    "link",
    "img",
    "input",
    "!--",
    "br",
    "noscript",
    "!--esi",
    "esi:include",
    "sup", //table
    "table",
    "tbody",
    "td",
    "tr",
    "th",
    "thead",
];

impl FilteredTree {
    pub fn new() -> Self {
        Self {
            tags: Vec::new(),
            is_in_main: false,
            is_at_footer: false,
            page: WikiPage::new(),
            depth: 0,
        }
    }

    pub fn feed_tag(&mut self, mut tag: Tag) {
        if !IGNORE_TAGS.contains(&tag.name.as_str()) && !self.is_at_footer {
            if tag.door == TagDoor::Closing {
                self.depth -= 1;
            } else if tag.door == TagDoor::Opening {
                self.depth += 1;
            }
            if tag.name == "main" {
                self.is_in_main = true;
                self.tags = Vec::new();
            }
            if tag.name == "footer" && self.is_in_main {
                self.is_at_footer = true;
                self.is_in_main = false;
                return;
            }
            if tag.name == "title" {
                self.page.title = tag.content;
                return;
            }
            if self.is_in_main {
                tag.depth = self.depth;
                tag.content = clean_whitespace(tag.content);
                self.page.tags.push(tag);
            }
        }
    }
}

fn clean_whitespace(content: Option<String>) -> Option<String> {
    if let Some(content) = content {
        let mut last_char_was_whitespace = false;
        let mut only_whitespace = true;
        let content = content
            .chars()
            .filter_map(|ch| {
                if ch.is_whitespace() && last_char_was_whitespace {
                    last_char_was_whitespace = true;
                    None
                } else if ch.is_whitespace() {
                    last_char_was_whitespace = true;
                    Some(' ')
                } else {
                    only_whitespace = false;
                    last_char_was_whitespace = false;
                    Some(ch)
                }
            })
            .collect();
        return if only_whitespace { None } else { Some(content) };
    }
    None
}

fn wait(tree: &FilteredTree) {
    let mut buf = String::new();
    if let Err(err) = io::stdin().read_line(&mut buf) {
        eprintln!("Wait error: {err}");
    };
    match buf.trim() {
        "tree" => {
            println!("\n\n{tree:?}\n\n")
        }
        "page" => {
            println!("\n\n{:?}\n\n", tree.page)
        }
        _ => (),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TagDoor {
    Opening,
    Closing,
    SelfClosing,
}

#[derive(Debug, Clone)]
struct Tag {
    name: String,
    door: TagDoor,
    items: Vec<String>,
    pub content: Option<String>,
    depth: usize,
}

impl Tag {
    fn new(name: String, door: TagDoor, items: Vec<String>, content: Option<String>) -> Self {
        Self {
            name,
            door,
            items,
            content,
            depth: 0,
        }
    }
}

#[derive(Debug)]
struct TagParser {
    buffer: String,
    tags: Vec<Tag>,
}
impl TagParser {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            tags: Vec::new(),
        }
    }
    fn parse_tag(&mut self, mut contents: String) {
        let door = if contents.starts_with("</") {
            contents.remove(0);
            contents.remove(0);
            TagDoor::Closing
        } else if contents.ends_with("/") {
            contents.pop();
            contents.remove(0);
            TagDoor::SelfClosing
        } else {
            contents.remove(0);
            TagDoor::Opening
        };
        if door == TagDoor::Closing {
            if let Some(tag) = self
                .tags
                .iter_mut()
                .rev()
                .find(|tag| tag.door == TagDoor::Opening)
            {
                tag.content = Some(std::mem::take(&mut self.buffer));
            }
        }
        let mut iter = contents.split_ascii_whitespace();
        let name = iter.next().unwrap().to_string();
        let items: Vec<String> = iter.map(|s| s.to_string()).collect();
        let tag = Tag::new(name, door, items, Some(std::mem::take(&mut self.buffer)));
        self.tags.push(tag);
    }
    fn try_finish_tag(&mut self) {
        if let Some((i, _)) = &self.buffer.char_indices().rev().find(|(i, ch)| ch == &'<') {
            let tag = self.buffer.split_off(*i);
            self.parse_tag(tag);
        }
    }

    pub fn feed_ch(&mut self, ch: char) {
        if ch == '>' {
            self.try_finish_tag();
        } else {
            self.buffer.push(ch);
        }
    }
}

fn get_wiki_page() -> Result<String, ureq::Error> {
    let wiki = "https://ja.wikipedia.org/wiki/%E7%89%B9%E5%88%A5:%E3%81%8A%E3%81%BE%E3%81%8B%E3%81%9B%E8%A1%A8%E7%A4%BA";
    let url = wiki; //"https://github.com/algesten/ureq/raw/main/README.md";
    let html: String = ureq::get(wiki).call()?.into_string()?;

    Ok(html)
}
