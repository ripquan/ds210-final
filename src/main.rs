use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::algo::dijkstra;
use csv::ReaderBuilder;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use serde::Deserialize;
use std::time::Instant;

// Define a struct to represent a Reddit hyperlink record
#[derive(Debug, Deserialize)]
struct RedditLink {
    #[serde(rename = "SOURCE_SUBREDDIT")]
    source_subreddit: String,
    #[serde(rename = "TARGET_SUBREDDIT")]
    target_subreddit: String,
    #[serde(rename = "POST_ID")]
    post_id: String,
    #[serde(rename = "TIMESTAMP")]
    timestamp: String,
    #[serde(rename = "LINK_SENTIMENT")]
    post_label: i32,
    #[serde(rename = "PROPERTIES")]
    post_properties: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let file_path = "soc-redditHyperlinks-body.tsv"; // Update the file path as necessary

    // Create a directed graph
    let mut graph = DiGraph::<String, i32>::new();

    // Maps to store node indices for subreddits
    let mut subreddit_indices: HashMap<String, NodeIndex> = HashMap::new();

    // Read the CSV file and populate the graph
    let file_read_start = Instant::now();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: RedditLink = result?;

        // Get or insert the source subreddit
        let source_index = *subreddit_indices
            .entry(record.source_subreddit.clone())
            .or_insert_with(|| graph.add_node(record.source_subreddit.clone()));

        // Get or insert the target subreddit
        let target_index = *subreddit_indices
            .entry(record.target_subreddit.clone())
            .or_insert_with(|| graph.add_node(record.target_subreddit.clone()));

        // Add an edge between the subreddits
        graph.add_edge(source_index, target_index, record.post_label);
    }
    println!("Time to read file and build graph: {:?}", file_read_start.elapsed());

    // Compute centrality metrics
    let closeness_start = Instant::now();
    let closeness_centrality = compute_closeness_centrality(&graph);
    println!("Time to compute closeness centrality: {:?}", closeness_start.elapsed());

    // Print top-ranked subreddits by closeness centrality
    println!("Top-ranked subreddits by Closeness Centrality:");
    let mut closeness_sorted: Vec<_> = closeness_centrality.iter().collect();
    closeness_sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    for (subreddit, centrality) in closeness_sorted.iter().take(10) {
        println!("{}: {:.4} (hub-like or generalist)", subreddit, centrality);
    }

    let betweenness_start = Instant::now();
    let betweenness_centrality = compute_betweenness_centrality(&graph);
    println!("Time to compute betweenness centrality: {:?}", betweenness_start.elapsed());

    // Print top-ranked subreddits by betweenness centrality
    println!("Top-ranked subreddits by Betweenness Centrality:");
    let mut betweenness_sorted: Vec<_> = betweenness_centrality.iter().collect();
    betweenness_sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    for (subreddit, centrality) in betweenness_sorted.iter().take(10) {
        println!("{}: {:.4} (bridge between communities)", subreddit, centrality);
    }

    println!("Total execution time: {:?}", start_time.elapsed());
    
    // Run test functions
    test_compute_closeness_centrality();
    test_compute_betweenness_centrality();

    Ok(())
}

fn compute_closeness_centrality(graph: &DiGraph<String, i32>) -> HashMap<String, f64> {
    let mut centrality = HashMap::new();

    // Iterate over each node
    for node in graph.node_indices() {
        // Run Dijkstra's algorithm to compute shortest paths from the current node
        let distances = dijkstra(graph, node, None, |_| 1);

        // Sum the distances to all reachable nodes
        let total_distance: f64 = distances.values().map(|&d| d as f64).sum();

        // Compute closeness centrality
        let node_count = distances.len() as f64;
        if total_distance > 0.0 {
            let centrality_value = (node_count - 1.0) / total_distance;
            centrality.insert(graph[node].clone(), centrality_value);
        } else {
            centrality.insert(graph[node].clone(), 0.0);
        }
    }

    centrality
}

fn compute_betweenness_centrality(graph: &DiGraph<String, i32>) -> HashMap<String, f64> {
    let mut centrality = HashMap::new();

    for node in graph.node_indices() {
        centrality.insert(graph[node].clone(), 0.0);
    }

    for source in graph.node_indices() {
        let mut stack = Vec::new();
        let mut paths = HashMap::new();
        let mut distance = HashMap::new();
        let mut predecessors = HashMap::new();

        // Initialize BFS data structures
        for node in graph.node_indices() {
            paths.insert(node, 0.0);
            distance.insert(node, -1);
            predecessors.insert(node, Vec::new());
        }

        paths.insert(source, 1.0);
        distance.insert(source, 0);

        let mut queue = VecDeque::new();
        queue.push_back(source);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for edge in graph.edges(v) {
                let w = edge.target();

                // First visit to node `w`
                if distance[&w] < 0 {
                    queue.push_back(w);
                    distance.insert(w, distance[&v] + 1);
                }

                // Shortest path to `w` via `v`
                if distance[&w] == distance[&v] + 1 {
                    paths.insert(w, paths[&w] + paths[&v]);
                    predecessors.get_mut(&w).unwrap().push(v);
                }
            }
        }

        // Accumulate betweenness centrality
        let mut dependencies = HashMap::new();
        for node in graph.node_indices() {
            dependencies.insert(node, 0.0);
        }

        while let Some(w) = stack.pop() {
            for v in &predecessors[&w] {
                let ratio = paths[v] / paths[&w];
                dependencies.insert(*v, dependencies[v] + ratio * (1.0 + dependencies[&w]));
            }
            if w != source {
                *centrality.get_mut(&graph[w]).unwrap() += dependencies[&w];
            }
        }
    }

    // Normalize centrality scores
    let node_count = graph.node_count() as f64;
    for value in centrality.values_mut() {
        *value /= (node_count - 1.0) * (node_count - 2.0);
    }

    centrality
}

fn test_compute_closeness_centrality() {
    let mut graph = DiGraph::<String, i32>::new();
    let a = graph.add_node("A".to_string());
    let b = graph.add_node("B".to_string());
    let c = graph.add_node("C".to_string());
    let d = graph.add_node("D".to_string());

    graph.add_edge(a, b, 1);
    graph.add_edge(b, c, 1);
    graph.add_edge(c, d, 1);
    graph.add_edge(d, a, 1);

    let closeness = compute_closeness_centrality(&graph);

    assert!(closeness["A"] > 0.0);
    assert!(closeness["B"] > 0.0);
    println!("Closeness centrality test passed.");
}

fn test_compute_betweenness_centrality() {
    let mut graph = DiGraph::<String, i32>::new();
    let a = graph.add_node("A".to_string());
    let b = graph.add_node("B".to_string());
    let c = graph.add_node("C".to_string());
    let d = graph.add_node("D".to_string());

    graph.add
}