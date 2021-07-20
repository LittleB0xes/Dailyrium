use crate::living_entity::LivingEntity;

pub enum Action {
    MOVE(i32, i32),
    WAITING,
}

pub fn action_manager(ent: &mut LivingEntity) {
    match ent.action {
        Action::MOVE(dx, dy) => {
            ent.move_entity(dx, dy);
            ent.action = Action::WAITING;
        }
        _ => {}
    }
}
