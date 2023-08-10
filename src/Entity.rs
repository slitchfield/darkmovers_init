use std::fmt;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Entity {
    pub name: String,
    base_init: u32,
    d6_init: u32,
    cur_init: u32,
}

impl Entity {
    pub fn new(name: &str, base_init: u32, d6_init: u32) -> Self {
        Self {
            name: String::from(name),
            base_init,
            d6_init,
            cur_init: 0,
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            name: String::from("Lattice"),
            base_init: 10,
            d6_init: 1,
            cur_init: 0,
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:3} | {} ({}d{})",
            self.cur_init, self.name, self.base_init, self.d6_init
        )
    }
}
