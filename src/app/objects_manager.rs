use crate::app::Object;
use std::collections::HashMap;

pub trait Querier {
    fn get(&self, object_id: &String) -> Option<Object>;
    fn get_all(&self) -> Vec<Object>;
}

pub trait Commander {
    fn create(&mut self, object: Object);
    fn modify(&mut self, object_id: &String, value: f64);
    fn delete(&mut self, object_id: &String);
}

pub struct ObjectsInMemory {
    objects: HashMap<String, Object>,
}

impl ObjectsInMemory {
    pub fn new() -> Self {
        return ObjectsInMemory {
            objects: HashMap::new(),
        };
    }
}

impl Querier for ObjectsInMemory {
    fn get(&self, object_id: &String) -> Option<Object> {
        match self.objects.get(object_id) {
            Some(o) => Some(o.clone()),
            None => None,
        }
    }

    fn get_all(&self) -> Vec<Object> {
        return self.objects.values().cloned().collect();
    }
}

impl Commander for ObjectsInMemory {
    fn create(&mut self, object: Object) {
        self.objects.insert(object.id.clone(), object);
    }

    fn modify(&mut self, object_id: &String, value: f64) {
        match self.objects.get(object_id) {
            Some(object) => {
                let mut new_object = (*object).clone();
                new_object.value = value;
                self.objects.insert(object_id.clone(), new_object);
            }
            None => {}
        }
    }

    fn delete(&mut self, object_id: &String) {
        self.objects.remove(object_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::app::Object;

    use crate::app::objects_manager::Commander;
    use crate::app::objects_manager::Querier;

    use crate::app::objects_manager::ObjectsInMemory;

    #[test]
    fn can_create_object() {
        let mut manager = ObjectsInMemory::new();
        manager.create(Object::new(10.0));
        assert_eq!(1, manager.get_all().len());
    }

    #[test]
    fn can_modify_object() {
        let mut manager = ObjectsInMemory::new();
        let object = Object::new(10.0);
        manager.create(object.clone());
        manager.modify(&object.id, 15.0);
        match manager.get(&object.id) {
            Some(object) => {
                assert_eq!(15.0, object.value);
            }
            None => {
                assert!(false);
            }
        }
        assert_eq!(1, manager.get_all().len());
    }

    #[test]
    fn can_delete_object() {
        let object = Object::new(10.0);
        let mut manager = ObjectsInMemory::new();
        manager.create(object.clone());
        manager.delete(&object.id);
        assert_eq!(0, manager.get_all().len());
    }
}
