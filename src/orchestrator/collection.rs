use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Collection {
    name: String,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Collection { name: name }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}
