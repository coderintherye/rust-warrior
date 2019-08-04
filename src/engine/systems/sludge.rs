//! contains system for sludge enemy AI

use std::cmp;

use specs::{prelude::*, System};

use crate::{engine::components::UnitComponent, unit::UnitType};

/// This system acts as an enemy AI, attacking the player if a sludge
/// exists and is in range of the [`Warrior`](crate::warrior::Warrior).
pub struct SludgeSystem;

impl<'a> System<'a> for SludgeSystem {
    type SystemData = WriteStorage<'a, UnitComponent>;

    fn run(&mut self, mut units: Self::SystemData) {
        let mut units = (&mut units).join();
        let warrior_comp = units
            .find(|comp| comp.unit.unit_type == UnitType::Warrior)
            .unwrap();
        for sludge_comp in units.filter(|comp| {
            comp.unit.unit_type == UnitType::Sludge || comp.unit.unit_type == UnitType::ThickSludge
        }) {
            let (wx, _) = warrior_comp.unit.position;
            let (sx, _) = sludge_comp.unit.position;
            let (hp, _) = sludge_comp.unit.hp;
            // if the Sludge is killed and the entity is deleted, but `world.maintain()` hasn't
            // been called yet, then we need to see if the Sludge is at 0 hp (dead) here.
            if hp > 0 && (wx - sx).abs() <= 1 {
                println!("{:?} attacks Warrior", sludge_comp.unit.unit_type);
                let (current, max) = warrior_comp.unit.hp;
                let remaining = cmp::max(current - sludge_comp.unit.atk, 0);
                println!(
                    "Warrior takes {} damage, {} HP left",
                    sludge_comp.unit.atk, remaining
                );
                warrior_comp.unit.hp = (remaining, max);
            }
        }
    }
}
