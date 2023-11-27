use core::fmt::Debug;
use log::{debug, trace};
use thiserror::Error;
use crate::bytes::CharIterator;
use crate::css3::Error::{Syntax, UnexpectedEof};
use crate::css3::new_tokenizer::{Token, Tokenizer};

mod new_tokenizer;
mod unicode;


#[derive(Error, Debug)]
pub enum Error {
    #[error("syntax error: {0}")]
    Syntax(String),
    #[error("unexpected end of stream")]
    UnexpectedEof,
}

// =================================================================================================

pub struct CSS3ParserTng<'stream> {
    tokenizer: Tokenizer<'stream>,
}

#[derive(Default)]
pub struct QualifiedRule {
    prelude: Vec<ComponentValue>,
    block: Option<SimpleBlock>,
}

impl Debug for QualifiedRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QualifiedRule {{ prelude: {:?}, block: {:?} }}", self.prelude, self.block)
    }
}

#[derive(Default)]
pub struct Declaration {
    name: String,
    value: Vec<ComponentValue>,
    important: bool,
}

pub enum DeclarationAndAtRules {
    Declaration(Declaration),
    AtRule(AtRule),
}

#[derive(PartialEq)]
pub enum ComponentValue {
    PreservedToken(Token),
    Function(Function),
    SimpleBlock(SimpleBlock)
}

impl Debug for ComponentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentValue::PreservedToken(token) => write!(f, "{:?}", token),
            ComponentValue::Function(function) => write!(f, "{:?}", function),
            ComponentValue::SimpleBlock(block) => write!(f, "{:?}", block),
        }
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct Function {
    name: String,
    values: Vec<ComponentValue>,
}

#[derive(PartialEq, Debug)]
pub struct SimpleBlock {
    associated_token: Token,
    values: Vec<ComponentValue>,
}

impl SimpleBlock {
    fn new(associated_token: Token) -> SimpleBlock {
        SimpleBlock {
            associated_token,
            values: Vec::new(),
        }
    }
}

pub enum Rule {
    NormRule(NormRule),
    AtRule(AtRule),
    QualifiedRule(QualifiedRule),
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::NormRule(rule) => write!(f, "\n{:?}", rule),
            Rule::AtRule(rule) => write!(f, "\n{:?}", rule),
            Rule::QualifiedRule(rule) => write!(f, "\n{:?}", rule),
        }
    }
}

#[derive(Default, Debug)]
pub struct AtRule {
    name: String,
    prelude: Vec<ComponentValue>,
    block: Option<SimpleBlock>
}

#[derive(Debug)]
pub struct NormRule {
    prelude: Vec<ComponentValue>,
    block: Option<SimpleBlock>,
}

#[derive(Default, Debug)]
pub struct Stylesheet {
    location: Option<String>,
    rules: Vec<Rule>,
}


impl<'stream> CSS3ParserTng<'stream> {
    pub fn new(tokenizer: Tokenizer) -> CSS3ParserTng {
        CSS3ParserTng { tokenizer }
    }

    pub fn from_input_stream(ci: &mut CharIterator) -> CSS3ParserTng {
        CSS3ParserTng::new(Tokenizer::new(ci))
    }

    // =============================================================================================
    // These are the public parse_* functions

    pub fn parse(&mut self, _grammar: String) -> Result<Vec<ComponentValue>, Error>
    {
        debug!("parse()");
        let _result = self.parse_list_of_component_values();

        // @todo: match grammar against result // !????
        return Err(Syntax("not implemented yet".to_string()));
    }

    pub fn parse_comma_separated_list(&mut self, _grammar: String) -> Result<Vec<ComponentValue>, Error>
    {
        debug!("parse_comma_separated_list()");

        let mut retvals = Vec::new();

        let result_list = self.parse_commaseparated_list_of_component_values();
        for result in result_list {
            // @todo: match grammar against result // !????
            // if matches against grammar {
            //     retvals.push(result);
            // }
            retvals.push(result)
        }

        Ok(retvals)
    }

    // This will parse a complete stylesheet, which isn't much more than a list of rules
    pub fn parse_stylesheet(&mut self, location: Option<String>) -> Result<Stylesheet, Error> {
        debug!("parse_stylesheet({:?})", location);

        let mut stylesheet = Stylesheet::default();
        stylesheet.location = location;
        stylesheet.rules = self.consume_list_of_rules(true);

        Ok(stylesheet)
    }

    /// This will return a list of rules found in the stream
    pub fn parse_list_of_rules(&mut self) -> Vec<Rule> {
        self.consume_list_of_rules(false)
    }

