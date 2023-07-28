pub mod ground;
pub mod meta;
pub mod rolling;
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

    pub fn info(&self, auth_token: Option<&str>) -> Value {
        if self.check_auth(auth_token) {
            return self.value.to_owned();
        }
        json!({
            "railroad":{
                "ground": self.ground(),
                "rolling": self.rolling(),
                "schedule": self.schedule(),
            }
        })
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

    pub fn rolling(&self) -> &Value {
        util::get(&self.value, &["railroad", "rolling"])
    }

    pub fn rolling_mut(&mut self) -> &mut Value {
        util::get_mut(&mut self.value, &["railroad", "rolling"])
    }

    pub fn schedule(&self) -> &Value {
        util::get(&self.value, &["railroad", "schedule"])
    }

    pub fn schedule_mut(&mut self) -> &mut Value {
        util::get_mut(&mut self.value, &["railroad", "schedule"])
    }

    pub async fn run(&mut self, auth_token: Option<&str>, steps: u64) -> Option<()> {
        if !self.check_auth(auth_token) {
            return None;
        }
        for _ in 0..steps {
            self.make_step().await;
        }
        Some(())
    }
}

impl World {
    fn check_auth(&self, _token: Option<&str>) -> bool {
        true
        // if let Some(auth_token) =
        //     token.and_then(|auth_token| uuid::Uuid::parse_str(auth_token).ok())
        // {
        //     if let Some(leader) = meta::get_leader_token(self.meta()) {
        //         if auth_token == leader {
        //             return true;
        //         }
        //     }
        // }
        // false
    }

    async fn make_step(&mut self) {
        self.prepare_to_move().await;
        self.do_move().await;
        self.after_move().await;
        schedule::advance_by_one(self.schedule_mut());
    }

    async fn prepare_to_move(&mut self) {
        ground::prepare_to_move(self.ground_mut());
        rolling::prepare_to_move(self.rolling_mut());
    }

    async fn do_move(&mut self) {
        ground::do_move(self.ground_mut());
        rolling::do_move(self.rolling_mut());
    }

    async fn after_move(&mut self) {
        ground::after_move(self.ground_mut());
        rolling::after_move(self.rolling_mut());
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
