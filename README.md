# YAML to Graph Converter

This Rust program converts YAML files into a graph representation, outputting the result in DOT format which can be visualized using tools like Graphviz.

## Requirements

- Rust (latest stable version)
- Graphviz (optional, for visualization)

## Installation

1. Clone the repository
2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

1. Run the program with a YAML file as input:
   ```bash
   cargo run example.yaml
   ```

2. To visualize the output, pipe it to Graphviz's dot command:
   ```bash
   cargo run example.yaml | dot -Tpng > graph.png
   ```

## Features

- Converts YAML structures into a graph representation
- Supports all YAML data types:
  - Mappings (objects)
  - Sequences (arrays)
  - Strings
  - Numbers
  - Booleans
  - Null values
- Outputs in DOT format for easy visualization

## Example

An example YAML file is provided (`example.yaml`). Try running it with:
```bash
cargo run example.yaml | dot -Tpng > graph.png
```