    /// When parsing a rule, the stream must return an EOF at the end of that rule.
    pub fn parse_rule(&mut self) -> Result<Rule, Error> {
        let rule;

        self.consume_whitespaces();

        match self.tokenizer.lookahead(0) {
            Token::EOF => {
                return Err(Syntax("unexpected eof".to_string()));
            }
            Token::AtKeyword(_) => {
                match self.consume_at_rule() {
                    Some(at_rule) => {
                        rule = Some(Rule::AtRule(at_rule));
                    }
                    None => {
                        return Err(Syntax("syntax error".to_string()));
                    }
                }
            }
            _ => {
                rule = match self.consume_qualified_rule() {
                    Some(qrule) => Some(Rule::QualifiedRule(qrule)),
                    None => {
                        return Err(Syntax("syntax error".to_string()));
                    }
                }
            },
        }

        self.consume_whitespaces();

        if ! self.next_token_is_eof() {
            return Err(Syntax("syntax error".to_string()));
        }

        Ok(rule.unwrap())
    }

    pub fn parse_declaration(&mut self) -> Result<Declaration, Error> {
        self.consume_whitespaces();

        if match self.tokenizer.consume() {
            Token::Ident(_) => false,
            _ => true,
        } {
            return Err(Syntax("syntax error".to_string()));
        }

        match self.consume_declaration() {
            Some(declaration) => {
                return Ok(declaration);
            }
            _ => {}
        }

        Err(Syntax("syntax error".to_string()))
    }

    pub fn parse_style_block_content(&mut self) -> (Vec<Declaration>, Vec<Rule>)  {
        self.consume_style_block_content()
    }

    pub fn parse_list_of_declarations(&mut self) -> Vec<DeclarationAndAtRules> {
        self.consume_list_of_declarations()
    }

    pub fn parse_component_value(&mut self) -> Result<ComponentValue, Error> {
        self.consume_whitespaces();

        if self.next_token_is_eof() {
            return Err(UnexpectedEof);
        }
        let result = self.consume_component_value();

        self.consume_whitespaces();

        if self.next_token_is_eof() {
            return Ok(result.unwrap());
        }

        return Err(Syntax("syntax error".to_string()));
    }

    pub fn parse_list_of_component_values(&mut self) -> Vec<ComponentValue> {
        trace!("parse_list_of_component_values()");

        let mut cvalues = Vec::new();
        loop {
            match self.tokenizer.consume() {
                Token::EOF => break,
                _ => {
                    if let Some(component_value) = self.consume_component_value() {
                        cvalues.push(component_value);
                    }
                }
            }
        }

        trace!("returning: {:?}", cvalues);
        return cvalues;
    }

    pub fn parse_commaseparated_list_of_component_values(&mut self) -> Vec<ComponentValue> {
        let mut cvalues = Vec::new();

        loop {
            match self.tokenizer.consume() {
                Token::EOF => break,
                Token::Comma => {
                    self.tokenizer.consume();
                    continue;
                }
                _ => {
                    if let Some(component_value) = self.consume_component_value() {
                        cvalues.push(component_value);
                    }
                }
            }
        }

        return cvalues;
    }

    // =============================================================================================
    // Helper functions

    /// This will eat up whitespaces found in the stream until we reach a non-whitespace
    fn consume_whitespaces(&mut self) {
        loop {
            match self.tokenizer.consume() {
                Token::Whitespace => continue,
                _ => break,
            }
        }
    }

    /// Returns true when the next token is an EOF. It does NOT consume the token.
    fn next_token_is_eof(&self) -> bool {
        self.tokenizer.lookahead(1) == Token::EOF
    }

    // =============================================================================================
    // These are the internal consume_* functions

    fn consume_list_of_rules(&mut self, top_level_flag: bool) -> Vec<Rule> {
        let mut rules = Vec::new();

        loop {
            match self.tokenizer.consume() {
                Token::Whitespace => continue,
                Token::EOF => break,
                Token::CDC | Token::CDO => {
                    if top_level_flag {
                        continue;
                    }

                    self.tokenizer.reconsume();

                    match self.consume_qualified_rule() {
                        Some(qrule) => rules.push(Rule::QualifiedRule(qrule)),
                        None => {}
                    }
                }
                Token::AtKeyword(_) => {
                    self.tokenizer.reconsume();

                    if let Some(at_rule) = self.consume_at_rule() {
                        rules.push(Rule::AtRule(at_rule));
                    }
                }
                _ => {
                    self.tokenizer.reconsume();

                    if let Some(qrule) = self.consume_qualified_rule() {
                        rules.push(Rule::QualifiedRule(qrule));
                    }
                }
            }
        }

        rules
    }

