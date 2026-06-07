//! # constellation-map
//!
//! Fleet visualization as a star chart — agents become stars, teams form constellations,
//! and the whole fleet renders as an ASCII star map with navigation between stars.
//!
//! ## Core Types
//! - [`Star`] — An agent with name, brightness (activity level), and position (embedding)
//! - [`Constellation`] — A named group of stars connected by edges
//! - [`StarChart`] — All constellations rendered as an ASCII map
//! - [`Navigation`] — Find paths between stars via constellation edges
//! - [`MagnitudeScale`] — Classify agents by activity magnitude

/// A star representing an agent in the fleet.
#[derive(Debug, Clone)]
pub struct Star {
    /// Agent name.
    pub name: String,
    /// Brightness proportional to activity (0.0–1.0).
    pub brightness: f64,
    /// 2D position (e.g., reduced from embedding).
    pub position: (f64, f64),
}

impl Star {
    /// Create a new star.
    pub fn new(name: impl Into<String>, brightness: f64, position: (f64, f64)) -> Self {
        Self {
            name: name.into(),
            brightness: brightness.clamp(0.0, 1.0),
            position,
        }
    }

    /// Display character based on brightness.
    pub fn symbol(&self) -> char {
        match self.brightness {
            b if b >= 0.8 => '★',
            b if b >= 0.5 => '✦',
            b if b >= 0.2 => '·',
            _ => '.',
        }
    }
}

/// A constellation — a named group of stars connected by edges.
#[derive(Debug, Clone)]
pub struct Constellation {
    /// Name of the constellation (team name).
    pub name: String,
    /// Stars in this constellation.
    pub stars: Vec<Star>,
    /// Edges as pairs of star indices.
    pub edges: Vec<(usize, usize)>,
}

impl Constellation {
    /// Create a new empty constellation.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            stars: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Add a star and return its index.
    pub fn add_star(&mut self, star: Star) -> usize {
        let idx = self.stars.len();
        self.stars.push(star);
        idx
    }

    /// Add an edge between two stars by index.
    pub fn add_edge(&mut self, a: usize, b: usize) {
        if a < self.stars.len() && b < self.stars.len() && a != b {
            self.edges.push((a, b));
        }
    }

    /// Find a star by name.
    pub fn find_star(&self, name: &str) -> Option<usize> {
        self.stars.iter().position(|s| s.name == name)
    }

    /// Total brightness of all stars.
    pub fn total_brightness(&self) -> f64 {
        self.stars.iter().map(|s| s.brightness).sum()
    }
}

/// The full star chart — all constellations rendered as ASCII.
#[derive(Debug, Clone)]
pub struct StarChart {
    pub constellations: Vec<Constellation>,
}

impl Default for StarChart {
    fn default() -> Self {
        Self::new()
    }
}

impl StarChart {
    /// Create an empty star chart.
    pub fn new() -> Self {
        Self {
            constellations: Vec::new(),
        }
    }

    /// Add a constellation and return its index.
    pub fn add_constellation(&mut self, c: Constellation) -> usize {
        let idx = self.constellations.len();
        self.constellations.push(c);
        idx
    }

    /// Find a star across all constellations, returning (constellation_idx, star_idx).
    pub fn find_star(&self, name: &str) -> Option<(usize, usize)> {
        for (ci, c) in self.constellations.iter().enumerate() {
            if let Some(si) = c.find_star(name) {
                return Some((ci, si));
            }
        }
        None
    }

    /// Render the chart as a simple ASCII representation.
    pub fn render_ascii(&self) -> String {
        let mut lines = Vec::new();
        for c in &self.constellations {
            lines.push(format!("╔══ {} ══╗", c.name));
            for star in &c.stars {
                let bar: String = "█".repeat((star.brightness * 10.0) as usize);
                lines.push(format!("  {} {} [{}]", star.symbol(), star.name, bar));
            }
            lines.push("╚════════╝".to_string());
        }
        lines.join("\n")
    }

    /// Count total stars across all constellations.
    pub fn total_stars(&self) -> usize {
        self.constellations.iter().map(|c| c.stars.len()).sum()
    }
}

/// Navigation pathfinding between stars.
#[derive(Debug)]
pub struct Navigation<'a> {
    chart: &'a StarChart,
}

impl<'a> Navigation<'a> {
    /// Create a new navigation helper.
    pub fn new(chart: &'a StarChart) -> Self {
        Self { chart }
    }

