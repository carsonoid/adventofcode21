use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: HashMap<String, Vec<(String, u32)>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, n: Node) {
        self.nodes.push(n);
    }

    pub fn add_edge(&mut self, n1: &str, n2: &str, weight: u32) {
        let e = self.edges.entry(n1.to_string()).or_default();
        e.push((n2.to_string(), weight));
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub id: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Edge<'n> {
    node: &'n Node,
    weight: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Path<'e> {
    edges: Vec<Edge<'e>>,
    weight: u32,
}

// pub fn do_test() {
//     let mut g = Graph::new();

//     g.add_node(Node {
//         id: "a".to_string(),
//     });
//     g.add_node(Node {
//         id: "b".to_string(),
//     });
//     g.add_node(Node {
//         id: "c".to_string(),
//     });
//     g.add_node(Node {
//         id: "d".to_string(),
//     });
//     g.add_node(Node {
//         id: "e".to_string(),
//     });

//     g.add_edge("a", "b", 4);
//     g.add_edge("a", "c", 2);
//     g.add_edge("b", "c", 3);
//     g.add_edge("b", "e", 3);
//     g.add_edge("b", "d", 2);
//     g.add_edge("c", "b", 1);
//     g.add_edge("c", "d", 4);
//     g.add_edge("c", "e", 5);
//     g.add_edge("e", "d", 1);

//     println!("{:?}", g);

//     get_shortest("a", "d", &g);

//     // let mut next = &a;

//     // let mut costs: HashMap<char, u32> = HashMap::new();
//     // for e in edges {
//     //     if e.ep1.id == next.id {
//     //         println!("PATH to {:?} costs {}", e.ep2, e.weight);
//     //         let cur = costs.entry(e.ep2.id).or_insert(10);
//     //         if e.weight < *cur {
//     //             *cur = e.weight;
//     //         }
//     //     }
//     // }
//     // println!("{:?}", costs);
// }

pub fn get_shortest(start: &str, end: &str, graph: &Graph) {
    // track unvisisted
    let mut unvisited = HashMap::new();
    for n in &graph.nodes {
        unvisited.insert(n.id.clone(), ());
    }

    // track shortest distances
    let mut shortest_distances = HashMap::new();
    for n in &graph.nodes {
        shortest_distances.insert(n.id.clone(), u32::MAX);
    }
    shortest_distances.insert(start.to_string(), 0);

    // path nodes
    let mut path_nodes = HashMap::new();

    loop {
        if unvisited.len() == 0 {
            break;
        }

        println!("unvisited {}", unvisited.len());

        let mut min_node = " ".to_string();
        for current_node in unvisited.keys() {
            if min_node == " " {
                min_node = current_node.to_string();
            } else if shortest_distances.get(&min_node).unwrap()
                > shortest_distances.get(current_node).unwrap()
            {
                min_node = current_node.to_string();
            }
        }

        if graph.edges.contains_key(&min_node) {
            for (node, value) in graph.edges.get(&min_node).unwrap() {
                if value + shortest_distances.get(&min_node).unwrap()
                    < *shortest_distances.get(node).unwrap()
                {
                    shortest_distances.insert(
                        node.clone(),
                        value + shortest_distances.get(&min_node).unwrap(),
                    );
                    path_nodes.insert(node, min_node.clone());
                }
            }
        }

        unvisited.remove(&min_node);
    }

    println!("shortest_distances: {:?}", shortest_distances);

    // find route
    let mut route: Vec<String> = Vec::new();

    let mut node = end.to_string();
    while node != start {
        route.insert(0, node.to_string());
        node = path_nodes.get(&node).expect("could not find").to_string();
    }

    println!("{:?}", route);

    println!("total_risk = {}", shortest_distances.get(end).unwrap())

    // // find total risk
    // let mut cur = start.to_string();
    // for node in route {
    //     println!("find cost from {} to {}", cur, node);
    //     // for e in graph.edges.get(&node).unwrap() {
    //     //     if e.0 ==
    //     // }
    //     cur = node;
    // }
}
