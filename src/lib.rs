use std::{collections::BTreeMap, sync::Arc};

use rustdoc_types::*;
use trustfall::FieldValue;
use trustfall_rustdoc_adapter::{IndexedCrate, RustdocAdapter};

/*



if the trait is pub-in-priv and not publicly re-exported
    - Easy
if the trait has a sealed supertrait (sealed in any way, not just pub-in-priv) that is defined in the same crate
    - Easy, unless it's cross-crarte
    - But cross-crate needs redoing anyway, so let's punt
if the trait has a "sealed method" without a default impl, in any of the ways that allow sealing a method:
    a pub-in-priv type as a parameter or return type (described here)
        - Doable??
    a sealed trait from the same crate used as a bound (described here)
if the trait has an associated type with a sealed trait from the same crate as a bound
if the trait has an associated const of a pub-in-priv type that doesn't have a default

*/

pub struct Checker<'a> {
    pub krate: &'a Crate,
    indexed: IndexedCrate<'a>,
    root_crate_id: u32,
}

impl<'a> Checker<'a> {
    pub fn new(krate: &'a Crate) -> Self {
        let root_crate_id = krate.index[&krate.root].crate_id;

        let indexed = IndexedCrate::new(&krate);

        Self {
            krate,
            indexed,
            root_crate_id,
        }
    }

    pub fn is_sealed(&self, id: &Id) -> bool {
        let the_trait = self.load_trait(id);

        // if the trait is pub-in-priv and not publicly re-exported
        if self.is_pub_in_priv(id) {
            return true;
        }

        // Sealed supertrait
        for bound in &the_trait.bounds {
            if let GenericBound::TraitBound {
                trait_: supertrait,
                // These are HRTB's which are only lifetimes, so don't care.
                generic_params: hrtbs,
                modifier,
            } = bound
            {
                assert_eq!(*modifier, TraitBoundModifier::None);

                for hrtb in hrtbs {
                    let GenericParamDefKind::Lifetime { .. } = hrtb.kind else {
                        panic!("unexpected non-lifetime HRTB {hrtb:?}")
                    };
                }

                if self.is_sealed(&supertrait.id) {
                    return true;
                }
            }
        }

        false
    }

    fn is_pub_in_priv(&self, id: &Id) -> bool {
        // TODO: Maybe just do this ourself, instead of with trustfall
        // Alternativly: expose rustc EffectiveVisibilities

        let is_pub = self.krate.index[id].visibility == Visibility::Public;

        if !is_pub {
            return false;
        }

        let schema = RustdocAdapter::schema();
        let adapter = Arc::new(RustdocAdapter::new(&self.indexed, None));

        let has_importable_path = trustfall::execute_query(
            &schema,
            adapter,
            r###"
        query {
            Crate {
              item {
                ... on Trait {
                  id @filter(op: "=", value: ["$id"])
          
                  importable_path @fold @transform(op: "count") @filter(op: ">", value: ["$zero"])
                }
              }
            }
          }
          

        "###,
            BTreeMap::<&str, FieldValue>::from_iter([
                ("id", id.0.as_str().into()),
                ("zero", 0.into()),
            ]),
        )
        .unwrap()
        .next()
        .is_some();

        !has_importable_path
    }

    fn is_other_crate(&self, id: &Id) -> bool {
        self.krate.index[id].crate_id == self.root_crate_id
    }

    fn load_trait(&self, id: &Id) -> &Trait {
        match &self.krate.index[id].inner {
            ItemEnum::Trait(t) => t,
            other => panic!("expected a trait, but got {other:?}"),
        }
    }
}