    fn consume_at_rule(&mut self) -> Option<AtRule> {
        let mut at_rule = AtRule::default();

        loop {
            match self.tokenizer.consume() {
                Token::Semicolon => {
                    return Some(at_rule);
                }
                Token::EOF => {
                    // @Todo: parser error
                    return Some(at_rule);
                },
                Token::LCurly => {
                    if let Some(block) = self.consume_simple_block(Token::RCurly) {
                        at_rule.block = Some(block);
                        return Some(at_rule);
                    }
                }
                _ => {
                    self.tokenizer.reconsume();
                    if let Some(component_value) = self.consume_component_value() {
                        at_rule.prelude.push(component_value);
                    }
                }
            }
        }
    }

    fn consume_qualified_rule(&mut self) -> Option<QualifiedRule> {
        let mut qrule = QualifiedRule::default();

        loop {
            match self.tokenizer.consume() {
                Token::EOF => {
                    // parse error
                    return None
                },
                Token::LCurly => {
                    if let Some(block) = self.consume_simple_block(Token::RCurly) {
                        qrule.block = Some(block);
                        return Some(qrule);
                    }
                }
                // TODO: handle simpleblock with an associated token of <{-token>  !???
                _ => {
                    self.tokenizer.reconsume();
                    if let Some(component_value) = self.consume_component_value() {
                        qrule.prelude.push(component_value);
                    }
                }
            }
        }
    }

    // https://github.com/w3c/csswg-drafts/issues/7286
    // Basically, we have a list of declarations, and a list of rules. We separate them
    // in this function. But should we? Suppose we have:
    //
    //  p {
    //      color: red;         // declaration
    //      a {                 // rule
    //         color: blue;     // single declaration within the rule
    //      }
    //      background-color: white;    // declaration
    //  }
    //
    // In this we have a list of 2 declarations (color first, background-color second), and a list of 1 rule.
    // There is no ordering in this list
    //
    fn consume_style_block_content(&mut self) -> (Vec<Declaration>, Vec<Rule>) {
        let mut decls = Vec::new();
        let mut rules = Vec::new();

        loop {
            match self.tokenizer.consume() {
                Token::Whitespace | Token::Semicolon => {
                    // do nothing
                    continue;
                }
                Token::EOF => {
                    break;
                },
                Token::AtKeyword(_) => {
                    self.tokenizer.reconsume();
                    if let Some(at_rule) = self.consume_at_rule() {
                        rules.push(Rule::AtRule(at_rule));
                    }
                }
                Token::Ident(_) => {

                    // <ident-token>
                    //   Initialize a temporary list initially filled with the current input token. As long
                    //   as the next input token is anything other than a <semicolon-token> or <EOF-token>,
                    //   consume a component value and append it to the temporary list. Consume a declaration
                    //   from the temporary list. If anything was returned, append it to decls.

                    let mut tmp = vec![ComponentValue::PreservedToken(self.tokenizer.current())];
                    loop {
                        match self.tokenizer.consume() {
                            Token::Semicolon | Token::EOF => {
                                // continue
                            }
                            _ => {
                                self.tokenizer.reconsume();
                                if let Some(component_value) = self.consume_component_value() {
                                    tmp.push(component_value);
                                }
                                // @todo: this is not ok
                                if let Some(declaration) = self.consume_declaration() {
                                    decls.push(declaration);
                                }
                            }
                        }
                    }
                }
                Token::Delim(ch) if ch == '&' => {
                    self.tokenizer.reconsume();
                    if let Some(qrule) = self.consume_qualified_rule() {
                        rules.push(Rule::QualifiedRule(qrule));
                    }
                }
                _ => {
                    // parse error
                    self.tokenizer.reconsume();
                    self.consume_and_drop_component_values();
                }
            }
        }

        (decls, rules)
    }

    fn consume_and_drop_component_values(&mut self) {
        loop {
            match self.tokenizer.consume() {
                Token::Semicolon | Token::EOF => {
                    // continue
                }
                _ => {
                    self.tokenizer.reconsume();
                    // Do nothing with the component value
                    self.consume_component_value();
                }
            }
        }
    }

