// Simplicity "Human-Readable" Language
// Written in 2023 by
//   Andrew Poelstra <simplicity@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Parsing

use santiago::grammar::{Associativity, Grammar};
use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // base combinators
        "DEFAULT" | "UNIT" = string "unit";
        "DEFAULT" | "IDEN" = string "iden";
        "DEFAULT" | "COMP" = string "comp";
        "DEFAULT" | "INJL" = string "injl";
        "DEFAULT" | "INJR" = string "injr";
        "DEFAULT" | "CASE" = string "case";
        "DEFAULT" | "TAKE" = string "take";
        "DEFAULT" | "DROP" = string "drop";
        "DEFAULT" | "PAIR" = string "pair";
        // witness and asserts
        "DEFAULT" | "DISCONNECT" = string "disconnect";
        "DEFAULT" | "WITNESS" = string "witness";
        "DEFAULT" | "ASSERTL" = string "assertl";
        "DEFAULT" | "ASSERTR" = string "assertr";
        // Literals
        "DEFAULT" | "HEXLIT" = pattern r"0x[0-9a-f]+";
        // Type constructors
        "DEFAULT" | "(" = string "(";
        "DEFAULT" | ")" = string ")";
        "DEFAULT" | "+" = string "+";
        "DEFAULT" | "*" = string "*";
        "DEFAULT" | "^" = string "^";
        "DEFAULT" | "->" = string "->";
        "DEFAULT" | "::" = string "::";
        // Assignment
        "DEFAULT" | "=" = string "=";

        // Comments (single-line comments and also (potentially-nested) C-style comments)
        // In all cases we skip over comments entirely in the lexer.
        "DEFAULT" | "LINE_COMMENT" = pattern r"(--|#|//).*\n" => |lexer| lexer.skip();
        "DEFAULT" | "/*" = string "/*" => |lexer| {
            lexer.push_state("C_COMMENT");
            lexer.skip()
        };
        "C_COMMENT" | "/*" = string "/*" => |lexer| {
            lexer.push_state("C_COMMENT");
            lexer.skip()
        };
        "C_COMMENT" | "*/" = string "*/" => |lexer| {
            lexer.pop_state();
            lexer.skip()
        };
        "C_COMMENT" | "C_COMMENT_STAR" = string "*" => |lexer| lexer.skip();
        "C_COMMENT" | "C_COMMENT_SLASH" = string "/" => |lexer| lexer.skip();
        "C_COMMENT" | "C_COMMENT_NON_STAR_SLASH" = pattern r"[^*/]+" => |lexer| lexer.skip();

        // Jets are alphanumerics which start with the prefix jet_
        "DEFAULT" | "JET" = pattern r"jet_[a-zA-Z0-9]+";
        // Basically everything else is fair game as a symbol (which is a placeholder
        // for an expression or type). That is, any string is permissible except one
        // that contains whitespace or any of < > + * # " ^ ( ).
        "DEFAULT" | "SYMBOL" = pattern r#"[^\s<>+*"^()]+"#;

        // Newlines are significant but skip all other whitespace
        "DEFAULT" | "NEWLINE" = pattern r"\n+";
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    )
}

pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
        "program" => empty;
        "program" => rules "line";
        "program" => rules "line" "NEWLINE" "program";

        "line" => rules "symbol" "::" "type";
        "line" => rules "symbol" "=" "expr";
        "line" => rules "symbol" "=" "expr" "::" "type";

        "type" => rules "symbol";
        "type" => rules "(" "type" ")";
        "type" => rules "type" "->" "type";
        "type" => rules "type" "^" "type";
        "type" => rules "type" "+" "type";
        "type" => rules "type" "*" "type";

        "expr" => rules "symbol";
        "expr" => rules "(" "expr" ")";
        "expr" => rules "UNIT";
        "expr" => rules "IDEN" "expr";
        "expr" => rules "COMP" "expr" "expr";
        "expr" => rules "INJL" "expr";
        "expr" => rules "INJR" "expr";
        "expr" => rules "CASE" "expr" "expr";
        "expr" => rules "TAKE" "expr";
        "expr" => rules "DROP" "expr";
        "expr" => rules "PAIR" "expr" "expr";

        "expr" => rules "ASSERTL" "expr";
        "expr" => rules "ASSERTR" "expr";
        "expr" => rules "WITNESS" "expr";
        "expr" => rules "DISCONNECT" "expr" "expr";

        "expr" => rules "JET";

        // Map rules to their corresponding Lexemes
        "UNIT" => lexemes "UNIT";
        "IDEN" => lexemes "IDEN";
        "COMP" => lexemes "COMP";
        "INJL" => lexemes "INJL";
        "INJR" => lexemes "INJR";
        "CASE" => lexemes "CASE";
        "TAKE" => lexemes "TAKE";
        "DROP" => lexemes "DROP";
        "PAIR" => lexemes "PAIR";
        "ASSERTL" => lexemes "ASSERTL";
        "ASSERTR" => lexemes "ASSERTR";
        "WITNESS" => lexemes "WITNESS";
        "DISCONNECT" => lexemes "DISCONNECT";
        "JET" => lexemes "JET";
        "(" => lexemes "(";
        ")" => lexemes ")";
        "+" => lexemes "+";
        "*" => lexemes "*";
        "^" => lexemes "^";
        "::" => lexemes "::";
        "->" => lexemes "->";
        "=" => lexemes "=";
        "NEWLINE" => lexemes "NEWLINE";
        "symbol" => lexemes "SYMBOL";

        // Define associativity rules for type constructors
        Associativity::Left => rules "*" "+";
        Associativity::Right => rules "^" "->";

    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let input = "
        sig :: 2^512
pk :: 2^256
sigHash :: 1 -> 2^256

-- 0, 1, etc can be arbitrary printable-ascii names except they can't have `=` or `:`
0 = witness sig
1 = witness pk
/* lol

/* lol */still in comment */2 = jet_checkSigHashAll 0 1
3 = disconnect 2 sighash
        10 + 20 + 30";

        let input = "sig :: 2
pk :: 2^256
sigHash :: 1 -> 2^256
        ";
        let lexer_rules = lexer_rules();
        let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
        for lex in &lexemes {
            println!("{}", lex);
        }


let grammar = grammar();
let parse_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
for tree in parse_trees {
            println!("{}", tree);
}
//        panic!("Deliberate panic to make it easy to see output");
    }
}

