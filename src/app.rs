// Actions Planner
//      - compares current real-world state with the declaration and outputs actions needed to
//      achieve desired state
pub mod actions_planner;

// Objects manager in CQRS (command query responsibility segregation)
//      - Query to understand current state,
//      - Command to apply actions in order to achieve desired state,
pub mod objects_manager;

// Desired State Declaration Store
//      - Store which allows to keep the state declaration specified by the user,
pub mod state;

use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Object {
    id: String,
    value: f64,
}

impl Object {
    pub fn new(value: f64) -> Self {
        let id = Uuid::new_v4();
        return Object {
            id: id.to_string(),
            value: value,
        };
    }
}

pub fn start() {
    println!("[app] init");
}
