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
    // let mut page: Vec<Object> = page.into();
    page.resource.add_font("Times");

    // page.iter_mut().for_each(|obj: &mut Object| obj.project().iter_mut().for_each(|p: &mut Primitive| println!("{:?}", p)));

    let mut doc = Document::new();
    doc.appendix(page);
    doc.encode();
    // let pages: Vec<&Object> = page
    //     .iter()
    //     .filter(|obj| obj.project_ref().iter().any(|prim| prim.is_type("Page")))
    //     .collect();

    // println!("{:#?}", page);
}