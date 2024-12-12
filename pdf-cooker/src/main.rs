mod object;
mod utils;
mod prim;
mod document;

use document::MediaBox;
use object::*;
use document::*;

fn main() {
    let mut doc = Document::new();
    let mut page = Page::new(MediaBox::A4);

    doc.appendix(page);

    doc.resolve();

    println!("{:#?}", doc);
}