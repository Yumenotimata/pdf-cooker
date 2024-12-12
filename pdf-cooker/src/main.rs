mod object;
mod utils;

use object::*;

fn main() {
    let mut obj = Object::new(vec![]);    
    obj.fmap(|r| {
        *r = Some(20);
    });
    let mut reference = obj.as_ref();

    obj.fmap(|r| {
        *r = Some(10);
    });
    
    unsafe {
        println!("{:?}", *reference);
    }
}