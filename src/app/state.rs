use crate::app::Object;
use crate::app::ObjectID;
use std::collections::HashMap;

pub trait Querier {
    fn get_all(&self) -> Vec<Object>;
}

pub trait Commander {
    fn upsert(&mut self, object: Object);
    fn delete(&mut self, object_id: &ObjectID);
}

pub struct StoreInMemory {
    objects: HashMap<ObjectID, Object>,
}

impl StoreInMemory {
    pub fn new() -> Self {
        return StoreInMemory {
            objects: HashMap::new(),
        };
    }
}

impl Querier for StoreInMemory {
    fn get_all(&self) -> Vec<Object> {
        return self.objects.values().cloned().collect();
    }
}

impl Commander for StoreInMemory {
    fn upsert(&mut self, object: Object) {
        self.objects.insert(object.id, object);
    }

    fn delete(&mut self, object_id: &ObjectID) {
        self.objects.remove(object_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::app::Object;

    use crate::app::state::StoreInMemory;

    use crate::app::state::Commander;
    use crate::app::state::Querier;

    #[test]
    fn can_add_and_delete_objects() {
        let mut state = StoreInMemory::new();
        let object = Object::new(10.1337);
        state.upsert(object.clone());
        assert_eq!(1, state.get_all().len());
        state.delete(&object.id);
        assert_eq!(0, state.get_all().len());
    }
}
