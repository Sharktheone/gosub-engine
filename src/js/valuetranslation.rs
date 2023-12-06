use crate::js::JSValue;

pub trait ValueTranslation {
    type Value: JSValue;

    fn to_value(&self) -> Self::Value;
}


//TODO: implement this for different rust types