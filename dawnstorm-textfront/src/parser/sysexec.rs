use dawnstorm_core::{entity::Entity, syscall::SysCall, world::World};

pub fn sys_exec(call: &SysCall, _player: &mut Entity, _world: &mut World, _current_node: &mut String) {
    println!("{:?}", call);
}
