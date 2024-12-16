mod prim;
mod object;
mod utils;
mod document;
mod page;

use prim::*;
use object::*;
use utils::*;
use document::*;
use page::*;

fn main() {
    let obj = map![("a", array![2, "a", Prim::defer(1 as *const RawObject)]), ("a", array![2, "a", Prim::defer(1 as *const RawObject)])];

    println!("{:#?}", obj);
}