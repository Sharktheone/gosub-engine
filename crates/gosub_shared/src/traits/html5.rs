use crate::byte_stream::{ByteStream, Location};
use crate::document::DocumentHandle;
use crate::traits::config::HasDocument;
use crate::traits::css3::CssSystem;
use crate::traits::document::Document;

use crate::types::{ParseError, Result};

pub trait Html5Parser<C: HasDocument> {

    type Options: ParserOptions;

    fn parse(
        stream: ByteStream,
        doc: DocumentHandle<C>,
        opts: Option<Self::Options>,
    ) -> Result<Vec<ParseError>>;

    #[allow(clippy::type_complexity)]
    fn parse_fragment(
        stream: ByteStream,
        doc: DocumentHandle<C>,
        context_node: &C::Node,
        options: Option<Self::Options>,
        start_location: Location,
    ) -> Result<Vec<ParseError>>;
}

pub trait ParserOptions {
    fn new(scripting: bool) -> Self;
}
