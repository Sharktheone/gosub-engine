use crate::css3::new_tokenizer::{Token, TokenStreamer};
use crate::css3::Error::{Syntax, UnexpectedEof};
use core::fmt::Debug;
use log::{debug, trace};
use thiserror::Error;

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
// Tokenstream is a simple already-tokenized stream of tokens. This is used for example when
// parsing a declaration, which is a list of tokens that are already known.
struct TokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream {
    fn new(tokens: Vec<Token>) -> TokenStream {
        TokenStream { tokens, index: 0 }
    }
}

impl TokenStreamer for TokenStream {
    fn current(&self) -> Token {
        if self.index >= self.tokens.len() {
            return Token::EOF;
        }

        self.tokens[self.index].clone()
    }

    fn lookahead(&self, n: usize) -> Token {
        if self.index + n >= self.tokens.len() {
            return Token::EOF;
        }

        self.tokens[self.index + n].clone()
    }

    fn consume(&mut self) -> Token {
        if self.index >= self.tokens.len() {
            return Token::EOF;
        }

        let token = self.tokens[self.index].clone();
        self.index += 1;
        token
    }

    fn reconsume(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }
}

// =================================================================================================

pub struct CSS3ParserTng<'a> {
    token_stream: &'a mut dyn TokenStreamer,
}

#[derive(Default)]
pub struct QualifiedRule {
    prelude: Vec<ComponentValue>,
    block: Option<SimpleBlock>,
}

impl Debug for QualifiedRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "QualifiedRule {{ prelude: {:?}, block: {:?} }}",
            self.prelude, self.block
        )
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
    SimpleBlock(SimpleBlock),
}

impl Debug for ComponentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentValue::PreservedToken(token) => write!(f, "token[{:?}]", token),
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
    block: Option<SimpleBlock>,
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

impl<'a> CSS3ParserTng<'a> {
    pub fn new(token_stream: &'a mut impl TokenStreamer) -> CSS3ParserTng {
        CSS3ParserTng { token_stream }
    }

    // =============================================================================================
    // These are the public parse_* functions

    pub fn parse(&mut self, _grammar: String) -> Result<Vec<ComponentValue>, Error> {
        debug!("parse()");
        let _result = self.parse_list_of_component_values();

        // @todo: match grammar against result // !????
        Err(Syntax("not implemented yet".to_string()))
    }

    pub fn parse_comma_separated_list(
        &mut self,
        _grammar: String,
    ) -> Result<Vec<ComponentValue>, Error> {
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

        match self.token_stream.lookahead(0) {
            Token::EOF => {
                return Err(Syntax("unexpected eof".to_string()));
            }
            Token::AtKeyword(_) => match self.consume_at_rule() {
                Some(at_rule) => {
                    rule = Some(Rule::AtRule(at_rule));
                }
                None => {
                    return Err(Syntax("syntax error".to_string()));
                }
            },
            _ => {
                rule = match self.consume_qualified_rule() {
                    Some(qrule) => Some(Rule::QualifiedRule(qrule)),
                    None => {
                        return Err(Syntax("syntax error".to_string()));
                    }
                }
            }
        }

        self.consume_whitespaces();

        if !self.next_token_is_eof() {
            return Err(Syntax("syntax error".to_string()));
        }

        Ok(rule.unwrap())
    }

