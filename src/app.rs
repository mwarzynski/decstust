use std::{thread, time::Duration};
use uuid::Uuid;

// Actions
//      - compares current real-world state with the declaration and outputs actions needed to
//      achieve desired state
pub mod actions;

// Objects manager in CQRS (command query responsibility segregation)
//      - Query to understand current state,
//      - Command to apply actions in order to achieve desired state,
pub mod objects_manager;
use crate::app::objects_manager::Querier;

// Desired State Declaration Store
//      - Store which allows to keep the state declaration specified by the user,
pub mod state;

type ObjectID = Uuid;

#[derive(Copy, Clone, Debug)]
pub struct Object {
    id: ObjectID,
    value: f64,
}

impl Object {
    pub fn new(value: f64) -> Self {
        let id = Uuid::new_v4();
        Object { id, value }
    }
}

pub fn start() {
    println!("[app] init");

    let mut state_manager = state::sqlite::Store::new().unwrap();
    let mut objects_manager = objects_manager::ObjectsInMemory::new();

    loop {
        state::chaos(&mut state_manager);

        let mut planner = actions::Planner::new(&state_manager, &objects_manager);
        planner.plan();
        planner.print_operations();
        planner.apply(&mut objects_manager);

        println!(
            "[objects] number of objects: {}",
            objects_manager.get_all().len()
        );

        thread::sleep(Duration::from_millis(1000));
    }
}
