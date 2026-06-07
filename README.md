# constellation-map

> **Fleet as star chart — agents as stars, teams as constellations**

[![crates.io](https://img.shields.io/crates/v/constellation-map.svg)](https://crates.io/crates/constellation-map)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Maps the fleet to a night sky: agents are stars with brightness proportional to activity, teams are constellations, and the whole fleet is a navigable star chart.

## Mapping

- **Stars** = agents (position from embeddings, brightness from activity)
- **Constellations** = named teams (connected subgraphs)
- **Star Chart** = the full fleet visualization
- **Navigation** = find paths between agents via constellation edges
- **Magnitude Scale** = classify agents by activity level (1-6, like stellar magnitude)

The star chart is a live TUI display for fleet operators.

## Installation

```toml
[dependencies]
constellation-map = "0.1.0"
```

## License

MIT © [SuperInstance](https://github.com/SuperInstance)

---

*Part of the [Exocortex](https://github.com/SuperInstance/exocortex) project — persistent cognitive substrate for multi-agent systems.*
