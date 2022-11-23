const BOSS_HP: isize = 51;
const BOSS_DMG: isize = 9;

const PLAYER_HP: isize = 50;
const PLAYER_MANA: isize = 500;

const MISSILE_COST: isize = 53;
const DRAIN_COST: isize = 73;
const SHIELD_COST: isize = 113;
const POISON_COST: isize = 173;
const RECHARGE_COST: isize = 229;

fn do_turn(mut player: Player, mut boss: Boss, is_player_turn: bool, cur_cost: isize, min_cost: &mut isize, is_hard_mode: bool) {
    // update buffs and debuffs
    if is_player_turn && is_hard_mode{
        player.hp -= 1;
        if player.is_dead() {
            return;
        }
    }

    player.update_buffs();    
    boss.update_poison();

    if boss.is_dead() {
        *min_cost = cur_cost;
        return;
    }

    if is_player_turn {
        let spells = player.get_castable_spells(&boss);
        for spell in spells {
            let new_cost = spell.get_cost() + cur_cost;
            if new_cost >= *min_cost {
                return;
            }

            let mut new_boss = boss;
            let mut new_player = player;

            new_player.cast(&spell, &mut new_boss);

            if boss.is_dead() {
                *min_cost = new_cost;
                return;
            }

            do_turn(new_player, new_boss, false, new_cost, min_cost, is_hard_mode);
        }
    } else {
        boss.attack(&mut player);

        if player.is_dead() {
            return;
        }

        do_turn(player, boss, true, cur_cost, min_cost, is_hard_mode);
    }
}

#[derive(Clone, Copy)]
struct Player {
    hp: isize,
    mp: isize,
    def: isize,
    shield: Option<usize>,
    recharge: Option<usize>,
}

impl Player {
    fn new() -> Self {
        Self { hp: PLAYER_HP, mp: PLAYER_MANA, def: 0, shield: None, recharge: None }
    }

    fn get_castable_spells(&self, boss: &Boss) -> Vec<Spell> {
        let mut spells = Vec::new();

        if self.mp >= MISSILE_COST {
            spells.push(Spell::Missile);
        }

        if self.mp >= DRAIN_COST {
            spells.push(Spell::Drain);
        }


        if self.shield.is_none() && self.mp >= SHIELD_COST {
            spells.push(Spell::Shield);
        }

        if self.recharge.is_none() && self.mp >= RECHARGE_COST {
            spells.push(Spell::Recharge);
        }

        if boss.poison.is_none() && self.mp >= POISON_COST {
            spells.push(Spell::Poison);
        }

        spells
    } 

    fn cast(&mut self, spell: &Spell, boss: &mut Boss) {
        self.mp -= spell.get_cost();
        match spell {
            Spell::Missile => self.cast_missile(boss),
            Spell::Drain => self.cast_drain(boss),
            Spell::Shield => self.cast_shield(),
            Spell::Poison => self.cast_poison(boss),
            Spell::Recharge => self.cast_recharge(),
        }
    }

    fn cast_shield(&mut self) {
        self.def += 7;
        self.shield = Some(6);
    }

    fn cast_recharge(&mut self) {
        self.recharge = Some(5);
    }

    fn cast_missile(&mut self, boss: &mut Boss) {
        boss.hp -= 4;
    }

    fn cast_drain(&mut self, boss: &mut Boss) {
        self.hp += 2;
        boss.hp -= 2; 
    }

    fn cast_poison(&mut self, boss: &mut Boss) {
        boss.poison = Some(6);
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    fn update_buffs(&mut self) {
        self.update_shield();
        self.update_recharge();
    }

    fn update_shield(&mut self) {
        // check for buffs
        if let Some(dur) = &mut self.shield {
            *dur -= 1;
            if *dur == 0 {
                self.def -= 7;
                self.shield = None;
            }
        }
    }

    fn update_recharge(&mut self) {
        if let Some(dur) = &mut self.recharge {
            self.mp += 101;
            *dur -= 1;

            if *dur == 0 {
                self.recharge = None;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Boss {
    hp: isize,
    dmg: isize,
    poison: Option<usize>
}

impl Boss {
    fn new() -> Self {
        Self { hp: BOSS_HP, dmg: BOSS_DMG, poison: None}
    }

    fn attack(&self, player: &mut Player) {
        player.hp -= std::cmp::max(self.dmg - player.def, 1);
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    fn update_poison(&mut self) {
        if let Some(dur) = &mut self.poison {
            self.hp -= 3;
            *dur -= 1;
            if *dur == 0 {
                self.poison = None;
            } 
        }
    }
}

enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn get_cost(&self) -> isize {
        match self {
            Spell::Missile => MISSILE_COST,
            Spell::Drain => DRAIN_COST,
            Spell::Shield => SHIELD_COST,
            Spell::Poison => POISON_COST,
            Spell::Recharge => RECHARGE_COST,
        }
    }
}

pub fn get_solution_1() -> isize {
    let mut min_cost = isize::MAX;

    do_turn(Player::new(), Boss::new(), true, 0, &mut min_cost, false);

    min_cost
}

pub fn get_solution_2() -> isize {
    let mut min_cost = isize::MAX;

    do_turn(Player::new(), Boss::new(), true, 0, &mut min_cost, true);

    min_cost
}
