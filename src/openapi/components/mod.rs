use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pet {
    pub id: String,
    pub name: String,
    pub animal: Animal,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Animal {
    pub id: String,
    pub name: String,
}
