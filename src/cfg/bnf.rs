use cfg;
use cfg::EPSILON;
use std::collections::HashMap;

pub struct BNFName;
impl ::typemap::Key for BNFName { type Value = HashMap<cfg::Symbol, char>; }



pub fn from_str(bnf: &str) -> cfg::Cfg<cfg::Mutable> {
    let mut c = cfg::Cfg::new();

    let mut terms = HashMap::new();
    let mut nonterms = HashMap::new();
    let mut names = HashMap::new();
    let mut first_line = true;

    for line in bnf.lines() {
        let line = &line.chars().filter(|c| !c.is_whitespace()).collect::<String>()[..];

        let mut chars = line.chars();
        if let Some(fst) = chars.next() {
            nonterms.entry(fst).or_insert_with(|| c.add_nonterminal());
            names.insert(*nonterms.get(&fst).unwrap(), fst);
            let mut seen_any = false;
            let mut syms = Vec::new();

            for sym in chars {
                seen_any = true;
                match sym.is_uppercase() {
                    true => {
                        nonterms.entry(sym).or_insert_with(||c.add_nonterminal());
                        names.insert(*nonterms.get(&sym).unwrap(), sym);
                        syms.push(*nonterms.get(&sym).unwrap());
                    },
                    false => {
                        terms.entry(sym).or_insert_with(|| c.add_terminal());
                        names.insert(*terms.get(&sym).unwrap(), sym);
                        syms.push(*terms.get(&sym).unwrap());
                    }
                }
            }
            let rule = if !seen_any {
                c.add_rule(nonterms.get(&fst).unwrap(), &[EPSILON])
            } else {
                c.add_rule(nonterms.get(&fst).unwrap(), &syms[..])
            };

            if first_line {
                c.set_start(rule);
            }
        }
        first_line = false;
    }

    c.mut_extra().insert::<BNFName>(names);

    c
}

pub fn to_string<T>(c: &cfg::Cfg<T>) -> String {
    let names = c.extra.get::<BNFName>().unwrap();

    c.rules().map(|&(lhs, ref rhs)| {
            let from = names.get(&lhs.into()).unwrap();
            let to = rhs.iter()
                .map(|rule| *names.get(&rule.into()).unwrap_or(&'ε'))
                .collect::<String>();
            format!("{} -> {}", from, to)
    }).collect::<Vec<String>>().connect("\n")
}

pub fn print_bnf<T>(c: &cfg::Cfg<T>) {
    println!("{}", to_string(c));
}
