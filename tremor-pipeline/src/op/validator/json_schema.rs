use crate::op::prelude::*;
use crate::{ConfigImpl, Event, Operator};
use simd_json::value::{BorrowedValue as Value, OwnedValue};
use value_trait::Builder;
use value_trait::Mutable;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub schema: String,
}

impl ConfigImpl for Config {}

#[derive(Debug, Clone)]
pub struct JsonSchema {
    pub config: Config,
}

impl From<Config> for JsonSchema {
    fn from(config: Config) -> Self {

        Self {
            config,
        }
    }
}
op!(JsonSchemaFactory (node) {
    if let Some(map) = &node.config {
        let config: Config = Config::new(map)?;
        Ok(Box::new(JsonSchema::from(config)))
    } else {
        Err(ErrorKind::MissingOpConfig(node.id.to_string()).into())
    }
});

impl Operator for JsonSchema {
    #[allow(mutable_transmutes)]
    fn on_event(&mut self, _port: &str, _state: &mut Value<'static>, event: Event) -> Result<Vec<(Cow<'static, str>, Event)>> {
        let (payload,_) = event.data.parts();

        let mut prop = OwnedValue::object();
        let mut cat = OwnedValue::object();
        cat.insert("type", "number").unwrap();
        cat.insert("minimum", 11.0).unwrap();
        let mut pat = OwnedValue::object();
        pat.insert("type", "string").unwrap();
        prop.insert("created_at", cat).unwrap();
        prop.insert("produced_at", pat).unwrap();
        let mut sobj = OwnedValue::object();
        sobj.insert("$id", "https://example.com/person.schema.json").unwrap();
        sobj.insert("$schema", "http://json-schema.org/draft-07/schema#").unwrap();
        sobj.insert("title", "Event").unwrap();
        sobj.insert("type", "object").unwrap();
        sobj.insert("properties", prop).unwrap();
        sobj.insert("additionalProperties", true).unwrap();
        let mut required = OwnedValue::array();
        required.push("created_at").unwrap();
        required.push("produced_at").unwrap();
        sobj.insert("required", required).unwrap();

        let mut scope: simd_json_schema::json_schema::scope::Scope<Value> = simd_json_schema::json_schema::scope::Scope::new();
        let schema = scope.compile_and_return(sobj.to_owned(), false).unwrap();
        let valid = schema.validate(&payload);
        println!("{:?}", valid);

        Ok(vec![])
    }
}
