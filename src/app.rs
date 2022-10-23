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
use crate::app::state::StoreCommander;
use crate::app::state::StoreQuerier;

#[derive(Clone, Debug)]
pub struct Object {
    id: String,
    value: f64,
}

pub fn start() {
    println!("[app] State init");

    let mut desired_state = state::StoreInMemory::new();
    desired_state.add(Object {
        id: String::from("test123"),
        value: 10.1337,
    });

    for obj in desired_state.get_all() {
        println!("[app] State object {:?}: {:?}", obj.id, obj);
    }

    println!("[app] State delete item id='test123'");
    desired_state.delete(&String::from("test123"));

    println!("[app] State size: {:?}", desired_state.get_all().len());
}
