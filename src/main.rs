mod state {
    use std::collections::HashMap;
    use std::collections::BTreeMap;


    /*
        State
    */
    pub struct State {
        pub entities: HashMap<u16, Entity>
    }
    impl State {
        pub fn new() -> State {
            State { entities: HashMap::new() }
        }
        pub fn print(self : &Self) {
            unsafe {
                debug!("There are {} entities in the state, and the next ID will be {}.",
                        self.entities.len(), NEXT_ENTITY_ID);
                for (id, entity) in &self.entities {
                    entity.print();
                }
            }
        }
    }


    /*
        Entity
    */
    static mut NEXT_ENTITY_ID : u16 = 0;
    pub struct Entity {
        id : u16,
        graphs : HashMap<GraphLabel, Graph>
    }
    impl Entity {
        pub fn new(state : &mut State) {
            unsafe {
                let mut new_entity = Entity { id: NEXT_ENTITY_ID,
                    graphs: HashMap::new() };
                state.entities.insert(NEXT_ENTITY_ID, new_entity);
                NEXT_ENTITY_ID += 1;
            }
        }
        pub fn add_graph(self : &mut Self, graph_label : GraphLabel) {
            self.graphs.insert(graph_label, Graph { keys: BTreeMap::new() });
        }
        pub fn add_key(self : &mut Self, graph_label : GraphLabel, t : u64, v : f64) {
            let graph = self.graphs.get_mut(&graph_label);
            match graph {
                Some(g) => g.add_key(t, v),
                None => ()
            }
        }
        pub fn print(self : &Self) {
            for (label, graph) in &self.graphs {
                graph.print();
            }
        }
    }


    /*
        Graph
    */
    #[derive(Hash, Eq, PartialEq)]
    pub enum GraphLabel {
        PositionX = 0,
        PositionY = 1,
        PositionZ = 2
    }
    struct Graph {
        keys: BTreeMap<u64, f64>
    }
    impl Graph {
        pub(in state) fn add_key(self : &mut Self, t : u64, v : f64) {
            debug!("Adding value {} at time {}.", v, t);
            self.keys.insert(t, v);
        }
        pub fn print(self : &Self) {
            for (key, value) in &self.keys {
                debug!("[ {} , {} ]", key, value);
            }
            //print!("[ {} , {} ]", );
        }
    }
}

#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
    simple_logger::init();

    debug!("Start");

    let mut state: state::State = state::State::new();

    state::Entity::new(&mut state);
    state::Entity::new(&mut state);
    state::Entity::new(&mut state);

    {
        let entity = state.entities.get_mut(&0u16);
        match entity {
            Some(e) => {
                    e.add_graph(state::GraphLabel::PositionX);
                    e.add_key(state::GraphLabel::PositionX, 4, 10f64);
                },
            None => ()
        }
    }

    state.print();

}