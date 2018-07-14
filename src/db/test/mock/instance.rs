use db::trait_define::InstanceDaoTrait;
use std::ops::Deref;
use super::*;

lazy_static! {
    pub static ref TABLE_INSTANCE_LOCK: Mutex<u8> = Mutex::new(1);
    pub static ref TABLE_INSTANCE_INSERT_VALUE: Mutex<Result<usize>> = Mutex::new(Ok(0));
}

pub struct MockTableInstance;

impl InstanceDaoTrait for MockTableInstance {
    fn insert(_instance: &Instance) -> Result<usize> {
        println!("------------ InstanceDao insert-----------------");
        TABLE_INSTANCE_INSERT_VALUE.lock().unwrap().deref().clone()
    }
    fn get_last_status_by_id(_id: &u128) -> Result<Option<Instance>> {
        println!("------------ InstanceDao get_last_status_by_id-----------------");
        unimplemented!()
    }
    fn is_exists(_instance: &Instance) -> Result<bool> {
        println!("------------ InstanceDao is_exists-----------------");
        unimplemented!()
    }
}