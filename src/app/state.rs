use crate::app::Object;
use std::collections::HashMap;

pub trait StoreQuerier {
    fn get_all(&self) -> Vec<Object>;
}

pub trait StoreCommander {
    fn add(&mut self, o: Object);
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
    fn add(&mut self, o: Object) {
        self.objects.insert(o.id.clone(), o);
    }

    fn delete(&mut self, object_id: &String) {
        self.objects.remove(object_id);
    }
}
