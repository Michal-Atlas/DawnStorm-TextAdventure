use dawnstorm_core::{entity::Entity, syscall::SysCall, world::World};

pub fn sys_exec(call: SysCall, _player: &mut Entity, world: &mut World, current_node: &mut String) {
    println!("{:?}", call);
    match call {
        SysCall::Move(target) => {
            *current_node = target;
            println!("{}", &world[current_node.as_str()]);
        }
        SysCall::Print(msg) => println!("{}", msg),
        _ => {}
    }
}
