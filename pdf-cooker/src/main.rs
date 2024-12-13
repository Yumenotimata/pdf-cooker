mod object;
mod utils;
mod prim;
mod document;
mod page;

use document::*;
use object::*;
use document::*;
use prim::*;
use page::*;

fn main() {
    let mut doc = Document::new();
    let mut page = Page::new(MediaBox::A4);

    doc.appendix(page);

    doc.resolve();

    println!("{:#?}", doc);
    // let mediabox = Primitive::Array(vec![Primitive::Number(1), Primitive::Number(2)]);
    // println!("{}", mediabox);
}