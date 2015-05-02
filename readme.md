# pct - parser construction toolkit

pct is a library, written in Rust, that provides the ability to generate and
execute LL(1) parsers.

Pro tip: Don't use this.

It provides functions for the following:

- `cfg::bnf::from_str`: from a string containing a grammar in BNF, create a `Cfg`.
- `cfg::ll1::generate_table`: from a `Cfg`, create the LL(1) parse table. Panics
   if there is a conflict.
- `cfg::ll1::parse`: from a LL(1) parse table and a `Vec<cfg::Token>`, returns
   the list of rules applied to derive the vector. Panics if it can't derive.

Limitations:
------------

- Does not remove left recursion or left factoring. Left recursion _will_ loop forever.
- Panics instead of returning options. You're welcome.
- Probably hella slow [sic].
- The start symbol can only go to one production.

The Input Format:
-----------------

A sample input may be:
```
S A B C
A
A d
B e
C
C f
```

This corresponds to the grammar
```
S → A B C
A → d | ε
B → e
C → f | ε
```

The input to `cfg::bnf::from_str` should be built by the following rules:

- Rules terminated by newlines.
- Upper case letters indicate nonterminals in the grammar.
- Lower case letters indicate terminals in the grammar.
- The first letter of the line must be upper case and is the LHS of the rule.
- The rest of the line is the RHS of the line; if it's empty, it's an epsilon production.
- The first rule is used as the start rule. For multiple start rules, use an augmented grammar.

Notes on the sample grammar:
----------------------------

```
(0) S → A B C
(1) A → ε
(2) A → d ε
(3) B → e
(4) C →  ε
(5) C → f
```

```
FIRST
 A | d, ε
 B | e
 C | f, ε
 S | d, e

FOLLOW
 A | e
 B | F, $
 C | $
 S | $
```

Parse Table:
```
   $  ε  d  e  f
----------------
S:       0  0
A:    1  2  1
B:          3
C: 4  4        5
```
