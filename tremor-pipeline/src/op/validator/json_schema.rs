use crate::op::prelude::*;
use crate::{ConfigImpl, Event, Operator};
use simd_json::value::{BorrowedValue as Value, OwnedValue};
use argonaut;

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
    fn on_event(&mut self, _port: &str, _state: &mut Value<'static>, event: Event) -> Result<Vec<(Cow<'static, str>, Event)>> {
        let (payload,_) = event.data.parts();

        let schema_obj = argonaut::object(|json| {
            json.set("$id", OwnedValue::from("https://wayfair.com/accelerator.event.schema.json"));
            json.set("$schema", OwnedValue::from("http://json-schema.org/draft-07/schema#"));
            json.set("title", OwnedValue::from("Event"));
            json.set("type", OwnedValue::from("object"));

            let props = argonaut::object(|json| {
                let created_at = argonaut::object(|json| {
                    json.set("type", OwnedValue::from("number"));
                    json.set("minimum", OwnedValue::from(11.0));
                }).unwrap();

                let produced_at = argonaut::object(|json| {
                    json.set("type", OwnedValue::from("number"));
                }).unwrap();

                json.set("created_at", created_at);
                json.set("produced_at", produced_at);
            });

            let required = argonaut::array(|json| {
                json.push(OwnedValue::from("created_at")); 
                json.push(OwnedValue::from("created_at")); 
            });

            json.set("additionalProperties", OwnedValue::from(true));
            json.set("properties", props.unwrap());
            json.set("required", required.unwrap());
        }).unwrap();

        let mut scope: simd_json_schema::json_schema::scope::Scope<Value> = simd_json_schema::json_schema::scope::Scope::new();

        let schema = scope.compile_and_return(schema_obj, false).unwrap();
        let valid = schema.validate(&payload);
        println!("{:?}", valid);

        Ok(vec![])
    }
}
