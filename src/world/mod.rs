pub mod ground;
pub mod meta;
pub mod schedule;

use serde_json::{json, Value};

use crate::util;

#[derive(Debug)]
pub struct World {
    value: Value,
}

impl World {
    pub fn new(data: Value) -> Self {
        Self { value: data }
    }

    pub fn info(&self) -> &Value {
        &self.value
    }

    pub fn update(&mut self) -> &mut Value {
        &mut self.value
    }

    pub fn meta(&self) -> &Value {
        util::get(&self.value, &["railroad", "meta"])
    }

    pub fn meta_mut(&mut self) -> &mut Value {
        util::get_mut(&mut self.value, &["railroad", "meta"])
    }

    pub fn ground(&self) -> &Value {
        util::get(&self.value, &["railroad", "ground"])
    }

    pub fn ground_mut(&mut self) -> &mut Value {
        util::get_mut(&mut self.value, &["railroad", "ground"])
    }

    pub fn schedule(&self) -> &Value {
        util::get(&self.value, &["railroad", "schedule"])
    }

    pub async fn _run(&self) -> Value {
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
                            "ea9f4fe9-89bd-4317-8450-73904e9608d9": {
                                "points": ["efcb3338-d39f-47e4-af10-e52b8927f0bf"],
                                "rolling": [],
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
