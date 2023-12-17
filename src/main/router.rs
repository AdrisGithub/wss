use std::collections::HashMap;

use whdp::HttpMethod;

use crate::helper::HTTPFunction;

pub struct Router {
    map: HashMap<String, HashMap<HttpMethod, HTTPFunction>>,
}

// TODO path param router
pub struct ParamRouter {
    router: Router,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ParamRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl ParamRouter {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }
    pub fn insert(&mut self, key: String, val: Methods) {
        self.router.insert(key, val)
    }
}

impl Router {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, val: Methods) {
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

pub enum Methods {
    GET(HTTPFunction),
}

impl Methods {
    pub fn get_inner(self) -> HTTPFunction {
        match self {
            Methods::GET(s) => s,
        }
    }
    pub fn get_type(&self) -> HttpMethod {
        match self {
            Methods::GET(_) => HttpMethod::Get,
        }
    }
}
