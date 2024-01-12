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
    assert!(!is_sealed(&checker, "Public"));
}

#[test]
fn pub_in_priv_supertrait() {
    let checker = checker_for("pub_in_priv_supertrait");
    assert!(is_sealed(&checker, "Sealed"));
}

fn checker_for(arg: &str) -> is_sealed_trait::Checker<'static> {
    let json = std::fs::read_to_string(Path::new("tests/corpus").join(arg).with_extension("json"))
        .unwrap();
    let krate = serde_json::from_str(&json).unwrap();

    let b = Box::new(krate);
    Checker::new(Box::leak(b))
}

fn is_sealed(checker: &Checker, name: &str) -> bool {
    checker.is_sealed(id_in_root(checker, name))
}
