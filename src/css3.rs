use crate::bytes::CharIterator;
use crate::css3::new_tokenizer::{Token, Tokenizer};
use crate::types::Error;

mod new_tokenizer;
mod unicode;

struct Ruleset {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

enum Selector {
    Simple(String),
    Class(String),
    Id(String),
    Attribute(String, String),
    PseudoClass(String, String),
    PseudoElement(String, String),
    Compound(Vec<Selector>),
}

struct Declaration {
    property: String,
    value: Value,
}

enum Value {
    Color(String),
    Length(String),
    Keyword(String),
    Function(String, Vec<Value>),
    Url(String),
    Gradient(Gradient),
    CustomProperty(String),
}

struct Gradient {
    kind: String,
    stops: Vec<ColorStop>,
}

struct ColorStop {
    color: String,
    position: Option<String>
}

struct AtRule {
    rule_type: AtRuleType,
    condition: Option<String>,
    rulesets: Vec<CssRule>,
}

enum AtRuleType {
    Media,
    Supports,
    Document,
    Page,
    FontFace,
    Keyframes,
    Namespace,
    CounterStyle,
    Import,
    Charset,
    Viewport,
    Unknown(String),
}

struct Keyframes {
    name: String,
    frames: Vec<Keyframe>,
}

struct Keyframe {
    selector: KeyframeSelector,
    declarations: Vec<Declaration>,
}

enum KeyframeSelector {
    Percentage(String),
    From,
    To,
}

enum CssRule {
    Ruleset(Ruleset),
    AtRule(AtRule),
    Keyframes(Keyframes),
}

enum CssNode {
    Stylesheet(Vec<CssRule>),
    Comment(String),
}


pub struct CSS3ParserTng<'stream> {
    tokenizer: Tokenizer<'stream>,
}

struct QualifiedRule {
    prelude: Vec<Selector>,
    block: SimpleBlock,
}

struct SimpleBlock {
    associated_token: Token,
    values: Vec<Selector>,
}

impl SimpleBlock {
    fn new(associated_token: Token) -> SimpleBlock {
        SimpleBlock {
            associated_token,
            values: Vec::new(),
        }
    }
}



impl<'stream> CSS3ParserTng<'stream> {
    pub fn new(tokenizer: Tokenizer) -> CSS3ParserTng {
        CSS3ParserTng { tokenizer }
    }

    pub fn from_input_stream(ci: &mut CharIterator) -> CSS3ParserTng {
        CSS3ParserTng::new(Tokenizer::new(ci))
    }

    pub fn parse(&mut self) -> Result<CssNode, Error>
    {
        let top_level = true;
        let mut rulesets = Vec::new();

        loop {
            match self.tokenizer.next() {
                Some(Token::Whitespace) => continue,
                Some(Token::EOF) => break,
                Some(Token::CDO) => {
                    if top_level {
                        continue;
                    }

                    self.tokenizer.consume();
                    if let Some(rule) = self.consume_qualified_rule() {
                        rulesets.push(rule);
                    }
                }
                Some(Token::AtKeyword) => {
                    self.tokenizer.consume();
                    if let Some(rule) = self.consume_at_rule() {
                        rulesets.push(rule);
                    }
                }
                _ => {
                    self.tokenizer.consume();
                    if let Some(rule) = self.consume_qualified_rule() {
                        rulesets.push(rule);
                    }
                }
            }
        }

        Ok(CssNode::Stylesheet(rulesets))
    }

    fn consume_at_rule(&mut self) -> Option<AtRule> {
        let mut at_rule = AtRule::default();

        loop {
            match self.tokenizer.next() {
                Some(Token::Semicolon) => {
                    self.tokenizer.consume();
                    return Some(at_rule);
                }
                Some(Token::EOF) => {
                    /// @Todo: parser error
                    return Some(at_rule);
                },
                Some(Token::LCurly) => {
                    self.tokenizer.consume();
                    if let Some(block) = self.consume_simple_block(Token::RCurly) {
                        at_rule.block = block;
                        return Some(at_rule);
                    }
                }
                _ => {
                    if let Some(component_value) = self.tokenizer.consume_component_value() {
                        at_rule.prelude.push(component_value.to_string());
                    }
                }
            }
        }
    }

    fn consume_qualified_rule(&mut self) -> Option<QualifiedRule> {
        let mut qrule = QualifiedRule::default();

        loop {
            match self.tokenizer.next() {
                Some(Token::EOF) => return None,
                Some(Token::LCurly) => {
                    self.tokenizer.consume();
                    if let Some(block) = self.consume_simple_block(Token::RCurly) {
                        qrule.block = block;
                        return Some(qrule);
                    }
                }
                Some(Token::Whitespace) => continue,
                // TODO: handle simpleblock with an associated token of <{-token>  !???
                _ => {
                    if let Some(component_value) = self.tokenizer.consume_component_value() {
                        qrule.prelude.push(component_value);
                    }
                }
            }
        }
    }

    fn consume_simple_block(&mut self, closing_token: Token) -> Option<SimpleBlock> {
        let mut block = SimpleBlock::default();
        block.associated_token = closing_token.clone();

        loop {
            match self.tokenizer.next() {
                Some(Token::EOF) => {
                    // @todo: parse_error
                    return Some(block);
                },
                Some(closing_token) => {
                    return Some(block);
                }
                _ => {
                    if let Some(component_value) = self.tokenizer.consume_component_value() {
                        block.values.push(component_value);
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::bytes::Encoding;
    use super::*;

    #[test]
    fn test_css3_parser() {
        let mut ci = CharIterator::read_from_str("test { color: #123; background-color: #11223344 }", Encoding::UTF8);
        let mut parser = CSS3ParserTng::from_input_stream(&mut ci);
        let node = parser.parse();

        println!("node: {:?}", node);
        // assert_eq!(node, CssNode::Stylesheet(vec![]));
    }
}

