use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("test.md")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let site = oranda::create_html(&data);

    let mut html_file = File::create("public/index.html")?;
    html_file.write_all(site.html.as_bytes())?;

    let mut css_file = File::create("public/styles.css")?;
    css_file.write_all(site.css.as_bytes())?;

    Ok(())
}
