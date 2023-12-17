use std::collections::BTreeMap;

use whdp::HttpMethod;

use crate::helper::HTTPFunction;

pub struct Router {
    map: BTreeMap<String, BTreeMap<HttpMethod, HTTPFunction>>
}
pub struct ParamRouter{
    router: Router
}

impl Router {
    pub const fn new() -> Self {
        Self {
            map: BTreeMap::new()
        }
    }

    pub fn insert(&mut self, key: String, val: Methods) {
        if let Some(tree) = self.map.get_mut(&key) {
            tree.insert(val.get_type(), val.get_inner());
        } else {
            let mut init = BTreeMap::new();
            init.insert(val.get_type(), val.get_inner());
            self.map.insert(key, init);
        }
    }
    pub fn get(&self, key: &String) -> Option<&BTreeMap<HttpMethod, HTTPFunction>> {
        self.map.get(key)
    }
    pub fn get_func(&self, key: &String, method: &HttpMethod) -> Option<&HTTPFunction> {
        self.get(key).and_then(|e| e.get(method))
    }
}

pub enum Methods {
    GET(HTTPFunction),
}

impl Methods {
    pub fn get_inner(self) -> HTTPFunction {
        match self {
            Methods::GET(s) => s
        }
    }
    pub fn get_type(&self) -> HttpMethod {
        match self {
            Methods::GET(_) => HttpMethod::Get
        }
    }
}