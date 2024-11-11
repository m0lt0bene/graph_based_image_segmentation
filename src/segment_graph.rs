use crate::union_find::{Edge, Universe};


/*
 * Segment a graph
 *
 * Returns a disjoint-set forest representing the segmentation.
 *
 * num_vertices: number of vertices in graph.
 * num_edges: number of edges in graph
 * edges: array of edges.
 * c: constant for threshold function.
 */
pub fn get_threshold(size:usize, val: f32) -> f32 {
  return val / size as f32
}

pub fn segment_graph(num_vertices: usize, num_edges: usize, edges: &mut Vec<Edge>, c: f32) -> Universe {
    // sort edges by weight
    edges.sort();
    // make a disjoint-set forest
    let mut u = Universe::new(num_vertices);

    // init thresholds
    let mut threshold = vec![get_threshold(1, c); num_vertices];

    // for each edge, in non-decreasing weight order...
    for i in 0..num_edges {
        let pedge = &edges[i];

        // components connected by this edge
        let a = u.find(pedge.a);
        let b = u.find(pedge.b);
        if a != b {
            if (pedge.w <= threshold[a]) && (pedge.w <= threshold[b]) {
                u.join(a, b);
                let a = u.find(a);
                threshold[a] = pedge.w + get_threshold(u.size(a).try_into().unwrap(), c);
            }
        }
    }

    u
}

