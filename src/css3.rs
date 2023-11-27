use core::fmt::Debug;
use log::{debug, trace};
use thiserror::Error;
use crate::bytes::CharIterator;
use crate::css3::Error::{Syntax, UnexpectedEof};
use crate::css3::new_tokenizer::{Token, Tokenize, Tokenizer};

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

pub struct CSS3ParserTng {
    tokenizer: Box<dyn Tokenize>,
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


impl CSS3ParserTng {
    pub fn new(tokenizer: Box<dyn Tokenize>) -> CSS3ParserTng {
        CSS3ParserTng { tokenizer }
    }

    pub fn from_input_stream(ci: &mut CharIterator) -> CSS3ParserTng {
        CSS3ParserTng::new(Box::new(Tokenizer::new(ci)))
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

    pub fn parse_declaration(&mut self, input: Vec<Token>) -> Result<Declaration, Error> {
        self.consume_whitespaces();

        let old_tokenizer = self.tokenizer;
        self.tokenizer = Tokenizer::new_from_tokens(input);

        if match self.tokenizer.consume() {
            Token::Ident(_) => false,
            _ => true,
        } {
            self.tokenizer = old_tokenizer;
            return Err(Syntax("syntax error".to_string()));
        }

        match self.consume_declaration() {
            Some(declaration) => {
                self.tokenizer = old_tokenizer;
                return Ok(declaration);
            }
            _ => {}
        }

        self.tokenizer = old_tokenizer;
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

                    let mut tmp_input = vec![self.tokenizer.current()];
                    loop {
                        match self.tokenizer.consume() {
                            Token::Semicolon | Token::EOF => break,
                            _ => {}
                        }

                        if let Some(component_value) = self.consume_component_value() {
                            match component_value {
                                ComponentValue::PreservedToken(token) => {
                                    tmp_input.push(token);
                                }
                                ComponentValue::Function(function) => {
                                    panic!("we should not have a function here");
                                    // tmp_input.push(ComponentValue::Function(function));
                                }
                                ComponentValue::SimpleBlock(block) => {
                                    panic!("we should not have a simple block here");
                                    // tmp_input.push(ComponentValue::SimpleBlock(block));
                                }
                            }
                        }

                        if let Ok(declaration) = self.parse_declaration(tmp_input) {
                            decls.push(declaration);
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
                    trace!("consume_function(): returning {:?}", function);
                    break;
                }
                Token::EOF => {
                    // parse error
                    break;
                }
                _ => {
                    self.tokenizer.reconsume();
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
}
", Some(Encoding::UTF8));
/*
        ci.read_from_str("\
.pointer {
    cursor: pointer
}

.clear {
    clear: both
}

.clearfix::after {
    clear: both;
    content: \" \";
    display: block;
    font-size: 0;
    height: 0;
    visibility: hidden
}

.nounderline {
    text-decoration: none
}

.sprite {
    background-repeat: no-repeat;
    display: inline-block !important;
    text-align: left;
    overflow: hidden;
    text-indent: -1000px
}

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
}

hr .short + h4, hr .long + h4 {
    font-size: 12px;
    margin-bottom: 15px;
    margin-top: 0
}

hr .long {
    margin-top: 0;
    width: 125px
}

@font-face {
    font-family: \"IBM Plex Sans Condensed\";
    src: local(\"IBM Plex Sans Condensed\"), url(\"../../fonts/IBMPlexSansCondensed-Regular.woff2\") format(\"woff2\"), url(\"../../fonts/IBMPlexSansCondensed-Regular.woff\") format(\"woff\");
    font-weight: normal
}

@font-face {
    font-family: \"IBM Plex Sans Condensed\";
    src: local(\"IBM Plex Sans Condensed SemiBold\"), url(\"../../fonts/IBMPlexSansCondensed-SemiBold.woff2\") format(\"woff2\"), url(\"../../fonts/IBMPlexSansCondensed-SemiBold.woff\") format(\"woff\");
    font-weight: bold
}

@media screen {
    h1, h2, .heading, header .subheading, .ankeiler .title, .fpaTitle, .fpaItem .productTitle, .streamer {
        font-family: \"IBM Plex Sans Condensed\", \"calibri\", \"helvetica\", \"Liberation Sans\", sans-serif;
        font-weight: bold
    }
}

body, input, button, textarea, select, .bar, .sitename {
    font-family: \"arial\", \"helvetica\", \"Liberation Sans\", sans-serif
}

body, table, td, th, input, button, textarea, select, .bar {
    font-size: 12px
}

a {
    color: var(--text-link-color);
    text-decoration: none
}

a.highlightlink {
    text-decoration: underline
}

a.disabled {
    color: var(--disabled-paragraph-text-color)
}

a:hover {
    color: var(--text-link-hover-color);
    text-decoration: underline
}

a img {
    border: 0
}

.useVisitedState a:not(.ctaButton,.btn):visited {
    color: var(--text-link-visited-color)
}

table.ellipsis {
    display: table;
    table-layout: fixed
}

table.fixedLayout {
    display: table;
    table-layout: fixed
}

.nowrap {
    white-space: nowrap
}

table.ellipsis td {
    white-space: nowrap
}

td.ellipsis {
    display: table-cell
}

span[unselectable] {
    -webkit-touch-callout: none
}

div[unselectable] {
    -webkit-touch-callout: none
}

abbr, acronym {
    border-bottom: 1px dotted;
    text-decoration: none
}

fieldset {
    border: none;
    margin: 0;
    padding: 0
}

fieldset legend {
    display: none
}

.ellipsis {
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap
}

.ellipsis_lite {
    overflow: hidden;
    text-overflow: ellipsis
}

.nodisplay {
    display: none
}

.text-right {
    text-align: right
}

.readMore {
    background: var(--readmore-arrow) no-repeat 0 center/8px 7px;
    padding-left: 13px
}

a.readMore, .advertorial br + a {
    background: var(--readmore-link-arrow) no-repeat 0 center/8px 7px
}

#menuwrapper {
    height: 45px;
    min-width: 320px
}

#menubar {
    background-color: var(--brand-color);
    height: 45px;
    left: 0;
    position: fixed;
    top: 0;
    width: 100%;
    z-index: 110
}

:target::before {
    content: \"\";
    display: block;
    height: 55px;
    margin-top: -55px
}

.popup.menuHeaderPopup {
    position: fixed
}

#menu {
    margin: 0 auto;
    max-width: 1392px;
    position: relative
}

#menu ul {
    list-style: none;
    margin: 0;
    padding: 0
}

#menu .sitename + ul {
    height: 45px;
    margin-left: 156px;
    position: relative;
    width: 590px;
    z-index: 95
}

#menu li {
    cursor: pointer;
    font-size: 13px;
    height: 32px;
    line-height: 32px
}

#menu .more {
    position: relative
}

#menu .more > div {
    display: none
}

#menu .more.last {
    border-left: 1px solid rgba(0, 0, 0, 0);
    border-right: 1px solid rgba(0, 0, 0, 0)
}

#menu .dropdown > div {
    box-shadow: 8px 5px 5px -3px rgba(0, 0, 0, .1), 5px 8px 5px -3px rgba(0, 0, 0, .1);
    display: block;
    left: -1px;
    position: absolute;
    top: 38px;
    z-index: 99
}

#menu .dropdown ul {
    padding-bottom: 5px;
    padding-top: 5px;
    width: 100%
}

#menu .dropdown li {
    border: none;
    padding: 0;
    text-shadow: none
}

#menu .dropdown li a {
    display: block;
    padding: 0 10px
}

#menu .dropdown li span {
    display: block;
    padding: 0 10px
}

#menu .dropdown li.divider {
    border-top: 1px solid var(--border-base-color)
}

#menu .dropdown li.indent > a {
    padding-left: 20px
}

#menu > ul > li {
    float: left;
    height: 45px;
    line-height: 41px;
    padding-left: 1px
}

#menu > ul > li.blackFriday {
    background: var(--surface-black-friday)
}

#menu > ul > li.last {
    padding-right: 1px
}

#menu > ul > li > a, #menu > ul > li form > a {
    color: var(--text-always-light-color);
    display: block;
    font-family: \"IBM Plex Sans Condensed\", \"calibri\", \"helvetica\", \"Liberation Sans\", sans-serif;
    font-size: 15px;
    -webkit-font-smoothing: antialiased;
    font-weight: bold;
    height: 44px;
    padding: 0 7px;
    text-decoration: none;
    text-shadow: 1px 1px 0 rgba(0, 0, 0, .1)
}

#menu > ul > li.more > a.trigger::after {
    background: var(--navbar-main-chevron-state-closed) no-repeat center/6px 10px;
    content: \"\";
    display: inline-block;
    height: 10px;
    margin-left: 4px;
    transform: rotate(90deg);
    vertical-align: middle;
    width: 10px
}

#menu > ul > li.more.dropdown {
    background: var(--surface-card-color);
    border-left: 1px solid var(--border-card-color)
}

#menu > ul > li.more.dropdown > a.trigger {
    color: var(--base-paragraph-text-color);
    text-shadow: none
}

#menu > ul > li.more.dropdown > a.trigger::after {
    background-image: var(--navbar-main-chevron-state-open);
    transform: rotate(-90deg)
}

#menu > ul > li.more.dropdown > div > ul {
    border-top: none
}

#menu > ul > li.active a {
    color: var(--text-link-highlight-color)
}

.click-to-load--modal {
    background-color: var(--surface-card-color);
    color: var(--base-paragraph-text-color);
    border: 1px solid var(--border-card-color);
    box-shadow: 8px 5px 5px -3px rgba(0, 0, 0, .1), 5px 8px 5px -3px rgba(0, 0, 0, .1);
    box-sizing: border-box;
    margin: 25px 0;
    max-width: 90%;
    padding: 15px
}

.click-to-load--modal h2 {
    color: var(--base-paragraph-text-color);
    font-family: \"arial\", \"helvetica\", \"Liberation Sans\", sans-serif;
    font-size: 14px;
    line-height: 1.3;
    margin-bottom: 10px
}

.click-to-load--modal p {
    line-height: 1.6
}

.click-to-load--modal__title-divider {
    display: grid;
    grid-template-columns:90% 10%
}

.click-to-load--title {
    background: var(--surface-transparent-dark-color);
    color: var(--text-always-light-color);
    left: 6px;
    max-width: calc(100% - 22px);
    padding: 5px;
    position: absolute;
    top: 6px;
    z-index: 10
}

.cookieInfo p, .cookieInfo form {
    margin-bottom: 5px
}

.cookieInfo p:last-child, .cookieInfo form:last-child {
    margin: 0
}

.cookieInfo .moreInfo {
    color: var(--tertiary-paragraph-text-color);
    font-size: 11px
}

.editIcon {
    background: url(\"/g/if/icons/edit2.png\");
    height: 14px;
    width: 14px
}

.moveIcon {
    background: url(\"/g/if/icons/move2.png\");
    height: 13px;
    width: 13px
}

.permaLinkIcon {
    background: url(\"/g/if/icons/link2.png\");
    height: 12px;
    width: 14px
}

.deleteIcon {
    background: url(\"/g/if/icons/delete_product_blue.png\");
    height: 13px;
    width: 13px
}

.addIcon {
    background: url(\"/g/if/icons/list-plus-min.png\") -1px 0;
    height: 12px;
    width: 12px
}

.editIcon, .moveIcon, .permaLinkIcon, .deleteIcon, .addIcon {
    display: inline-block
}

.editIcon:hover, .moveIcon:hover, .permaLinkIcon:hover, .deleteIcon:hover, .addIcon:hover {
    opacity: .7
}

#layout {
    clear: both;
    padding-bottom: 249px
}

#contentArea {
    background: var(--surface-base-color);
    margin-bottom: 20px;
    padding-bottom: 20px;
    padding-top: 20px
}

#header {
    margin-bottom: 10px;
    overflow: hidden
}

#header h1 {
    margin-bottom: 5px
}

#header h1 .subtitle {
    color: var(--base-paragraph-text-color);
    display: block;
    font-size: 16px
}

#header h1 .subtitle a {
    color: var(--text-link-color)
}

div.hr hr {
    display: none
}

#menu .leftSidebarToggle, #menu .rightSidebarToggle, #navMenu li.frontpage, .site-sidebar {
    display: none
}

.pageIndex {
    margin-bottom: 15px;
    text-align: right
}

.pageNrResults {
    float: left;
    margin-right: 10px;
    text-align: left
}

.pageDistribution {
    text-align: left
}

.pageDistribution a, .pageDistribution .current {
    padding-left: 5px;
    padding-right: 5px
}

.pageDistribution .label {
    display: none
}

.greyTopBorderBlock {
    border-top: 2px solid var(--border-card-color)
}

.greyBottomBorderBlock {
    border-bottom: 2px solid var(--border-card-color)
}

.labelCard {
    background: var(--surface-base-color);
    border: 1px solid var(--border-base-color);
    border-radius: 2px;
    font-size: 11px;
    height: 17px;
    line-height: 17px;
    overflow: hidden;
    padding: 0
}

.labelCard.single {
    display: inline-block;
    height: 17px;
    margin-left: 5px
}

.labelCard > a {
    color: var(--secondary-paragraph-text-color);
    cursor: pointer;
    padding: 0 5px
}

img.lazyload {
    background: var(--surface-card-color)
}

img.error {
    background: var(--surface-error-color)
}

div.lazyload {
    height: 0;
    width: 0
}

.lazyerror {
    display: inline-block;
    position: relative
}

.lazyerror > img.error + img {
    left: 0;
    position: absolute;
    top: 0
}

.video-container.loading {
    position: relative
}

.video-container.loading::after {
    content: url(\"/g/imageviewer/loading.gif\");
    height: 32px;
    left: 50%;
    margin-left: -16px;
    margin-top: -16px;
    position: absolute;
    top: 50%;
    width: 32px
}

.video-container iframe {
    vertical-align: top
}

@media only screen and (-webkit-min-device-pixel-ratio: 1.25), only screen and (min-resolution: 120dpi) {
    #userbar li.icon a, .keywordSearch input.submit {
        background-image: url(\"/g/if/v3/framework/menu_icons_v2_x2.png\");
        background-size: 80px 180px
    }
}
", Some(Encoding::UTF8));
//         ci.read_from_str("\
// test { color: #123; background-color: #11223344 }\
// \
// foo { yes: 12px }
//
// @media screen and (min-width: 900px) {
//     body {
//         background-color: lightgreen;
//     }
// }\
// \
// ", Some(Encoding::UTF8));

 */
        let mut parser = CSS3ParserTng::from_input_stream(&mut ci);
        let stylesheet = parser.parse_stylesheet(Some("style.css".to_string())).unwrap();

        println!("stylesheet: {:?}", stylesheet);

        // for (idx, rule) in stylesheet.unwrap().rules.iter().enumerate() {
        //     match rule {
        //         Rule::AtRule(at_rule) => {
        //             println!("at-rule: {:?}", at_rule);
        //         }
        //         Rule::QualifiedRule(qrule) => {
        //             println!("qualified-rule: {:?}", qrule);
        //         }
        //         Rule::NormRule(nrule) => {
        //             println!("norm-rule: {:?}", nrule);
        //         }
        //     }
        //     stylesheet.rules[idx] = rule.clone();
        // }

        // assert_eq!(node, CssNode::Stylesheet(vec![]));
    }
}

