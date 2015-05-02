use cfg;
use std::collections::HashMap;

pub fn from_str(bnf: &str) -> cfg::Cfg<cfg::Mutable> {
    let mut cfg = cfg::Cfg::new();

    let mut terms = HashMap::new();
    let mut nonterms = HashMap::new();

    for line in bnf.lines() {
        println!("{}", line);
    }

    cfg
}

#[test]
fn test_from_str() {
    
}
