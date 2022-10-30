use std::error::Error;

use rusqlite::{params, Connection, Result};
use uuid::Uuid;

use crate::app::state::{Store as StateStore, StoreCommander, StoreQuerier};
use crate::app::{Object, ObjectID};

pub struct Store {
    conn: Connection,
}

impl Store {
    fn init_schema(&mut self) -> Result<(), Box<dyn Error>> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS objects (
             id INTEGER PRIMARY KEY,
             id_uuid TEXT NOT NULL UNIQUE,
             value NUMBER NOT NULL
         )",
            [],
        )?;
        Ok(())
    }

    pub fn new() -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open("database.sqlite")?;
        let mut store = Store { conn };
        store.init_schema()?;
        Ok(store)
    }

    #[allow(dead_code)]
    pub fn new_in_memory() -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open_in_memory()?;
        let mut store = Store { conn };
        store.init_schema()?;
        Ok(store)
    }
}

impl StoreQuerier for Store {
    fn get_all(&self) -> Result<Vec<Object>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare("SELECT id_uuid, value FROM objects;")?;

        let objects_iter = stmt.query_map(params![], |row| {
            let row_value: String = row.get(0)?;
            let id_raw = row_value;
            Ok(Object {
                id: Uuid::parse_str(&id_raw).unwrap(),
                value: row.get(1)?,
            })
        })?;

        let mut objects = Vec::new();
        for object in objects_iter {
            objects.push(object.unwrap());
        }

        Ok(objects)
    }
}

impl StoreCommander for Store {
    fn upsert(&mut self, object: Object) -> Result<(), Box<dyn Error>> {
        self.conn.execute(
            "INSERT INTO objects (id_uuid, value)
                  VALUES (?1, ?2) ON CONFLICT (id_uuid) DO UPDATE SET value = ?2",
            params![object.id.to_string(), object.value],
        )?;
        Ok(())
    }

    fn delete(&mut self, object_id: &ObjectID) -> Result<(), Box<dyn Error>> {
        self.conn.execute(
            "DELETE FROM objects WHERE id_uuid = ?1",
            params![object_id.to_string()],
        )?;
        Ok(())
    }
}

impl StateStore for Store {}

#[cfg(test)]
mod tests {
    use crate::app::state::StoreQuerier;
    use crate::app::state::{sqlite::Store, StoreCommander};
    use crate::app::Object;

    #[test]
    fn can_initialize_database() {
        let store = Store::new_in_memory().unwrap();
        assert_eq!(0, store.get_all().unwrap().len())
    }

    #[test]
    fn can_add_object() {
        let mut store = Store::new_in_memory().unwrap();
        let object = Object::new(10.0);
        store.upsert(object).unwrap();
        assert_eq!(1, store.get_all().unwrap().len())
    }

    #[test]
    fn can_upsert_object() {
        let mut store = Store::new_in_memory().unwrap();
        let mut object = Object::new(10.0);
        store.upsert(object).unwrap();
        object.value = 20.0;
        store.upsert(object).unwrap();
        assert_eq!(20.0, store.get_all().unwrap()[0].value)
    }

    #[test]
    fn can_delete_object() {
        let mut store = Store::new_in_memory().unwrap();
        let object = Object::new(10.0);
        store.upsert(object).unwrap();
        store.delete(&object.id).unwrap();
        assert_eq!(0, store.get_all().unwrap().len())
    }
}
