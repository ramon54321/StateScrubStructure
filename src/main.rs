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
        pub fn get_at(self : &Self, graph_label : GraphLabel, t : u64) -> Option<f64> {
            let graph = self.graphs.get(&graph_label);
            match graph {
                Some(g) => g.get_at(t),
                None => None
            }
        }
        pub fn print(self : &Self) {
            debug!("Entity {}.", self.id);
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
        pub(in state) fn get_at(self : &Self, t : u64) -> Option<f64> {
            use std::collections::Bound::Included;
            use std::collections::Bound::Excluded;
            use std::collections::Bound::Unbounded;
            let key_left = self.keys.range((Unbounded, (Excluded(t)))).next_back();
            let key_right = self.keys.range((Included(t), Unbounded)).next();
            let keys = (key_left, key_right);
            match keys {
                (Some((key_l, value_l)), Some((key_r, value_r))) => {
                    return Some(interpolate::between(*key_l, *value_l, *key_r, *value_r, t));
                },
                _ => return None,
            }
        }
        pub fn print(self : &Self) {
            for (key, value) in &self.keys {
                debug!("Key [{},{}]", key, value);
            }
        }
    }


    /*
        Interpolations
    */
    mod interpolate {
        pub fn between(x0 : u64, y0 : f64, x1 : u64, y1 : f64, t : u64) -> f64 {
            return y0 + (t - x0) as f64 * ((y1 - y0) / (x1 - x0) as f64);
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
        let entity = state.entities.get_mut(&0u16).unwrap();
        entity.add_graph(state::GraphLabel::PositionX);
        entity.add_key(state::GraphLabel::PositionX, 8, 10f64);
        entity.add_key(state::GraphLabel::PositionX, 7, 10f64);
        entity.add_key(state::GraphLabel::PositionX, 10, 10f64);
        entity.add_key(state::GraphLabel::PositionX, 2, 10f64);
        entity.add_key(state::GraphLabel::PositionX, 6, 10f64);
        entity.add_key(state::GraphLabel::PositionX, 16, 20f64);
        entity.add_key(state::GraphLabel::PositionX, 44, 10f64);
        println!("Value at t=13 : {:?}", entity.get_at(state::GraphLabel::PositionX, 13));
    }
}