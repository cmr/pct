use cfg;
use cfg::EPSILON;
use std::collections::HashMap;

pub fn from_str(bnf: &str) -> cfg::Cfg<cfg::Mutable> {
    let mut c = cfg::Cfg::new();

    let mut terms = HashMap::new();
    let mut nonterms = HashMap::new();

    for line in bnf.lines() {
        let line = &line.chars().filter(|c| !c.is_whitespace()).collect::<String>()[..];
        //println!("{}", line);
        let mut chars = line.chars();
        if let Some(fst) = chars.next() {
            nonterms.entry(fst).or_insert_with(|| c.add_nonterminal());
            let mut seen_any = false;
            let mut syms = Vec::new();

            for sym in chars {
                seen_any = true;
                match sym.is_uppercase() {
                    true => {
                        nonterms.entry(sym).or_insert_with(||c.add_nonterminal());
                        syms.push(*nonterms.get(&sym).unwrap());
                    },
                    false => {
                        terms.entry(sym).or_insert_with(|| c.add_terminal());
                        syms.push(*terms.get(&sym).unwrap());
                    }
                }
            }
            if !seen_any {
                c.add_rule(nonterms.get(&fst).unwrap(), &[EPSILON]);
            } else {
                c.add_rule(nonterms.get(&fst).unwrap(), &syms[..]);
            }
        }
    }

    c
}