    /// Note that even though it says this consumes a list of declarations, it actually reutrns
    /// a list of declarations and at-rules. This is because the CSS grammar allows for at-rules
    /// to be mixed in with declarations. This is not the case for rules, which are always
    /// separated by a semicolon.
    fn consume_list_of_declarations(&mut self) -> Vec<DeclarationAndAtRules> {
        let mut mixed_list = Vec::new();

        loop {
            match self.tokenizer.consume() {
                Token::Whitespace | Token::Semicolon => {
                    // do nothing
                    continue;
                }
                Token::EOF => {
                    break;
                }
                Token::AtKeyword(_) => {
                    self.tokenizer.reconsume();
                    if let Some(at_rule) = self.consume_at_rule() {
                        mixed_list.push(DeclarationAndAtRules::AtRule(at_rule));
                    }
                }
                Token::Ident(_) => {
                    let mut tmp = vec![ComponentValue::PreservedToken(self.tokenizer.current())];
                    loop {
                        match self.tokenizer.consume() {
                            Token::Semicolon | Token::EOF => {
                                // continue
                            }
                            _ => {
                                if let Some(component_value) = self.consume_component_value() {
                                    tmp.push(component_value);
                                }

                                // @todo: consume declaration from tmp list
                            }
                        }
                    }
                }
                _ => {
                    // parse error
                    self.tokenizer.reconsume();
                    self.consume_and_drop_component_values();
                }
            }
        }

        mixed_list
    }

    fn consume_declaration(&mut self) -> Option<Declaration> {
        let mut declaration = Declaration::default();
        let t = self.tokenizer.consume();
        declaration.name = t.to_string();

        self.consume_whitespaces();

        if self.tokenizer.lookahead(0) != Token::Colon {
            // parse error
            return None;
        } else {
            self.tokenizer.consume();
        }

        self.consume_whitespaces();

        loop {
            match self.tokenizer.consume() {
                Token::EOF => break,
                _ => {
                    if let Some(component_value) = self.consume_component_value() {
                        declaration.value.push(component_value);
                    }
                }
            }
        }

        if declaration.value.len() >= 2 {
            if declaration.value[declaration.value.len() - 2] == ComponentValue::PreservedToken(Token::Delim('!')) {
                if declaration.value[declaration.value.len() - 1] == ComponentValue::PreservedToken(Token::Ident("important".to_string())) {
                    declaration.important = true;
                    declaration.value.pop();
                    declaration.value.pop();
                }
            }
        }

        while declaration.value.len() > 0 && declaration.value[declaration.value.len() - 1] == ComponentValue::PreservedToken(Token::Whitespace) {
            declaration.value.pop();
        }

        Some(declaration)
    }

    fn consume_component_value(&mut self) -> Option<ComponentValue> {
        let current_token = self.tokenizer.current();
        match self.tokenizer.consume() {
            Token::LCurly | Token::LBracket | Token::LParen => {
                match self.consume_simple_block(current_token) {
                    Some(block) => {
                        return Some(ComponentValue::SimpleBlock(block));
                    }
                    None => {
                        // parse error
                    }
                }
            }
            Token::Function(_) => {
                match self.consume_function() {
                    Some(function) => {
                        return Some(ComponentValue::Function(function));
                    }
                    None => {
                        // parse error
                    }
                }
            }
            _ => {} // return preserved token below
        }

        Some(ComponentValue::PreservedToken(self.tokenizer.current()))
    }

    fn consume_simple_block(&mut self, closing_token: Token) -> Option<SimpleBlock> {
        let mut block = SimpleBlock::new(self.tokenizer.current());

        loop {
            match self.tokenizer.consume() {
                Token::EOF => {
                    // @todo: parse_error
                    return Some(block);
                },
                _ => {
                    if self.tokenizer.current() == closing_token {
                        return Some(block);
                    }

                    self.tokenizer.reconsume();
                    if let Some(component_value) = self.consume_component_value() {
                        block.values.push(component_value);
                    }
                }
            }
        }
    }

    fn consume_function(&mut self) -> Option<Function> {
        let mut function = Function::default();
        function.name = self.tokenizer.current().to_string();

        loop {
            match self.tokenizer.consume() {
                Token::RParen => {
                    break;
                }
                Token::EOF => {
                    // parse error
                    break;
                }
                _ => {
                    if let Some(component_value) = self.consume_component_value() {
                        function.values.push(component_value);
                    }
                }
            }
        }

        Some(function)
    }

}


#[cfg(test)]
mod tests {
    use simple_logger::SimpleLogger;
    use crate::bytes::Encoding;
    use super::*;

    #[test]
    fn test_css3_parser() {
        SimpleLogger::new().init().unwrap();

        let mut ci = CharIterator::new();
        ci.read_from_str("\
test { color: #123; background-color: #11223344 }\
\
foo { yes: 12px }

@media screen and (min-width: 900px) {
    body {
        background-color: lightgreen;
    }
}\
\
", Some(Encoding::UTF8));
        let mut parser = CSS3ParserTng::from_input_stream(&mut ci);
        let node = parser.parse_stylesheet(Some("style.css".to_string()));

        println!("node: {:?}", node.unwrap());
        // assert_eq!(node, CssNode::Stylesheet(vec![]));
    }
}

