use std::collections::HashMap;
use std::error::Error;

use crate::app::state::{Store as StateStore, StoreCommander, StoreQuerier};
use crate::app::{Object, ObjectID};

pub struct Store {
    objects: HashMap<ObjectID, Object>,
}

impl Store {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Store {
            objects: HashMap::new(),
        }
    }
}

impl StoreQuerier for Store {
    fn get_all(&self) -> Result<Vec<Object>, Box<dyn Error>> {
        Ok(self.objects.values().cloned().collect())
    }
}

impl StoreCommander for Store {
    fn upsert(&mut self, object: Object) -> Result<(), Box<dyn Error>> {
        self.objects.insert(object.id, object);
        Ok(())
    }

    fn delete(&mut self, object_id: &ObjectID) -> Result<(), Box<dyn Error>> {
        self.objects.remove(object_id);
        Ok(())
    }
}

impl StateStore for Store {}

#[cfg(test)]
mod tests {
    use crate::app::Object;

    use crate::app::state::in_memory::Store;

    use crate::app::state::StoreCommander;
    use crate::app::state::StoreQuerier;

    #[test]
    fn can_add_and_delete_objects() {
        let mut state = Store::new();
        let object = Object::new(10.1337);
        state.upsert(object).unwrap();
        assert_eq!(1, state.get_all().unwrap().len());
        state.delete(&object.id).unwrap();
        assert_eq!(0, state.get_all().unwrap().len());
    }
}