    pub fn parse_declaration(&mut self) -> Result<Declaration, Error> {
        self.consume_whitespaces();

        if match self.token_stream.consume() {
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

    pub fn parse_style_block_content(&mut self) -> (Vec<Declaration>, Vec<Rule>) {
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

        Err(Syntax("syntax error".to_string()))
    }

    pub fn parse_list_of_component_values(&mut self) -> Vec<ComponentValue> {
        trace!("parse_list_of_component_values()");

        let mut cvalues = Vec::new();
        loop {
            match self.token_stream.consume() {
                Token::EOF => break,
                _ => {
                    if let Some(component_value) = self.consume_component_value() {
                        cvalues.push(component_value);
                    }
                }
            }
        }

        trace!("returning: {:?}", cvalues);
        cvalues
    }

    pub fn parse_commaseparated_list_of_component_values(&mut self) -> Vec<ComponentValue> {
        let mut cvalues = Vec::new();

        loop {
            match self.token_stream.consume() {
                Token::EOF => break,
                Token::Comma => {
                    self.token_stream.consume();
                    continue;
                }
                _ => {
                    if let Some(component_value) = self.consume_component_value() {
                        cvalues.push(component_value);
                    }
                }
            }
        }

        cvalues
    }

    // =============================================================================================
    // Helper functions

    /// This will eat up whitespaces found in the stream until we reach a non-whitespace
    fn consume_whitespaces(&mut self) {
        loop {
            match self.token_stream.consume() {
                Token::Whitespace => continue,
                _ => break,
            }
        }
    }

    /// Returns true when the next token is an EOF. It does NOT consume the token.
    fn next_token_is_eof(&self) -> bool {
        self.token_stream.lookahead(1) == Token::EOF
    }

    // =============================================================================================
    // These are the internal consume_* functions

    fn consume_list_of_rules(&mut self, top_level_flag: bool) -> Vec<Rule> {
        let mut rules = Vec::new();

        loop {
            match self.token_stream.consume() {
                Token::Whitespace => continue,
                Token::EOF => break,
                Token::CDC | Token::CDO => {
                    if top_level_flag {
                        continue;
                    }

                    self.token_stream.reconsume();
                    match self.consume_qualified_rule() {
                        Some(qrule) => rules.push(Rule::QualifiedRule(qrule)),
                        None => {}
                    }
                }
                Token::AtKeyword(_) => {
                    self.token_stream.reconsume();

                    if let Some(at_rule) = self.consume_at_rule() {
                        rules.push(Rule::AtRule(at_rule));
                    }
                }
                _ => {
                    self.token_stream.reconsume();

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
            match self.token_stream.consume() {
                Token::Semicolon => {
                    return Some(at_rule);
                }
                Token::EOF => {
                    // @Todo: parser error
                    return Some(at_rule);
                }
                Token::LCurly => {
                    if let Some(block) = self.consume_simple_block(Token::RCurly) {
                        at_rule.block = Some(block);
                        return Some(at_rule);
                    }
                }
                _ => {
                    self.token_stream.reconsume();
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
            match self.token_stream.consume() {
                Token::EOF => {
                    // parse error
                    return None;
                }
                Token::LCurly => {
                    if let Some(block) = self.consume_simple_block(Token::RCurly) {
                        qrule.block = Some(block);
                        return Some(qrule);
                    }
                }
                // TODO: handle simpleblock with an associated token of <{-token>  !???
                _ => {
                    self.token_stream.reconsume();
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
            match self.token_stream.consume() {
                Token::Whitespace | Token::Semicolon => {
                    // do nothing
                    continue;
                }
                Token::EOF => {
                    break;
                }
                Token::AtKeyword(_) => {
                    self.token_stream.reconsume();
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

                    let mut tmp_input = vec![self.token_stream.current()];
                    loop {
                        match self.token_stream.consume() {
                            Token::Semicolon | Token::EOF => break,
                            _ => {}
                        }

                        if let Some(component_value) = self.consume_component_value() {
                            match component_value {
                                ComponentValue::PreservedToken(token) => {
                                    tmp_input.push(token);
                                }
                                ComponentValue::Function(_function) => {
                                    panic!("we should not have a function here");
                                    // tmp_input.push(ComponentValue::Function(function));
                                }
                                ComponentValue::SimpleBlock(_block) => {
                                    panic!("we should not have a simple block here");
                                    // tmp_input.push(ComponentValue::SimpleBlock(block));
                                }
                            }
                        }

                        let mut token_stream = TokenStream::new(tmp_input.clone());
                        let mut parser = CSS3ParserTng::new(&mut token_stream);

                        if let Ok(declaration) = parser.parse_declaration() {
                            decls.push(declaration);
                        }
                    }
                }
                Token::Delim(ch) if ch == '&' => {
                    self.token_stream.reconsume();
                    if let Some(qrule) = self.consume_qualified_rule() {
                        rules.push(Rule::QualifiedRule(qrule));
                    }
                }
                _ => {
                    // parse error
                    self.token_stream.reconsume();
                    self.consume_and_drop_component_values();
                }
            }
        }

        (decls, rules)
    }

    fn consume_and_drop_component_values(&mut self) {
        loop {
            match self.token_stream.consume() {
                Token::Semicolon | Token::EOF => {
                    // continue
                }
                _ => {
                    self.token_stream.reconsume();
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
            match self.token_stream.consume() {
                Token::Whitespace | Token::Semicolon => {
                    // do nothing
                    continue;
                }
                Token::EOF => {
                    break;
                }
                Token::AtKeyword(_) => {
                    self.token_stream.reconsume();
                    if let Some(at_rule) = self.consume_at_rule() {
                        mixed_list.push(DeclarationAndAtRules::AtRule(at_rule));
                    }
                }
                Token::Ident(_) => {
                    let mut tmp = vec![ComponentValue::PreservedToken(self.token_stream.current())];
                    loop {
                        match self.token_stream.consume() {
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
                    self.token_stream.reconsume();
                    self.consume_and_drop_component_values();
                }
            }
        }

        mixed_list
    }

    fn consume_declaration(&mut self) -> Option<Declaration> {
        let mut declaration = Declaration::default();
        let t = self.token_stream.consume();
        declaration.name = t.to_string();

        self.consume_whitespaces();

        if self.token_stream.lookahead(0) != Token::Colon {
            // parse error
            return None;
        } else {
            self.token_stream.consume();
        }

        self.consume_whitespaces();

        loop {
            match self.token_stream.consume() {
                Token::EOF => break,
                _ => {
                    if let Some(component_value) = self.consume_component_value() {
                        declaration.value.push(component_value);
                    }
                }
            }
        }

        if declaration.value.len() >= 2 && declaration.value[declaration.value.len() - 2] == ComponentValue::PreservedToken(Token::Delim('!')) && declaration.value[declaration.value.len() - 1] == ComponentValue::PreservedToken(Token::Ident("important".to_string())) {
            declaration.important = true;
            declaration.value.pop();
            declaration.value.pop();
        }

        while !declaration.value.is_empty()
            && declaration.value[declaration.value.len() - 1]
                == ComponentValue::PreservedToken(Token::Whitespace)
        {
            declaration.value.pop();
        }

        Some(declaration)
    }

    fn consume_component_value(&mut self) -> Option<ComponentValue> {
        let current_token = self.token_stream.current();
        match self.token_stream.consume() {
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

        Some(ComponentValue::PreservedToken(self.token_stream.current()))
    }

    fn consume_simple_block(&mut self, closing_token: Token) -> Option<SimpleBlock> {
        let mut block = SimpleBlock::new(self.token_stream.current());

        loop {
            match self.token_stream.consume() {
                Token::EOF => {
                    // @todo: parse_error
                    return Some(block);
                }
                _ => {
                    if self.token_stream.current() == closing_token {
                        return Some(block);
                    }

                    self.token_stream.reconsume();
                    if let Some(component_value) = self.consume_component_value() {
                        block.values.push(component_value);
                    }
                }
            }
        }
    }

    fn consume_function(&mut self) -> Option<Function> {
        let mut function = Function::default();
        function.name = self.token_stream.current().to_string();

        loop {
            match self.token_stream.consume() {
                Token::RParen => {
                    trace!("consume_function(): returning {:?}", function);
                    break;
                }
                Token::EOF => {
                    // parse error
                    break;
                }
                _ => {
                    self.token_stream.reconsume();
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
    use super::*;
    use crate::bytes::CharIterator;
    use crate::bytes::Encoding;
    use crate::css3::new_tokenizer::Tokenizer;
    use simple_logger::SimpleLogger;

    #[test]
    fn test_css3_parser() {
        SimpleLogger::new().init().unwrap();

        let mut ci = CharIterator::new();
        ci.read_from_str(
            "
            hr .short, hr .long {
            background-color: var(--border-base-color);
            border: 0;
            color: var(--border-base-color);
            height: 1px;
            margin: 20px 0 0 0;
            overflow: hidden;
            padding: 0;
            text-align: left;
            width: 65px
        }",
            Some(Encoding::UTF8),
        );

        let mut tokenizer = Tokenizer::new(&mut ci);

        let mut parser = CSS3ParserTng::new(&mut tokenizer);
        let stylesheet = parser
            .parse_stylesheet(Some("style.css".to_string()))
            .unwrap();

        println!("stylesheet: {:?}", stylesheet);
    }
}
