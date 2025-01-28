use anyhow::{Context, Result};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;


/**
 * Builds a graph from a YAML value.
 */
fn build_graph(value: &Value, graph: &mut Graph<String, String>, parent: Option<NodeIndex>, node_map: &mut HashMap<String, NodeIndex>) -> Result<NodeIndex> {
    match value {
        Value::Mapping(map) => {
            let node_label = format!("Map({})", map.len());
            let node_idx = graph.add_node(node_label.clone());
            
            if let Some(parent_idx) = parent {
                graph.add_edge(parent_idx, node_idx, String::from("contains"));
            }

            for (key, val) in map {
                let key_str = key.as_str().context("Failed to convert key to string")?;
                let key_idx = graph.add_node(key_str.to_string());
                graph.add_edge(node_idx, key_idx, String::from("key"));
                build_graph(val, graph, Some(key_idx), node_map)?;
            }
            Ok(node_idx)
        }
        Value::Sequence(seq) => {
            let node_label = format!("Sequence({})", seq.len());
            let node_idx = graph.add_node(node_label);
            
            if let Some(parent_idx) = parent {
                graph.add_edge(parent_idx, node_idx, String::from("contains"));
            }

            for (i, val) in seq.iter().enumerate() {
                let idx_node = graph.add_node(format!("[{}]", i));
                graph.add_edge(node_idx, idx_node, String::from("index"));
                build_graph(val, graph, Some(idx_node), node_map)?;
            }
            Ok(node_idx)
        }
        Value::String(s) => {
            let node_idx = graph.add_node(format!("\"{}\"", s));
            if let Some(parent_idx) = parent {
                graph.add_edge(parent_idx, node_idx, String::from("value"));
            }
            Ok(node_idx)
        }
        Value::Number(n) => {
            let node_idx = graph.add_node(n.to_string());
            if let Some(parent_idx) = parent {
                graph.add_edge(parent_idx, node_idx, String::from("value"));
            }
            Ok(node_idx)
        }
        Value::Bool(b) => {
            let node_idx = graph.add_node(b.to_string());
            if let Some(parent_idx) = parent {
                graph.add_edge(parent_idx, node_idx, String::from("value"));
            }
            Ok(node_idx)
        }
        Value::Null => {
            let node_idx = graph.add_node("null".to_string());
            if let Some(parent_idx) = parent {
                graph.add_edge(parent_idx, node_idx, String::from("value"));
            }
            Ok(node_idx)
        }
        Value::Tagged(tagged) => {
            let node_label = format!("Tagged({})", tagged.tag);
            let node_idx = graph.add_node(node_label);
            
            if let Some(parent_idx) = parent {
                graph.add_edge(parent_idx, node_idx, String::from("contains"));
            }

            build_graph(&tagged.value, graph, Some(node_idx), node_map)?;
            Ok(node_idx)
        }
    }
}



fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <yaml_file>", args[0]);
        std::process::exit(1);
    }

    let yaml_content = fs::read_to_string(&args[1])
        .context("Failed to read YAML file")?;
    
    let yaml_value: Value = serde_yaml::from_str(&yaml_content)
        .context("Failed to parse YAML content")?;

    let mut graph = Graph::<String, String>::new();
    let mut node_map = HashMap::new();

    build_graph(&yaml_value, &mut graph, None, &mut node_map)?;

    // Output the graph in DOT format
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    Ok(())
}