    /// Find the shortest path between two named stars using BFS.
    /// Returns a list of (constellation_idx, star_idx) pairs forming the path.
    pub fn find_path(&self, from: &str, to: &str) -> Option<Vec<(usize, usize)>> {
        let start = self.chart.find_star(from)?;
        let end = self.chart.find_star(to)?;

        if start == end {
            return Some(vec![start]);
        }

        // Build adjacency: map star coords to neighbors
        use std::collections::{HashMap, VecDeque};
        let mut adj: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

        for (ci, c) in self.chart.constellations.iter().enumerate() {
            for &(a, b) in &c.edges {
                adj.entry((ci, a)).or_default().push((ci, b));
                adj.entry((ci, b)).or_default().push((ci, a));
            }
        }

        // BFS
        let mut visited = std::collections::HashSet::new();
        let mut queue: VecDeque<(Vec<(usize, usize)>, (usize, usize))> = VecDeque::new();
        queue.push_back((vec![start], start));
        visited.insert(start);

        while let Some((path, current)) = queue.pop_front() {
            if let Some(neighbors) = adj.get(&current) {
                for &next in neighbors {
                    if next == end {
                        let mut result = path;
                        result.push(end);
                        return Some(result);
                    }
                    if visited.insert(next) {
                        let mut new_path = path.clone();
                        new_path.push(next);
                        queue.push_back((new_path, next));
                    }
                }
            }
        }
        None
    }

    /// Get the distance between two stars by name.
    pub fn distance(&self, from: &str, to: &str) -> Option<f64> {
        let (fc, fs) = self.chart.find_star(from)?;
        let (tc, ts) = self.chart.find_star(to)?;
        let p1 = &self.chart.constellations[fc].stars[fs].position;
        let p2 = &self.chart.constellations[tc].stars[ts].position;
        let dx = p1.0 - p2.0;
        let dy = p1.1 - p2.1;
        Some((dx * dx + dy * dy).sqrt())
    }
}

/// Magnitude scale for classifying agents by activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Magnitude {
    /// Hyperactive: brightness >= 0.9
    I,
    /// Very active: 0.7–0.9
    II,
    /// Active: 0.5–0.7
    III,
    /// Moderate: 0.3–0.5
    IV,
    /// Quiet: 0.1–0.3
    V,
    /// Dormant: < 0.1
    VI,
}

impl Magnitude {
    /// Classify a star by its brightness.
    pub fn from_brightness(brightness: f64) -> Self {
        match brightness {
            b if b >= 0.9 => Magnitude::I,
            b if b >= 0.7 => Magnitude::II,
            b if b >= 0.5 => Magnitude::III,
            b if b >= 0.3 => Magnitude::IV,
            b if b >= 0.1 => Magnitude::V,
            _ => Magnitude::VI,
        }
    }

