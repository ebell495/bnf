#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

extern crate bnf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Pulled from https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form#Further_examples
        // and modifed so that <EOL> evaluates to ';'
        let bnf_for_bnf: &str =
            "<syntax>        ::= <rule> | <rule> <syntax>
            <rule>           ::= <opt-whitespace> \"<\" <rule-name> \">\" 
                                <opt-whitespace> \"::=\" <opt-whitespace> 
                                <expression> <line-end>
            <opt-whitespace> ::= \" \" <opt-whitespace> | \"\"
            <expression>     ::= <list> | <list> <opt-whitespace> \"|\" 
                                <opt-whitespace> <expression>
            <line-end>       ::= <opt-whitespace> <EOL> | <line-end> <line-end>
            <list>           ::= <term> | <term> <opt-whitespace> <list>
            <term>           ::= <literal> | \"<\" <rule-name> \">\"
            <literal>        ::= '\"' <text1> '\"' | \"'\" <text2> \"'\"
            <text1>          ::= \"\" | <character1> <text1>
            <text2>          ::= \"\" | <character2> <text2>
            <character>      ::= <letter> | <digit> | <symbol>
            <letter>         ::= \"A\" | \"B\" | \"C\" | \"D\" | \"E\" | \"F\" 
                                | \"G\" | \"H\" | \"I\" | \"J\" | \"K\" | \"L\" 
                                | \"M\" | \"N\" | \"O\" | \"P\" | \"Q\" | \"R\" 
                                | \"S\" | \"T\" | \"U\" | \"V\" | \"W\" | \"X\" 
                                | \"Y\" | \"Z\" | \"a\" | \"b\" | \"c\" | \"d\" 
                                | \"e\" | \"f\" | \"g\" | \"h\" | \"i\" | \"j\" 
                                | \"k\" | \"l\" | \"m\" | \"n\" | \"o\" | \"p\" 
                                | \"q\" | \"r\" | \"s\" | \"t\" | \"u\" | \"v\" 
                                | \"w\" | \"x\" | \"y\" | \"z\"

            <digit>          ::= \"0\" | \"1\" | \"2\" | \"3\" | \"4\" | \"5\" 
                                | \"6\" | \"7\" | \"8\" | \"9\"

            <symbol>         ::=  \"|\" | \" \" | \"-\" | \"!\" | \"#\" | \"$\" 
                                | \"%\" | \"&\" | \"(\" | \")\" | \"*\" | \"+\" 
                                | \",\" | \"-\" | \".\" | \"/\" | \":\" | ";" 
                                |\">\" | \"=\" | \"<\" | \"?\" | \"@\" | \"[\" 
                                | \"\\\" | \"]\" | \"^\" | \"_\" | \"`\" 
                                | \"{{\" | \"}}\" | \"~\"

            <character1>     ::= <character> | \"'\"
            <character2>     ::= <character> | '\"'
            <rule-name>      ::= <letter> | <rule-name> <rule-char>
            <rule-char>      ::= <letter> | <digit> | \"-\"
            <EOL>            ::= \";\"";

            let grammar = bnf::parse(bnf_for_bnf);
            let sentence = grammar.generate();
            let meta_grammar = bnf::parse(&sentence);

            assert!(meta_grammar.to_string().len() != 0)
            
    }    
}