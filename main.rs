mod state {
    use std::collections::HashMap;
    use std::collections::BTreeMap;

    pub struct State {
        entities: HashMap<u16, Entity>
    }
    impl State {
        pub fn new() -> State {
            State { entities: HashMap::new() }
        }
        pub fn print(self : &Self) {
            unsafe {
                println!("There are {} entities in the state, and the next ID will be {}.",
                         self.entities.len(), NEXT_ENTITY_ID);
            }
        }
    }

    static mut NEXT_ENTITY_ID : u16 = 0;
    pub struct Entity {
        id : u16,
        graphs : HashMap<GraphDataLabel, Graph>
    }
    impl Entity {
        pub fn new(state : &mut State){
            unsafe {
                let new_entity = Entity { id: NEXT_ENTITY_ID, graphs: HashMap::new() };
                state.entities.insert(NEXT_ENTITY_ID, new_entity);
                NEXT_ENTITY_ID += 1;
            }
        }
    }

    #[derive(Hash, Eq, PartialEq)]
    enum GraphDataLabel {
        PositionX = 0,
        PositionY = 1,
        PositionZ = 2
    }
    struct Graph {
        keys: BTreeMap<u64, f64>
    }
}

fn main() {
    println!("Start");

    let mut state: state::State = state::State::new();

    state::Entity::new(&mut state);
    state::Entity::new(&mut state);
    state::Entity::new(&mut state);

    state.print();

}