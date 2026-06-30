use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AttributeValue {
    Int(i64),
    Str(String),
    Bool(bool),
    ListStr(Vec<String>),
}

pub type AttributesMap = HashMap<String, AttributeValue>;

pub struct AttributeHelper;

impl AttributeHelper {
    pub fn string_value(value: &AttributeValue) -> Result<String, String> {
        return if let AttributeValue::Str(val) = value {
            Ok(val.to_string())
        } else {
            Err("invalid string".to_owned())
        }
    }

    pub fn int_value(value: &AttributeValue) -> Result<i64, String> {
        return if let AttributeValue::Int(val) = value {
            Ok(*val)
        } else {
            Err("invalid integer".to_owned())
        }
    }

    pub fn bool_value(value: &AttributeValue) -> Result<bool, String> {
        return if let AttributeValue::Bool(val) = value {
            Ok(*val)
        } else {
            Err("invalid boolean".to_owned())
        }
    }

   pub fn decimal_value(value: &AttributeValue) -> Result<f64, String> {
       match value {
           AttributeValue::Int(val) => Ok(*val as f64),
           AttributeValue::Str(val) => {
               let result = val.parse::<f64>();
               return match result {
                   Ok(r) => Ok(r),
                   Err(_) => Err("invalid decimal".to_owned()),
               }
           }
           _ => return Err("invalid decimal".to_owned())
       }
   }
    pub fn is_valid_attribute(value: &AttributeValue)  -> bool {
        return match value {
            AttributeValue::Bool(_) => true,
            AttributeValue::Int(_) => true,
            AttributeValue::Str(_) => true,
            AttributeValue::ListStr(_) => true,
        }
    }
}

pub struct AttributeMapHelper;

impl AttributeMapHelper {
    pub fn bool_value(attributes: &AttributesMap, attribute_name: &str) -> Option<bool> {
        if attributes.is_empty() {
            return None;
        }
        let attribute = attributes.get(attribute_name);
        match attribute {
            Some(attr) => return match AttributeHelper::bool_value(&attr) {
                Ok(attr_value) => Some(attr_value),
                Err(_) => None
            },
            _ => {}
        };
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::attributes::AttributeValue;

    #[test]
    fn serialize_bool_attribute() {
        let result = serde_json::from_str::<AttributeValue>(
            serde_json::to_string(&AttributeValue::Bool(true))
                .unwrap()
                .as_str(),
        )
            .unwrap();
        assert_eq!(result, AttributeValue::Bool(true));
    }

    #[test]
    fn serialize_string_attribute() {
        let result = serde_json::from_str::<AttributeValue>(
            serde_json::to_string(&AttributeValue::Str("I'm Michel".to_owned()))
                .unwrap()
                .as_str()
        )
            .unwrap();
        assert_eq!(result, AttributeValue::Str("I'm Michel".to_owned()));
    }

    #[test]
    fn serialize_int_attribute() {
        let result = serde_json::from_str::<AttributeValue>(
            serde_json::to_string(&AttributeValue::Int(929))
                .unwrap()
                .as_str()
        )
            .unwrap();
        assert_eq!(result, AttributeValue::Int(929));
    }

    #[test]
    fn serialize_list_str_attribute() {
        let result = serde_json::from_str::<AttributeValue>(
            serde_json::to_string(&AttributeValue::ListStr(vec!["I'm Michel".to_owned()]))
                .unwrap()
                .as_str()
        )
            .unwrap();
        assert_eq!(result, AttributeValue::ListStr(vec!["I'm Michel".to_owned()]))
    }
}