use crate::traits::css3::{CssProperty, CssPropertyMap, CssStylesheet, CssSystem, CssValue};

pub trait HasCssSystem: Sized + HasCssSystemExt<Self> {
    type CssSystem: CssSystem;
}


pub trait HasCssSystemExt<C: HasCssSystem> {
    type Stylesheet: CssStylesheet;
    type CssPropertyMap: CssPropertyMap<C::CssSystem>;
    type Property: CssProperty<C::CssSystem>;
    type CssValue: CssValue;
}


impl<C: HasCssSystem> HasCssSystemExt<C> for C {
    type Stylesheet = <C::CssSystem as CssSystem>::Stylesheet;
    type CssPropertyMap = <C::CssSystem as CssSystem>::PropertyMap;
    type Property =  <C::CssSystem as CssSystem>::Property;
    type CssValue =  <C::CssSystem as CssSystem>::Value;
}
