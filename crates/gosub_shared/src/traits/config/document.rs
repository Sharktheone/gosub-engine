use crate::traits::config::css_system::HasCssSystem;
use crate::traits::document::{Document, DocumentFragment};
use crate::traits::html5::Html5Parser;
use crate::traits::node::{CommentDataType, DocTypeDataType, DocumentDataType, ElementDataType, Node, TextDataType};

pub trait HasDocument: Sized + HasCssSystem + HasDocumentExt<Self> {
    type Document: Document<Self>;
    type DocumentFragment: DocumentFragment<Self>;
}

pub trait HasHtmlParser: HasDocument {
    type HtmlParser: Html5Parser<Self>;
}


pub trait HasDocumentExt<C: HasDocument> {
    type Node: Node<C>;
    type DocumentData: DocumentDataType;
    type DocTypeData: DocTypeDataType;
    type TextData: TextDataType;
    type CommentData: CommentDataType;
    type ElementData: ElementDataType<C>;
}


impl<C: HasDocument> HasDocumentExt<C> for C {
    type Node = <C::Document as Document<Self>>::Node;
    type DocumentData = <Self::Node as Node<C>>::DocumentData;
    type DocTypeData = <Self::Node as Node<C>>::DocTypeData;
    type TextData = <Self::Node as Node<C>>::TextData;
    type CommentData = <Self::Node as Node<C>>::CommentData;
    type ElementData = <Self::Node as Node<C>>::ElementData;
}  