use std::collections::HashMap;

use whdp::HttpMethod;

use crate::helper::HTTPFunction;
use crate::methods::Methods;
#[derive(Clone, PartialEq, Eq, Debug)]
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

    pub fn insert(&mut self, key: &str, val: Methods) {
        if let Some(tree) = self.map.get_mut(key) {
            tree.insert(val.get_type(), val.get_inner());
        } else {
            let mut init = HashMap::new();
            init.insert(val.get_type(), val.get_inner());
            self.map.insert(String::from(key), init);
        }
    }
    pub fn get(&self, key: &str) -> Option<&HashMap<HttpMethod, HTTPFunction>> {
        self.map.get(key)
    }
    pub fn get_func(&self, key: &str, method: &HttpMethod) -> Option<&HTTPFunction> {
        self.get(key).and_then(|e| e.get(method))
    }
}
