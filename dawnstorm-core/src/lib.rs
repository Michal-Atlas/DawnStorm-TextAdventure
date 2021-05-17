pub mod dataload;
pub mod entity;
pub mod item;
pub mod load_methods;
pub mod profession;
pub mod savegame;
pub mod syscall;
pub mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
