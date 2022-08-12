use macroquad::color::*;
use crate::nursery::behaviour::Behaviour;
use crate::nursery::{LivingEntity, EntityType};


pub fn create_zomby(clay_mod: &mut LivingEntity) {
    clay_mod.entity_type = EntityType::Zomby;
    clay_mod.glyph = 'Z' as u16;
    clay_mod.fg_color = RED;
    clay_mod.behaviour = Behaviour::Drunk;
}