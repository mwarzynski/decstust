use crate::app::Object;
use crate::app::ObjectID;

use core::result::Result;
use std::error::Error;

pub mod in_memory;
pub mod sqlite;

pub trait StoreQuerier {
    fn get_all(&self) -> Result<Vec<Object>, Box<dyn Error>>;
}

pub trait StoreCommander {
    fn upsert(&mut self, object: Object) -> Result<(), Box<dyn Error>>;
    fn delete(&mut self, object_id: &ObjectID) -> Result<(), Box<dyn Error>>;
}

pub trait Store: StoreQuerier + StoreCommander {}

pub fn chaos(state: &mut dyn Store) {
    match rand::random::<u8>() % 8 {
        1 => {
            // CREATE
            state
                .upsert(Object::new(rand::random::<f64>() * 100.0))
                .unwrap()
        }
        2 => {
            // MODIFY
            match state.get_all() {
                Ok(objects) => {
                    if !objects.is_empty() {
                        let mut object = objects[0];
                        object.value = rand::random::<f64>() * 100.0;
                        state.upsert(object).unwrap();
                    }
                }
                Err(err) => {
                    println!("[state] modify: get all objects: {:?}", err);
                }
            }
        }
        3 => {
            // DELETE
            match state.get_all() {
                Ok(objects) => {
                    if !objects.is_empty() {
                        state.delete(&objects[0].id).unwrap();
                    }
                }
                Err(err) => {
                    println!("[state] delete: get all objects: {:?}", err);
                }
            }
        }
        _ => {}
    }
}
