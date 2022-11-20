const BOSS_HP: isize = 109;
const BOSS_DMG: isize = 8;
const BOSS_DEF: isize = 2;

const PLAYER_HP: isize = 100;

fn calculate_when_zero(armor_rating: isize, attack_rating: isize, hp: isize) -> usize {
    let slope = std::cmp::max(attack_rating - armor_rating, 1);
    ((hp as f64) / (slope as f64)).ceil() as usize
}

fn determine_minimal_cost() -> usize {
    let mut shop = Item::create_shop();
    let mut equipment = Equipment::new();
    while calculate_when_zero(BOSS_DEF, equipment.calculate_attack_rating(), BOSS_HP) > calculate_when_zero(equipment.calculate_def_rating(), BOSS_DMG, PLAYER_HP) {
        equipment = equipment.determine_cheapest_upgrade(&mut shop)
    }

    equipment.get_cost()
}

fn determine_maximum_cost() -> usize {
    let equipments = Equipment::create_all_equipments(Item::create_option_shop());

    for equipment in equipments {
        if calculate_when_zero(BOSS_DEF, equipment.calculate_attack_rating(), BOSS_HP) > calculate_when_zero(equipment.calculate_def_rating(), BOSS_DMG, PLAYER_HP) {
            return equipment.get_cost()
        }
    }

    unreachable!();
}

#[derive(Clone, Copy, Debug)]
struct Equipment {
    weapon: Option<Item>, 
    armor: Option<Item>, 
    ring_1: Option<Item>, 
    ring_2: Option<Item>,
}

impl Equipment {
    fn new() -> Self {
        Equipment { weapon: None, armor: None, ring_1: None, ring_2: None }
    }

    fn get_cost(&self) -> usize {
        self.weapon.map(|item| item.get_cost()).unwrap_or(0) + 
        self.armor.map(|item| item.get_cost()).unwrap_or(0) + 
        self.ring_1.map(|item| item.get_cost()).unwrap_or(0) + 
        self.ring_2.map(|item| item.get_cost()).unwrap_or(0)
    }

    fn calculate_attack_rating(&self) -> isize {
        let mut attack_rating = 0;
        if let Some(Item::Weapon(val, _)) = self.weapon {
            attack_rating += val;
        }

        if let Some(Item::DmgRing(val, _)) = self.ring_1 {
            attack_rating += val;
        }

        if let Some(Item::DmgRing(val, _)) = self.ring_2 {
            attack_rating += val;
        }

        attack_rating
    }

    fn calculate_def_rating(&self) -> isize {
        let mut def_rating = 0;

        if let Some(Item::Armor(val, _)) = self.armor {
            def_rating += val;
        }

        if let Some(Item::DefRing(val, _)) = self.ring_1 {
            def_rating += val;
        }

        if let Some(Item::DefRing(val, _)) = self.ring_2 {
            def_rating += val;
        }

        def_rating
    }

    fn determine_cheapest_upgrade(self, shop: &mut Vec<Item>) -> Self {
        // for all items, select the item which will lead to the equipment set with the lowest cost
        // also store index of where to find item in shop
        let equipments = self.create_possible_equipments(shop);

        let (idx, min) = equipments.into_iter()
                            .min_by(|(_, a), (_, b)| a.get_cost().cmp(&b.get_cost()))
                            .unwrap();
        shop.remove(idx);

        min
    }

    fn create_possible_equipments(&self, shop: &mut Vec<Item>) -> Vec<(usize, Equipment)> {
        let mut equipments = Vec::<(usize, Equipment)>::new();

        for (i, item) in shop.iter().enumerate() {
            let mut new_equipment = self.clone();
            match &item {
                Item::Weapon(_, _) => new_equipment.weapon = Some(*item),
                Item::Armor(_, _) => new_equipment.armor = Some(*item),
                _ => {
                    let mut other_ring_slot = new_equipment.clone();
                    new_equipment.ring_1 = Some(*item);
                    other_ring_slot.ring_2 = Some(*item);
                    equipments.push((i, other_ring_slot));
                },
            }
            equipments.push((i, new_equipment))
        }

        equipments
    }

    fn create_all_equipments(option_shop: Vec<Vec<Option<Item>>>) -> Vec<Equipment> {
        let mut equipments = Vec::new();

        for weapon in &option_shop[0] {
            for armor in &option_shop[1] {
                for (i, ring_1) in option_shop[2].iter().enumerate() {
                    for (j, ring_2) in option_shop[2].iter().enumerate() {
                        if i == j { continue }
                        equipments.push(Equipment{ weapon: *weapon, armor: *armor, ring_1: *ring_1, ring_2: *ring_2 });
                    }
                }
            }
        }

        equipments.sort_by(|a, b| a.get_cost().cmp(&b.get_cost()).reverse());

        equipments
    }
}

#[derive(Clone, Copy, Debug)]
enum Item {
    Weapon(isize, usize), // (rating, cost)
    Armor(isize, usize),
    DmgRing(isize, usize),
    DefRing(isize, usize),
}

impl Item {
    // hardcode all weapons
    fn create_shop() -> Vec<Item> {
        use Item::*;
        vec![
            Weapon(4, 8),   // Dagger
            Weapon(5, 10),  // Shortsword
            Weapon(6, 25),  // Warhammer
            Weapon(7, 40),  // Longsword
            Weapon(8, 74),  // GreatAxe
            Armor(1, 13),   // Leather
            Armor(2, 31),   // Chainmail
            Armor(3, 53),   // Splintmail
            Armor(4, 75),   // Bandedmail
            Armor(5, 102),  // Platemail
            DmgRing(1, 25),
            DmgRing(2, 50),
            DmgRing(3, 100),
            DefRing(1, 20),
            DefRing(2, 40),
            DefRing(3, 80),
        ]
    }

    fn create_option_shop() -> Vec<Vec<Option<Item>>> {
        use Item::*;
        vec![
            vec![
                Some(Weapon(4, 8)),   // Dagger
                Some(Weapon(5, 10)),  // Shortsword
                Some(Weapon(6, 25)),  // Warhammer
                Some(Weapon(7, 40)),  // Longsword
                Some(Weapon(8, 74)),  // GreatAxe
            ],
            vec![
                None,
                Some(Armor(1, 13)),   // Leather
                Some(Armor(2, 31)),   // Chainmail
                Some(Armor(3, 53)),   // Splintmail
                Some(Armor(4, 75)),   // Bandedmail
                Some(Armor(5, 102)),  // Platemail
            ],
            vec![
                None,
                None,
                Some(DmgRing(1, 25)),
                Some(DmgRing(2, 50)),
                Some(DmgRing(3, 100)),
                Some(DefRing(1, 20)),
                Some(DefRing(2, 40)),
                Some(DefRing(3, 80)),
            ],
        ]
    }

    fn get_cost(&self) -> usize {
        *match self {
            Item::Weapon(_, c) => c,
            Item::Armor(_, c) => c,
            Item::DmgRing(_, c) => c,
            Item::DefRing(_, c) => c,
        }
    }
}

pub fn get_solution_1() -> usize {
    determine_minimal_cost()
}

pub fn get_solution_2() -> usize {
    determine_maximum_cost() 
}