use crate::app::Object;
use crate::app::ObjectID;

use crate::app::state::Querier as StateQuerier;

use crate::app::objects_manager::Commander as ObjectsManagerCommander;
use crate::app::objects_manager::Querier as ObjectsManagerQuerier;

pub trait Operation: std::fmt::Debug {
    fn exec(&self, objects_manager: &mut dyn ObjectsManagerCommander);
}

#[derive(Debug, Copy, Clone)]
pub struct OperationCreate {
    object: Object,
}

impl OperationCreate {
    pub fn new(object: Object) -> Self {
        return OperationCreate { object: object };
    }
}

impl Operation for OperationCreate {
    fn exec(&self, objects_manager: &mut dyn ObjectsManagerCommander) {
        objects_manager.create(self.object.clone());
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OperationModify {
    object_id: ObjectID,
    value: f64,
}

impl OperationModify {
    pub fn new(object_id: ObjectID, value: f64) -> Self {
        return OperationModify {
            object_id: object_id,
            value: value,
        };
    }
}

impl Operation for OperationModify {
    fn exec(&self, objects_manager: &mut dyn ObjectsManagerCommander) {
        objects_manager.modify(&self.object_id, self.value);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OperationDelete {
    object_id: ObjectID,
}

impl OperationDelete {
    pub fn new(object_id: ObjectID) -> Self {
        return OperationDelete {
            object_id: object_id,
        };
    }
}

impl Operation for OperationDelete {
    fn exec(&self, objects_manager: &mut dyn ObjectsManagerCommander) {
        objects_manager.delete(&self.object_id);
    }
}

pub struct Planner {
    state_desired: Vec<Object>,
    state_existing: Vec<Object>,

    operations: Vec<Box<dyn Operation>>,
}

impl Planner {
    pub fn new(
        state_querier: &dyn StateQuerier,
        objects_fetcher: &dyn ObjectsManagerQuerier,
    ) -> Self {
        let state_objects = state_querier.get_all();
        let real_objects = objects_fetcher.get_all();
        return Planner {
            state_desired: state_objects,
            state_existing: real_objects,
            operations: Vec::new(),
        };
    }

    pub fn plan(&mut self) {
        self.plan_create();
        self.plan_update();
        self.plan_delete();
    }

    fn plan_create(&mut self) {
        for object in &self.state_desired {
            let mut already_exists = false;
            for real_object in &self.state_existing {
                if real_object.id == object.id {
                    already_exists = true;
                    break;
                }
            }
            if !already_exists {
                let operation = OperationCreate::new(*object);
                self.operations.push(Box::new(operation));
            }
        }
    }

    fn plan_update(&mut self) {
        for object in &self.state_desired {
            for real_object in &self.state_existing {
                if real_object.id != object.id {
                    continue;
                }
                if real_object.value != object.value {
                    let operation = OperationModify::new(object.id, object.value);
                    self.operations.push(Box::new(operation));
                }
            }
        }
    }

    fn plan_delete(&mut self) {
        for real_object in &self.state_existing {
            let mut should_exist = false;
            for object in &self.state_desired {
                if real_object.id == object.id {
                    should_exist = true;
                    break;
                }
            }
            if !should_exist {
                let operation = OperationDelete::new(real_object.id);
                self.operations.push(Box::new(operation));
            }
        }
    }

    fn apply(&self, objects_manager: &mut dyn ObjectsManagerCommander) {
        for operation in &self.operations {
            operation.exec(objects_manager);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::Object;

    use crate::app::actions::Planner;

    use crate::app::objects_manager::Commander as OMCommander;
    use crate::app::objects_manager::ObjectsInMemory;
    use crate::app::objects_manager::Querier as OMQuerier;

    use crate::app::state::Commander as StateCommander;
    use crate::app::state::StoreInMemory;

    #[test]
    fn can_plan_objects_creation() {
        let mut state = StoreInMemory::new();
        let mut objects_manager = ObjectsInMemory::new();

        state.upsert(Object::new(10.0));

        let mut planner = Planner::new(&state, &objects_manager);
        planner.plan();
        planner.apply(&mut objects_manager);

        assert_eq!(1, objects_manager.get_all().len());
    }

    #[test]
    fn can_plan_objects_modification() {
        let mut object = Object::new(10.0);
        let mut state = StoreInMemory::new();
        let mut objects_manager = ObjectsInMemory::new();

        objects_manager.create(object);

        object.value = 1337.0;
        state.upsert(object);

        let mut planner = Planner::new(&state, &objects_manager);
        planner.plan();
        planner.apply(&mut objects_manager);

        match objects_manager.get(&object.id) {
            Some(object_modified) => {
                assert_eq!(1337.0, object_modified.value);
            }
            None => assert!(false),
        }
    }

    #[test]
    fn can_plan_objects_deletion() {
        let object = Object::new(10.0);
        let state = StoreInMemory::new();
        let mut objects_manager = ObjectsInMemory::new();

        objects_manager.create(object);

        let mut planner = Planner::new(&state, &objects_manager);
        planner.plan();
        planner.apply(&mut objects_manager);

        assert_eq!(0, objects_manager.get_all().len());
    }
}
