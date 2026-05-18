pub mod edge {
    use std::collections::HashMap;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Edge {
        from: String,
        to: String,
        attrs: HashMap<String, String>,
    }

    impl Edge {
        pub fn new (from: &str, to: &str) -> Self {
            Edge {
                from: String::from(from),
                to: String::from(to),
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs
                .extend(attrs.iter().map(|&(key, value)| (key.into(), value.into())));
            self
        }

        pub fn attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| &s[..])
        }
    }
}
pub mod node {
    use std::collections::HashMap;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Node {
        pub name: String,
        attrs: HashMap<String, String>,
    }

    impl Node {
        pub fn new(name: &str) -> Self {
            Node {
                name: String::from(name),
                attrs: HashMap::new()
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs
                .extend(attrs.iter().map(|&(key, value)| (key.into(), value.into())));
            self
        }

        pub fn attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| &s[..])
        }
    }
}
