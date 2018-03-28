mod state {
    use std::collections::HashMap;
    use std::collections::BTreeMap;
    use std::collections::Bound::Included;
    use std::collections::Bound::Excluded;
    use std::collections::Bound::Unbounded;


    /*
        State.
        Contains all entities whose states' are maintained.
    */
    pub struct State {
        pub entities: HashMap<u16, Entity>
    }
    impl State {
        pub fn new() -> State {
            State { entities: HashMap::new() }
        }

        /*
            Prints each entity.
        */
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
        Entity.
        Contains the graphs representing the state of the enitity over time.
    */
    // -- Global next entity id
    static mut NEXT_ENTITY_ID : u16 = 0;
    pub struct Entity {
        id : u16,
        graphs : HashMap<GraphLabel, Graph>
    }
    impl Entity {
        /*
            Creates a new entity and links it to the specified state to
            be managed.
        */
        pub fn new(state : &mut State) {
            unsafe {
                let mut new_entity = Entity { id: NEXT_ENTITY_ID,
                    graphs: HashMap::new() };
                state.entities.insert(NEXT_ENTITY_ID, new_entity);
                NEXT_ENTITY_ID += 1;
            }
        }

        /*
            Adds a new graph to the entity.

            Only one graph of each label can exist. Adding a graph which
            already exists will cause the current graph to be overwriten with
            an empty graph.
        */
        pub fn add_graph(self : &mut Self, graph_label : GraphLabel) {
            // -- Insert graph to map with given label
            self.graphs.insert(graph_label, Graph { keys: BTreeMap::new() });
        }

        /*
            Adds a new key to the specified graph with the given value at the
            given time.

            Does nothing if the graph does no exist.
        */
        pub fn add_key(self : &mut Self, graph_label : GraphLabel, t : u64, v : f64) {
            // -- Get graph given label
            let graph = self.graphs.get_mut(&graph_label);

            // -- Match on graph and return none or add key to graph
            match graph {
                Some(g) => g.add_key(t, v),
                None => ()
            }
        }

        /*
            Returns the linearly interpolated value on the given graph at
            time t.
        */
        pub fn get_linear(self : &Self, graph_label : GraphLabel, t : u64) -> Option<f64> {
            // -- Get graph given label
            let graph = self.graphs.get(&graph_label);

            // -- Match on graph and return none or value at t
            match graph {
                Some(g) => g.get_linear(t),
                None => None
            }
        }

        /*
            Returns the step value on the given graph of the last key less
            than the given time t.
        */
        pub fn get_step(self : &Self, graph_label : GraphLabel, t : u64) -> Option<f64> {
            // -- Get graph given label
            let graph = self.graphs.get(&graph_label);

            // -- Match on graph and return none or value at t
            match graph {
                Some(g) => g.get_step(t),
                None => None
            }
        }

        /*
            Prints each graph.
        */
        pub fn print(self : &Self) {
            debug!("Entity {}.", self.id);
            for (label, graph) in &self.graphs {
                graph.print();
            }
        }
    }


    /*
        Graph.
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
        /*
            Adds a key to the graph.

            Called from the containing entity. See details in containing entity.
        */
        pub(in state) fn add_key(self : &mut Self, t : u64, v : f64) {
            debug!("Adding value {} at time {}.", v, t);
            self.keys.insert(t, v);
        }

        /*
            Gets the linear value.

            Called from the containing entity. See details in containing entity.
        */
        pub(in state) fn get_linear(self : &Self, t : u64) -> Option<f64> {
            // -- Get key less than t
            let key_left = self.keys.range((Unbounded, (Excluded(t)))).next_back();

            // -- Get key more than or equal to t
            let key_right = self.keys.range((Included(t), Unbounded)).next();

            // -- Add keys to tuple
            let keys = (key_left, key_right);

            // -- Match keys and return none or the interpolated value
            match keys {
                (Some((key_l, value_l)), Some((key_r, value_r))) => {
                    return Some(interpolate::between(*key_l, *value_l,
                        *key_r, *value_r, t));
                },
                _ => return None,
            }
        }

        /*
            Gets the step value.

            Called from the containing entity. See details in containing entity.
        */
        pub(in state) fn get_step(self : &Self, t : u64) -> Option<f64> {
            // -- Get the last key less than or equal to t
            let keyOption = self.keys.range((Unbounded, (Included(t)))).next_back();

            // -- Match on keyOption and none or value
            match keyOption {
                Some((key, value)) => {
                    return Some(*value);
                },
                _ => return None,
            }
        }

        /*
            Prints each key in the graph.
        */
        pub fn print(self : &Self) {
            for (key, value) in &self.keys {
                debug!("Key [{},{}]", key, value);
            }
        }
    }


    /*
        Interpolations.
    */
    mod interpolate {
        /*
            Linear interpolation between y0 and y1, given x0, x1 and t.
        */
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

    let t = 15;

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
        println!("Linear Value at t=13 : {:?}", entity.get_linear(state::GraphLabel::PositionX, t));
        println!("Step Value at t=13 : {:?}", entity.get_step(state::GraphLabel::PositionX, t));
    }
}