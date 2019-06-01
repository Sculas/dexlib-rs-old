use std::fs::File;
use std::io;

use dex_parser::DexBuilder;

fn main() -> io::Result<()> {
    let dex = DexBuilder::from_file(&File::open("resources/classes.dex")?).unwrap();
    for _class in dex.classes() {
        //println!("class: {:?}", class.unwrap());
    }
    Ok(())
}
