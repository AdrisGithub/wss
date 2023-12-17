use std::collections::HashMap;

use whdp::HttpMethod;
use wjp::{Deserialize, Serialize};

use crate::helper::HTTPFunction;
use crate::methods::Methods;

pub struct Router {
    map: HashMap<String, HashMap<HttpMethod, HTTPFunction>>,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert<I: Deserialize, O: Serialize>(&mut self, key: String, val: Methods<I, O>) {
        if let Some(tree) = self.map.get_mut(&key) {
            tree.insert(val.get_type(), val.get_inner());
        } else {
            let mut init = HashMap::new();
            init.insert(val.get_type(), val.get_inner());
            self.map.insert(key, init);
        }
    }
    pub fn get(&self, key: &String) -> Option<&HashMap<HttpMethod, HTTPFunction>> {
        self.map.get(key)
    }
    pub fn get_func(&self, key: &String, method: &HttpMethod) -> Option<&HTTPFunction> {
        self.get(key).and_then(|e| e.get(method))
    }
}