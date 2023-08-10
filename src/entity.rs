use std::fmt;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Entity {
    pub name: String,
    pub base_init: u32,
    pub d6_init: u32,
    pub cur_init: u32,
    pub turn_taken: bool,
}

impl Entity {
    pub fn new(name: &str, base_init: u32, d6_init: u32) -> Self {
        Self {
            name: String::from(name),
            base_init,
            d6_init,
            cur_init: 0,
            turn_taken: false,
        }
    }

    pub fn reroll_init(self: &mut Entity) {
        let mut new_init = self.base_init;
        for _ in 0..self.d6_init {
            new_init += (rand::random::<u32>() % 6) + 1; // Add 1d6
        }
        self.cur_init = new_init;
    }

    pub fn take_turn(self: &mut Entity) {
        if self.cur_init >= 10 {
            self.cur_init -= 10;
        } else {
            self.cur_init = 0;
        }
        self.turn_taken = true;
    }

    pub fn interrupt(self: &mut Entity, value: u32) {
        if self.cur_init >= value {
            self.cur_init -= value;
        } else {
            print!("Cannot interrupt; init too low");
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
            turn_taken: false,
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:3}\t|\t{} ({}+{}d6)",
            self.cur_init, self.name, self.base_init, self.d6_init
        )
    }
}
