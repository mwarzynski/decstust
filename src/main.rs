// Goal: Component which ensures declarative state is reflected in the real-world environment.
//
// Components:
//  - Objects manager in CQRS (command query responsibility segregation)
//      - Query to understand current state,
//      - Command to apply actions in order to achieve desired state,
//  - Desired State Declaration Store
//      - Store which allows to keep the state declaration specified by the user,
//
//  - Actions Planner
//      - compares current real-world state with the declaration and outputs actions needed to
//      achieve desired state

fn main() {
    println!("Hello, world!");
}
