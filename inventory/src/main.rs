use std::collections::HashMap;

trait InventoryItem {
    fn name(&self) -> &str;
    fn apply_effect(&self, character: &mut Character);
}

struct HealthPotion {
    name: String,
    heal_amount: i32,
}

impl InventoryItem for HealthPotion {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn apply_effect(&self, character: &mut Character) {
        character.health += self.heal_amount;
        println!(
            "{} used {}.  Health is now {}",
            character.name, self.name, character.health
        );
    }
}

struct ManaPotion {
    name: String,
    mana_amount: i32,
}

impl InventoryItem for ManaPotion {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn apply_effect(&self, character: &mut Character) {
        character.mana += self.mana_amount;
        println!(
            "{} used {}.  Mana is now {}",
            character.name, self.name, character.mana
        );
    }
}

struct StrengthAmulet {
    name: String,
    strength_boost: i32,
}

impl InventoryItem for StrengthAmulet {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn apply_effect(&self, character: &mut Character) {
        character.strength += self.strength_boost;
        println!(
            "{} used {}.  Strength is now {}",
            character.name, self.name, character.strength
        );
    }
}

struct Character {
    name: String,
    health: i32,
    mana: i32,
    strength: i32,
}

struct Inventory {
    inventory: HashMap<String, Box<dyn InventoryItem>>,
}

impl Inventory {
    fn new() -> Self {
        Self {
            inventory: HashMap::new(),
        }
    }

    fn add_item(&mut self, item: Box<dyn InventoryItem>) {
        self.inventory.insert(item.name().to_string(), item);
    }

    fn use_item(&self, name: &str, character: &mut Character) {
        if let Some(item) = self.inventory.get(name) {
            item.apply_effect(character);
        } else {
            println!("Invalid item: {}", name);
        }
    }
}

fn main() {
    let mut character = Character {
        name: "Hero".to_string(),
        health: 50,
        mana: 20,
        strength: 10,
    };

    let mut inventory = Inventory::new();

    inventory.add_item(Box::new(HealthPotion {
        name: "Small Health Potion".to_string(),
        heal_amount: 30,
    }));
    inventory.add_item(Box::new(ManaPotion {
        name: "Small Mana Potion".to_string(),
        mana_amount: 20,
    }));
    inventory.add_item(Box::new(StrengthAmulet {
        name: "Amulet of Strength".to_string(),
        strength_boost: 5,
    }));

    inventory.use_item("Small Health Potion", &mut character);
    inventory.use_item("Amulet of Strength", &mut character);
}
