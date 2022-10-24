use uuid::Uuid;

// Actions
//      - compares current real-world state with the declaration and outputs actions needed to
//      achieve desired state
pub mod actions;

// Objects manager in CQRS (command query responsibility segregation)
//      - Query to understand current state,
//      - Command to apply actions in order to achieve desired state,
pub mod objects_manager;

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
        return Object {
            id: id,
            value: value,
        };
    }
}

pub fn start() {
    println!("[app] init");
}
