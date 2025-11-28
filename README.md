# Branch Information

- This branch serves as an archive branch
- The main branch contains significant modifications to the Rust algorithm implementations for visualization purposes
- This branch preserves the undeveloped frontend components and the most fundamental Rust graph algorithm implementations
- Includes a React scaffold

# Development Commands Quick Reference

## Check Current Branch
```shell
# Verify the current working branch
git checkout -
```

## Rust Backend Development
```shell
cd lab4-rust-code4visualizer
cargo run
```

## React Frontend Development
```shell
cd lab4-web-visualizer

# Install dependencies (admin privileges required, first time only)
# npm install

npm run dev
```

**Development Server URL**: [http://localhost:5173/](http://localhost:5173/)

# Project Structure

```text
lab4/
├── lab4-rust-code4visualizer/     # 💻 Rust Algorithm Core
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs                 # Library Entry Point
│   │   ├── graph.rs               # Graph Data Structure
│   │   ├── dfs.rs                 # DFS Algorithm Implementation
│   │   ├── prim.rs                # Prim Algorithm Implementation
│   │   └── dijkstra.rs            # Dijkstra Algorithm Implementation
│   ├── Cargo.toml
│   └── Cargo.lock
│
├── lab4-web-visualizer/           # 🎨 React Frontend Visualization
│   ├── src/
│   │   └── ...
│   ├── public/                    # Static Assets
│   ├── package.json
│   ├── vite.config.ts            # Vite Configuration
│   └── index.html
│
├── README_pic/                    # Documentation Images
├── README.md                      # Main Documentation
├── readme4frontend.md             # Frontend-specific Documentation
└── readme4rust.md                 # Rust-specific Documentation
```