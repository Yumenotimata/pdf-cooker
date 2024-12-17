#[macro_use] mod prim;
mod object;
mod document;
mod page;
mod fix;

use prim::*;
use object::*;
use document::*;
use page::*;

fn main() {
    let mut page = Page::new(MediaBox::A4);
    let mut page: Vec<Object> = page.into();

    println!("{:#?}", page);
}