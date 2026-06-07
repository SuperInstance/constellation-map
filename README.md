# constellation-map

> **Fleet as star chart — agents become stars, teams form constellations, navigate between them**

[![crates.io](https://img.shields.io/crates/v/constellation-map.svg)](https://crates.io/crates/constellation-map)
[![docs.rs](https://docs.rs/constellation-map/badge.svg)](https://docs.rs/constellation-map)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## What is Constellation Map?

When you run a fleet of AI agents, understanding who's active, how they're organized, and how they relate to each other becomes a visualization challenge. Constellation Map treats your fleet like a night sky:

- Each **agent** is a **star** with a brightness proportional to its activity level and a position derived from its embedding
- Each **team** is a **constellation** — a named group of stars connected by edges (communication channels, shared tasks)
- The entire fleet renders as an **ASCII star chart** that updates in real time
- **Navigation** between stars uses BFS pathfinding along constellation edges

Stars are classified by a **magnitude scale** (I–VI, borrowing from astronomy) based on their brightness, from "Hyperactive" (magnitude I, brightness ≥ 0.9) to "Dormant" (magnitude VI, brightness < 0.1).

## Why Does This Matter?

Managing multi-agent systems requires intuitive visualization:

- **Fleet health**: See at a glance which agents are busy (bright stars) and which are idle (dim stars)
- **Team structure**: Constellations reveal organizational hierarchy and communication patterns
- **Navigation**: Find paths between agents through the team graph — useful for message routing
- **Distance metrics**: Measure how far apart agents are in embedding space — proxies for capability similarity
- **ASCII rendering**: Works in any terminal, no GUI required — perfect for headless servers and monitoring dashboards

Real-world applications:
- **DevOps monitoring**: Visualize microservice agent fleets with real-time activity
- **Multi-agent coordination**: See which agent teams are active and how they're connected
- **Cluster analysis**: Detect disconnected components or isolated agents
- **Load balancing**: Identify overloaded (bright) and underutilized (dim) agents

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                  Constellation Map System                      │
│                                                              │
│  Agent Fleet                                                  │
│  ┌────────────────────────────────────────────────────┐      │
│  │  Agent A [0.9] ─── Agent B [0.7] ─── Agent C [0.3]│      │
│  │      │              │                    │         │      │
│  │  Agent D [0.8]     Agent E [0.1]      Agent F [0.6]│      │
│  └──────────────┬─────────────────────────────────────┘      │
│                 │                                             │
│                 ▼                                             │
│  Star Chart                                                   │
│  ╔══ team-alpha ══╗  ╔══ team-beta ══╗                       │
│  ║ ★ agent-a [███]║  ║ ✦ agent-c [█] ║                      │
│  ║ ✦ agent-b [██ ]║  ║ · agent-f [█ ]║                      │
│  ║ ★ agent-d [███]║  ╚════════════════╝                      │
│  ║ · agent-e [   ]║     Magnitude Scale:                     │
│  ╚════════════════╝     ★ ≥ 0.8  (I: Hyperactive)           │
│                         ✦ ≥ 0.5  (II-III: Active)            │
│  Navigation:            · ≥ 0.2  (IV-V: Moderate-Quiet)      │
│  A ──▶ B ──▶ C         . < 0.2  (VI: Dormant)                │
│  (via constellation edges)                                    │
└──────────────────────────────────────────────────────────────┘
```

## Quick Start

```rust
use constellation_map::{Star, Constellation, StarChart, Navigation, Magnitude};

// Create a constellation (team)
let mut team = Constellation::new("backend-agents");
let a = team.add_star(Star::new("api-handler", 0.9, (0.0, 0.0)));
let b = team.add_star(Star::new("db-worker", 0.7, (1.0, 0.0)));
let c = team.add_star(Star::new("cache-mgr", 0.3, (2.0, 0.0)));
team.add_edge(a, b); // api-handler communicates with db-worker
team.add_edge(b, c); // db-worker communicates with cache-mgr

// Create the star chart
let mut chart = StarChart::new();
chart.add_constellation(team);

// Render as ASCII
println!("{}", chart.render_ascii());

// Classify agents by magnitude
let star = chart.find_star("api-handler").unwrap();
let mag = Magnitude::from_brightness(0.9);
println!("api-handler is magnitude {:?} ({})", mag, mag.label());
```

### Navigation Between Stars

```rust
// Find the shortest path between two agents
let nav = Navigation::new(&chart);
if let Some(path) = nav.find_path("api-handler", "cache-mgr") {
    println!("Route: {:?}", path);
    // Goes through: api-handler → db-worker → cache-mgr
}

// Compute distance in embedding space
let dist = nav.distance("api-handler", "db-worker");
println!("Euclidean distance: {:.2}", dist.unwrap());
```

### Multi-Constellation Charts

```rust
let mut chart = StarChart::new();

// Team 1
let mut backend = Constellation::new("backend");
backend.add_star(Star::new("worker-1", 0.8, (0.0, 0.0)));
backend.add_star(Star::new("worker-2", 0.6, (1.0, 0.0)));
chart.add_constellation(backend);

// Team 2
let mut frontend = Constellation::new("frontend");
frontend.add_star(Star::new("web-server", 0.9, (5.0, 0.0)));
chart.add_constellation(frontend);

println!("Total stars: {}", chart.total_stars());
```

## API Reference

### Star

| Method | Returns | Description |
|--------|---------|-------------|
| `Star::new(name, brightness, position)` | `Star` | Create a star (brightness clamped to 0.0–1.0) |
| `star.symbol()` | `char` | Display character: ★ ✦ · . based on brightness |

### Constellation

| Method | Returns | Description |
|--------|---------|-------------|
| `Constellation::new(name)` | `Constellation` | Create empty constellation |
| `c.add_star(star)` | `usize` | Add star, return its index |
| `c.add_edge(a, b)` | `()` | Connect two stars by index |
| `c.find_star(name)` | `Option<usize>` | Find star by name |
| `c.total_brightness()` | `f64` | Sum of all star brightnesses |

### StarChart

| Method | Returns | Description |
|--------|---------|-------------|
| `StarChart::new()` | `StarChart` | Create empty chart |
| `chart.add_constellation(c)` | `usize` | Add constellation, return index |
| `chart.find_star(name)` | `Option<(usize, usize)>` | Find star across all constellations |
| `chart.render_ascii()` | `String` | ASCII rendering of the full chart |
| `chart.total_stars()` | `usize` | Count stars across all constellations |

### Navigation

| Method | Returns | Description |
|--------|---------|-------------|
| `Navigation::new(&chart)` | `Navigation` | Create navigator |
| `nav.find_path(from, to)` | `Option<Vec<(usize, usize)>>` | BFS shortest path |
| `nav.distance(from, to)` | `Option<f64>` | Euclidean distance between stars |

### Magnitude

| Variant | Brightness | Label |
|---------|-----------|-------|
| `I` | ≥ 0.9 | Hyperactive |
| `II` | ≥ 0.7 | Very Active |
| `III` | ≥ 0.5 | Active |
| `IV` | ≥ 0.3 | Moderate |
| `V` | ≥ 0.1 | Quiet |
| `VI` | < 0.1 | Dormant |

| Method | Returns | Description |
|--------|---------|-------------|
| `Magnitude::from_brightness(b)` | `Magnitude` | Classify by brightness |
| `mag.label()` | `&str` | Human-readable label |

## Mathematical Background

### Astronomical Magnitude

The magnitude scale is borrowed from astronomy, where the apparent magnitude of a star is:
```
m = -2.5 · log₁₀(brightness) + constant
```
We simplify this to 6 discrete bins following the modern astronomical magnitude classes, mapping agent activity (normalized to [0, 1]) to brightness.

### Graph Navigation

The pathfinding uses **Breadth-First Search (BFS)** on the graph defined by constellation edges. BFS guarantees the shortest path (in number of hops) between any two connected stars. Time complexity is O(V + E) where V is the total number of stars and E is the total number of edges.

### Euclidean Distance

Star positions can be derived from agent embeddings (e.g., via t-SNE or PCA reduction to 2D). The distance between stars approximates the dissimilarity of the underlying agents:
```
d(s₁, s₂) = ||pos₁ - pos₂||₂ = √((x₁-x₂)² + (y₁-y₂)²)
```

## Installation

```bash
cargo add constellation-map
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
constellation-map = "0.1.0"
```

## Related Crates

- [`memory-plimpsest`](https://github.com/SuperInstance/memory-plimpsest) — Layered memory with ghost traces
- [`knowledge-compass`](https://github.com/SuperInstance/knowledge-compass) — Provenance navigation for knowledge graphs
- [`emotional-colorist`](https://github.com/SuperInstance/emotional-colorist) — Valence-based color mapping for agent states
- [`cortex-toml`](https://github.com/SuperInstance/cortex-toml) — Configuration-as-code for Exocortex

## License

MIT © [SuperInstance](https://github.com/SuperInstance)

---

*Part of the [Exocortex](https://github.com/SuperInstance/exocortex) project — persistent cognitive substrate for multi-agent systems.*
