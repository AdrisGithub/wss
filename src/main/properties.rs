use std::collections::HashMap;
use std::fs::read_to_string;

struct Properties {
    props: HashMap<String, String>,
}

impl Properties {
    fn new() -> Properties {
        let mut map = HashMap::new();
        let res = read_to_string("application.properties");
        if let Ok(string) = res {
            let prop = string.lines()
                .map(|line| line.split('='))
                .map(|mut e| (e.next(), e.remainder()));

            prop.for_each(|e| {
                if let Some(key) = e.0 {
                    if let Some(value) = e.1 {
                        map.insert(key.to_string(), value.to_string());
                    }
                }
            })
        }
        Self {
            props: map
        }
    }

    fn get_attr(&self, key: &str) -> Option<String> {
        self.props.get(&String::from(key)).map(|str| str.to_string())
    }
}

pub struct PropWrapper(Option<Properties>);

impl PropWrapper {
    pub fn get_attr(&mut self, key: &str) -> Option<String> {
        match &self.0 {
            None => {
                self.0 = Some(Properties::new());
                self.get_attr(key)
            }
            Some(s) => s.get_attr(key)
        }
    }

    pub const fn new() -> PropWrapper {
        PropWrapper(None)
    }
}

pub static PROPS: PropWrapper = PropWrapper::new();
#[macro_export]
macro_rules! prop {
    ($key:expr) => {
        PROPS.get_attr($key)
    };
    () => {None}
}