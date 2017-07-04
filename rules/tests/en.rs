extern crate rustling;
extern crate rustling_ontology_rules as rules;
extern crate rustling_ontology_values as values;

use values::Dimension;
use values::check::*;

use rustling::Check;
use rules::PreprocessingOptions;

fn run(rules: &rustling::RuleSet<Dimension, PreprocessingOptions>, sentence: &str) -> Vec<rustling::ParsedNode<Dimension>> {
    rules.apply_all(&sentence.to_lowercase()).unwrap()
}

#[test]
fn ex_100k() {
    let en = rules::en_config::rule_set().unwrap();
    assert!(run(&en, "100k").into_iter().any(|it| check_integer(100000).check(&it)));
    assert!(!run(&en, "100kilo").into_iter().any(|it| check_integer(100000).check(&it)));
    assert!(run(&en, "100k$").into_iter().any(|it| check_integer(100000).check(&it)));
    assert!(!run(&en, "toto100k").into_iter().any(|it| check_integer(100000).check(&it)));
}

