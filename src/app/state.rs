use crate::app::Object;
use std::collections::HashMap;

pub trait StoreQuerier {
    fn get_all(&self) -> Vec<Object>;
}

pub trait StoreCommander {
    fn upsert(&mut self, o: Object);
    fn delete(&mut self, object_id: &String);
}

pub struct StoreInMemory {
    objects: HashMap<String, Object>,
}

impl StoreInMemory {
    pub fn new() -> Self {
        return StoreInMemory {
            objects: HashMap::new(),
        };
    }
}

impl StoreQuerier for StoreInMemory {
    fn get_all(&self) -> Vec<Object> {
        return self.objects.values().cloned().collect();
    }
}

impl StoreCommander for StoreInMemory {
    fn upsert(&mut self, o: Object) {
        self.objects.insert(o.id.clone(), o);
    }

    fn delete(&mut self, object_id: &String) {
        self.objects.remove(object_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::app::Object;

    use crate::app::state::StoreInMemory;

    use crate::app::state::StoreCommander;
    use crate::app::state::StoreQuerier;

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
