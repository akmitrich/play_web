pub mod ground;

use serde_json::{json, Value};

#[derive(Debug)]
pub struct World {
    value: Value,
}

impl World {
    pub fn info(&self) -> &Value {
        &self.value
    }

    pub fn update(&mut self) -> &mut Value {
        &mut self.value
    }

    pub async fn run(&self) -> Value {
        Value::Null
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            value: json!({
                "railroad": {
                    "meta": {
                        "name": "Линия в никуда",
                        "version": "0.0.0",
                        "id" : uuid::Uuid::new_v4().to_string()
                    },
                    "schedule": {},
                    "ground": {
                        "points": {
                            "efcb3338-d39f-47e4-af10-e52b8927f0bf": {"label": "M01"}
                        },
                        "sections": {
                            uuid::Uuid::new_v4().to_string(): {
                                "points": ["efcb3338-d39f-47e4-af10-e52b8927f0bf"]
                            }
                        }
                    },
                    "rolling":{
                        "cars": {}
                    }
                }
            }),
        }
    }
}
