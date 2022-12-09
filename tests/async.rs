use dexlib::DexReader;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::sync::Arc;

#[test]
fn test_async() {
    let dex = Arc::new(DexReader::from_file("resources/classes.dex").expect("can't open dex"));
    let types = vec!["Lorg/adw/launcher/Launcher;".to_string()];
    types.par_iter().for_each(|jtype| {
        assert!(dex.find_class_by_name(jtype).is_ok());
    });
}