    /// Human-readable label.
    pub fn label(&self) -> &str {
        match self {
            Magnitude::I => "Hyperactive",
            Magnitude::II => "Very Active",
            Magnitude::III => "Active",
            Magnitude::IV => "Moderate",
            Magnitude::V => "Quiet",
            Magnitude::VI => "Dormant",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_creation() {
        let s = Star::new("agent-1", 0.8, (1.0, 2.0));
        assert_eq!(s.name, "agent-1");
        assert!((s.brightness - 0.8).abs() < 1e-9);
    }

    #[test]
    fn test_star_symbol() {
        assert_eq!(Star::new("a", 0.95, (0.0, 0.0)).symbol(), '★');
        assert_eq!(Star::new("b", 0.6, (0.0, 0.0)).symbol(), '✦');
        assert_eq!(Star::new("c", 0.25, (0.0, 0.0)).symbol(), '·');
        assert_eq!(Star::new("d", 0.05, (0.0, 0.0)).symbol(), '.');
    }

    #[test]
    fn test_constellation_add_stars() {
        let mut c = Constellation::new("team-a");
        let s1 = c.add_star(Star::new("a1", 0.8, (0.0, 0.0)));
        let s2 = c.add_star(Star::new("a2", 0.6, (1.0, 0.0)));
        assert_eq!(s1, 0);
        assert_eq!(s2, 1);
        assert_eq!(c.stars.len(), 2);
    }

    #[test]
    fn test_constellation_edges() {
        let mut c = Constellation::new("team");
        let s1 = c.add_star(Star::new("a", 0.5, (0.0, 0.0)));
        let s2 = c.add_star(Star::new("b", 0.5, (1.0, 0.0)));
        c.add_edge(s1, s2);
        assert_eq!(c.edges.len(), 1);
        // Invalid edge should be ignored
        c.add_edge(0, 5);
        assert_eq!(c.edges.len(), 1);
    }

    #[test]
    fn test_constellation_find_star() {
        let mut c = Constellation::new("team");
        c.add_star(Star::new("x", 0.5, (0.0, 0.0)));
        assert_eq!(c.find_star("x"), Some(0));
        assert_eq!(c.find_star("y"), None);
    }

    #[test]
    fn test_constellation_total_brightness() {
        let mut c = Constellation::new("team");
        c.add_star(Star::new("a", 0.4, (0.0, 0.0)));
        c.add_star(Star::new("b", 0.6, (0.0, 0.0)));
        assert!((c.total_brightness() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_star_chart_find() {
        let mut chart = StarChart::new();
        let mut c = Constellation::new("team");
        c.add_star(Star::new("alpha", 0.9, (0.0, 0.0)));
        chart.add_constellation(c);
        assert_eq!(chart.find_star("alpha"), Some((0, 0)));
        assert_eq!(chart.find_star("missing"), None);
    }

    #[test]
    fn test_star_chart_total() {
        let mut chart = StarChart::new();
        let mut c1 = Constellation::new("t1");
        c1.add_star(Star::new("a", 0.5, (0.0, 0.0)));
        c1.add_star(Star::new("b", 0.5, (0.0, 0.0)));
        let mut c2 = Constellation::new("t2");
        c2.add_star(Star::new("c", 0.5, (0.0, 0.0)));
        chart.add_constellation(c1);
        chart.add_constellation(c2);
        assert_eq!(chart.total_stars(), 3);
    }

    #[test]
    fn test_ascii_render() {
        let mut chart = StarChart::new();
        let mut c = Constellation::new("fleet");
        c.add_star(Star::new("ship", 0.9, (0.0, 0.0)));
        chart.add_constellation(c);
        let rendered = chart.render_ascii();
        assert!(rendered.contains("fleet"));
        assert!(rendered.contains("ship"));
    }

    #[test]
    fn test_navigation_same_star() {
        let mut chart = StarChart::new();
        let mut c = Constellation::new("team");
        c.add_star(Star::new("a", 0.5, (0.0, 0.0)));
        c.add_star(Star::new("b", 0.5, (1.0, 0.0)));
        c.add_edge(0, 1);
        chart.add_constellation(c);
        let nav = Navigation::new(&chart);
        let path = nav.find_path("a", "a").unwrap();
        assert_eq!(path.len(), 1);
    }

    #[test]
    fn test_navigation_path() {
        let mut chart = StarChart::new();
        let mut c = Constellation::new("team");
        c.add_star(Star::new("a", 0.5, (0.0, 0.0)));
        c.add_star(Star::new("b", 0.5, (1.0, 0.0)));
        c.add_edge(0, 1);
        chart.add_constellation(c);
        let nav = Navigation::new(&chart);
        let path = nav.find_path("a", "b").unwrap();
        assert_eq!(path.len(), 2);
    }

    #[test]
    fn test_navigation_no_path() {
        let mut chart = StarChart::new();
        let mut c = Constellation::new("team");
        c.add_star(Star::new("a", 0.5, (0.0, 0.0)));
        c.add_star(Star::new("b", 0.5, (5.0, 5.0)));
        // No edge
        chart.add_constellation(c);
        let nav = Navigation::new(&chart);
        assert!(nav.find_path("a", "b").is_none());
    }

    #[test]
    fn test_navigation_distance() {
        let mut chart = StarChart::new();
        let mut c = Constellation::new("team");
        c.add_star(Star::new("a", 0.5, (0.0, 0.0)));
        c.add_star(Star::new("b", 0.5, (3.0, 4.0)));
        chart.add_constellation(c);
        let nav = Navigation::new(&chart);
        let d = nav.distance("a", "b").unwrap();
        assert!((d - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_magnitude_scale() {
        assert_eq!(Magnitude::from_brightness(0.95), Magnitude::I);
        assert_eq!(Magnitude::from_brightness(0.75), Magnitude::II);
        assert_eq!(Magnitude::from_brightness(0.55), Magnitude::III);
        assert_eq!(Magnitude::from_brightness(0.35), Magnitude::IV);
        assert_eq!(Magnitude::from_brightness(0.15), Magnitude::V);
        assert_eq!(Magnitude::from_brightness(0.05), Magnitude::VI);
    }

    #[test]
    fn test_magnitude_labels() {
        assert_eq!(Magnitude::I.label(), "Hyperactive");
        assert_eq!(Magnitude::VI.label(), "Dormant");
    }
}
