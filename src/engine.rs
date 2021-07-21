use crate::dailyrium::Action;
use crate::living_entity::LivingEntity;


pub fn give_id() -> u32 {
    static mut ID: u32 = 0;
    unsafe {
        ID += 1;
        ID
    }
}

pub fn action_manager(ent: &mut LivingEntity) {
    match ent.action {
        Action::Move(dx, dy) => {
            ent.move_entity(dx, dy);
            ent.action = Action::Waiting;
        }
        _ => {}
    }
}
