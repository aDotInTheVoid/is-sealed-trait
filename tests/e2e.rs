use std::path::Path;

use is_sealed_trait::Checker;
use rustdoc_types::{Id, ItemEnum};

fn id_in_root<'a>(checker: &'a Checker, name: &str) -> &'a Id {
    let root = &checker.krate.index[&checker.krate.root].inner;
    let ItemEnum::Module(root_mod) = root else {
        unreachable!()
    };
    for id in &root_mod.items {
        if checker.krate.index[id].name.as_deref() == Some(name) {
            return id;
        }
    }
    panic!("failed to find {name:?} in crate");
}

#[test]
fn simple_public() {
    let checker = checker_for("simple_public");

    assert!(!checker.is_sealed(id_in_root(&checker, "Public")));
}

fn checker_for(arg: &str) -> is_sealed_trait::Checker {
    let json = std::fs::read_to_string(Path::new("tests/corpus").join(arg).with_extension("json"))
        .unwrap();
    let krate = serde_json::from_str(&json).unwrap();

    Checker::new(krate)
}
