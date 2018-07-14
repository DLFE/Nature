use diesel::prelude::*;
use super::*;

pub struct InstanceDaoImpl;

impl InstanceDaoTrait for InstanceDaoImpl {
    fn insert(instance: &Instance) -> Result<usize> {
        use self::schema::instances;
        let new = NewInstance::new(instance)?;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let rtn = diesel::insert_into(instances::table)
            .values(new)
            .execute(conn);
        match rtn {
            Ok(x) => Ok(x),
            Err(e) => Err(NatureError::from(e))
        }
    }

    fn get_last_status_by_id(instance_id: &u128) -> Result<Option<Instance>> {
        use self::schema::instances::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let def = instances.filter(id.eq(instance_id.to_bytes().to_vec()))
            .order(status_version.desc())
            .limit(1)
            .load::<NewInstance>(conn)?;
        match def.len() {
            0 => Ok(None),
            1 => Ok(Some(Instance::from(def[0].clone())?)),
            _ => Err(NatureError::SystemError("should less than 2 record return".to_string())),
        }
    }
    /// check whether source stored earlier
    fn is_exists(ins: &Instance) -> Result<bool> {
        use self::schema::instances::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let def = instances.filter(id.eq(ins.id.to_bytes().to_vec()))
            .filter(thing.eq(ins.thing.key.clone()))
            .filter(version.eq(ins.thing.version))
            .filter(status_version.eq(ins.status_version))
            .order(status_version.desc())
            .limit(1)
            .load::<NewInstance>(conn)?;
        match def.len() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(NatureError::SystemError("should less than 2 record return".to_string())),
        }
    }
}

impl InstanceDaoImpl {
    pub fn delete(ins: &Instance) -> Result<usize> {
        use self::schema::instances::dsl::*;
        let conn: &SqliteConnection = &CONN.lock().unwrap();
        let rtn = diesel::delete(instances
            .filter(id.eq(ins.id.to_bytes().to_vec()))
            .filter(thing.eq(ins.thing.key.clone()))
            .filter(version.eq(ins.thing.version))
            .filter(status_version.eq(ins.status_version))
        ).execute(conn);
        match rtn {
            Ok(x) => Ok(x),
            Err(err) => Err(NatureError::from(err))
        }
    }
}
