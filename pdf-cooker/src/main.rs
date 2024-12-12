mod object;

use object::*;
// use std::pin::Pin;
fn main() {
    let mut entity = Object::new(vec![]);
    let mut reference = entity.as_ref();
    // entity.fmap(|pin| {
    //     unsafe {
    //         let this = pin.get_unchecked_mut();
    //         this.number = Some(10);
    //     }
    // });
    // a.fmap(|pin| {
    //     unsafe {
    //         let this = pin.get_unchecked_mut();
    //         this.number = Some(120);
    //     }
    // });
    reference.fmap(|raw| {
        println!("{:?}", raw);
    });

    println!("{:?}", entity);
}