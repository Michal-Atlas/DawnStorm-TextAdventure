use dawnstorm_core::{entity::Entity, syscall::SysCall, world::World};

pub fn sys_exec(call: &SysCall, player: &mut Entity, world: &mut World, current_node: &mut String) {
    println!("{:?}", call);
}
