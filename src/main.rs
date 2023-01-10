use nipper::Document;

fn main() -> Result<(), ureq::Error> {
	let wiki = "https://ja.wikipedia.org/wiki/%E7%89%B9%E5%88%A5:%E3%81%8A%E3%81%BE%E3%81%8B%E3%81%9B%E8%A1%A8%E7%A4%BA";
	let url = "https://github.com/algesten/ureq/raw/main/README.md";
    let html: String = ureq::get(wiki)
        .call()?
        .into_string()?;

	let document = Document::from(&html);
	let class = ".mw-parser-output";
	
	let main_content = document.select(class).iter().next().unwrap();
	let text = main_content.text();
	for line in main_content.html().lines() {
		println!("{line}");
	}
    Ok(())
}
