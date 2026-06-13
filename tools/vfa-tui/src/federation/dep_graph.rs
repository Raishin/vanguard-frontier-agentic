//! Dependency graph builder and query engine (Tasks 7.12 / 7.13).
//!
//! Builds an in-memory directed graph from catalog data and exposes:
//! - upstream / downstream traversal
//! - blast-radius analysis
//! - cycle detection
//! - ASCII-tree rendering
//! - JSON adjacency-list export

#![deny(warnings)]

use std::collections::{HashMap, HashSet, VecDeque};

use serde::Serialize;

use crate::catalog::store::{CatalogStore, EdgeKind};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Asset category used to annotate graph nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    Agent,
    Skill,
    Role,
    Mcp,
    Rule,
    Unknown,
}

/// A node in the dependency graph.
#[derive(Debug, Clone, Serialize)]
pub struct DependencyNode {
    pub id: String,
    pub asset_type: AssetType,
}

/// The logical relationship that an edge encodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EdgeType {
    /// Agent → Skill via `companion_skills`.
    DependsOn,
    /// Role → Agent via role membership.
    Contains,
    /// Agent → MCP reference.
    References,
    /// Agent → Rule (shared harness).
    Configures,
}

impl EdgeType {
    fn from_edge_kind(kind: &EdgeKind) -> Self {
        match kind {
            EdgeKind::AgentSkill => EdgeType::DependsOn,
            EdgeKind::RoleAgent => EdgeType::Contains,
            EdgeKind::AgentMcp => EdgeType::References,
            EdgeKind::AgentRule => EdgeType::Configures,
        }
    }
}

/// A directed edge between two assets.
#[derive(Debug, Clone, Serialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
}

/// What breaks when an asset is removed.
#[derive(Debug, Clone, Default)]
pub struct BlastRadius {
    /// Direct and transitive dependents of the asset.
    pub dependents: Vec<String>,
}

/// Built dependency graph.
pub struct DependencyGraph {
    pub nodes: HashMap<String, DependencyNode>,
    pub edges: Vec<DependencyEdge>,
}

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

impl DependencyGraph {
    /// Build the graph from a loaded [`CatalogStore`].
    pub fn build(store: &CatalogStore) -> Self {
        let mut nodes: HashMap<String, DependencyNode> = HashMap::new();

        // Seed node type from known collections
        for a in &store.agents {
            nodes.insert(
                a.id.clone(),
                DependencyNode {
                    id: a.id.clone(),
                    asset_type: AssetType::Agent,
                },
            );
        }
        for s in &store.skills {
            nodes.insert(
                s.id.clone(),
                DependencyNode {
                    id: s.id.clone(),
                    asset_type: AssetType::Skill,
                },
            );
        }
        for (rid, _) in &store.roles {
            nodes.insert(
                rid.clone(),
                DependencyNode {
                    id: rid.clone(),
                    asset_type: AssetType::Role,
                },
            );
        }
        for m in &store.mcp_refs {
            nodes.insert(
                m.id.clone(),
                DependencyNode {
                    id: m.id.clone(),
                    asset_type: AssetType::Mcp,
                },
            );
        }
        for r in &store.rules {
            nodes.insert(
                r.id.clone(),
                DependencyNode {
                    id: r.id.clone(),
                    asset_type: AssetType::Rule,
                },
            );
        }

        let raw_edges = store.dependency_edges();
        let mut edges: Vec<DependencyEdge> = Vec::with_capacity(raw_edges.len());

        for (from, to, kind) in &raw_edges {
            // Ensure both endpoints have a node entry even if not in the named
            // collections (defensive).
            nodes.entry(from.clone()).or_insert_with(|| DependencyNode {
                id: from.clone(),
                asset_type: AssetType::Unknown,
            });
            nodes.entry(to.clone()).or_insert_with(|| DependencyNode {
                id: to.clone(),
                asset_type: AssetType::Unknown,
            });

            edges.push(DependencyEdge {
                from: from.clone(),
                to: to.clone(),
                edge_type: EdgeType::from_edge_kind(kind),
            });
        }

        // Deterministic ordering
        edges.sort_by(|a, b| a.from.cmp(&b.from).then(a.to.cmp(&b.to)));

        DependencyGraph { nodes, edges }
    }

    // -----------------------------------------------------------------------
    // Traversal
    // -----------------------------------------------------------------------

    /// All edges that originate *from* `asset_id` — i.e. what `asset_id` depends on.
    pub fn upstream(&self, asset_id: &str) -> Vec<&DependencyEdge> {
        let mut result: Vec<&DependencyEdge> =
            self.edges.iter().filter(|e| e.from == asset_id).collect();
        result.sort_by(|a, b| a.to.cmp(&b.to));
        result
    }

    /// All edges that point *to* `asset_id` — i.e. what depends on `asset_id`.
    pub fn downstream(&self, asset_id: &str) -> Vec<&DependencyEdge> {
        let mut result: Vec<&DependencyEdge> =
            self.edges.iter().filter(|e| e.to == asset_id).collect();
        result.sort_by(|a, b| a.from.cmp(&b.from));
        result
    }

    /// Convenience: sorted list of IDs that `asset_id` depends on (upstream nodes).
    pub fn upstream_ids(&self, asset_id: &str) -> Vec<String> {
        let mut ids: Vec<String> = self
            .upstream(asset_id)
            .iter()
            .map(|e| e.to.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Convenience: sorted list of IDs that depend on `asset_id` (downstream nodes).
    pub fn downstream_ids(&self, asset_id: &str) -> Vec<String> {
        let mut ids: Vec<String> = self
            .downstream(asset_id)
            .iter()
            .map(|e| e.from.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Transitive blast-radius: which assets break if `asset_id` is removed?
    ///
    /// Walks backwards through the graph (BFS over `downstream` edges) and
    /// returns every transitive dependent, sorted and deduplicated.
    pub fn blast_radius(&self, asset_id: &str) -> BlastRadius {
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(asset_id.to_string());

        while let Some(current) = queue.pop_front() {
            for edge in self.downstream(&current) {
                if visited.insert(edge.from.clone()) {
                    queue.push_back(edge.from.clone());
                }
            }
        }

        // Exclude the queried asset itself from the result set
        visited.remove(asset_id);

        let mut dependents: Vec<String> = visited.into_iter().collect();
        dependents.sort();
        BlastRadius { dependents }
    }

    // -----------------------------------------------------------------------
    // Cycle detection
    // -----------------------------------------------------------------------

    /// Detect all cycles in the graph using iterative DFS.
    ///
    /// Returns a list of cycles where each cycle is represented as an ordered
    /// list of node IDs forming the cycle path.  The result is deterministically
    /// sorted so tests can make stable assertions.
    pub fn find_cycles(&self) -> Vec<Vec<String>> {
        // Build adjacency list
        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        for edge in &self.edges {
            adj.entry(edge.from.as_str())
                .or_default()
                .push(edge.to.as_str());
        }
        // Sort each adjacency list for determinism
        for neighbors in adj.values_mut() {
            neighbors.sort_unstable();
        }

        let mut all_nodes: Vec<&str> = self.nodes.keys().map(|s| s.as_str()).collect();
        all_nodes.sort_unstable();

        // Standard DFS with three-color marking (white=0, grey=1, black=2)
        let mut color: HashMap<&str, u8> = HashMap::new();
        let mut path: Vec<String> = Vec::new();
        let mut cycles: Vec<Vec<String>> = Vec::new();

        for &start in &all_nodes {
            if color.get(start).copied().unwrap_or(0) == 0 {
                Self::dfs_cycles(start, &adj, &mut color, &mut path, &mut cycles);
            }
        }

        // Deduplicate and sort for determinism
        for c in &mut cycles {
            // Normalize: rotate so the lexicographically smallest element is first
            if let Some(min_pos) = c
                .iter()
                .enumerate()
                .min_by_key(|(_, v)| v.as_str())
                .map(|(i, _)| i)
            {
                c.rotate_left(min_pos);
            }
        }
        cycles.sort();
        cycles.dedup();
        cycles
    }

    fn dfs_cycles<'a>(
        node: &'a str,
        adj: &HashMap<&'a str, Vec<&'a str>>,
        color: &mut HashMap<&'a str, u8>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        color.insert(node, 1); // grey — in current path
        path.push(node.to_string());

        if let Some(neighbors) = adj.get(node) {
            for &neighbor in neighbors {
                let c = color.get(neighbor).copied().unwrap_or(0);
                if c == 1 {
                    // Found a back-edge → cycle
                    if let Some(pos) = path.iter().position(|s| s == neighbor) {
                        let cycle: Vec<String> = path[pos..].to_vec();
                        cycles.push(cycle);
                    }
                } else if c == 0 {
                    Self::dfs_cycles(neighbor, adj, color, path, cycles);
                }
            }
        }

        path.pop();
        color.insert(node, 2); // black — fully processed
    }

    // -----------------------------------------------------------------------
    // Rendering
    // -----------------------------------------------------------------------

    /// Render an ASCII dependency tree rooted at `focus_id`.
    ///
    /// Each level is indented by 2 spaces per depth.  Nodes already visited
    /// in the current path are marked `[cycle]` to avoid infinite output.
    pub fn render_ascii_tree(&self, focus_id: &str, max_depth: usize) -> String {
        let mut out = String::new();
        let mut visited: HashSet<String> = HashSet::new();
        Self::render_node(self, focus_id, 0, max_depth, &mut visited, &mut out);
        out
    }

    fn render_node(
        graph: &DependencyGraph,
        node_id: &str,
        depth: usize,
        max_depth: usize,
        visited: &mut HashSet<String>,
        out: &mut String,
    ) {
        let prefix = "  ".repeat(depth);
        if visited.contains(node_id) {
            out.push_str(&format!("{}{} [cycle]\n", prefix, node_id));
            return;
        }
        out.push_str(&format!("{}{}\n", prefix, node_id));

        if depth >= max_depth {
            return;
        }

        visited.insert(node_id.to_string());

        // Children = nodes this asset depends on (upstream direction in visual tree)
        let mut children: Vec<String> = graph
            .edges
            .iter()
            .filter(|e| e.from == node_id)
            .map(|e| e.to.clone())
            .collect();
        children.sort();
        children.dedup();

        for child in &children {
            Self::render_node(graph, child, depth + 1, max_depth, visited, out);
        }

        visited.remove(node_id);
    }

    // -----------------------------------------------------------------------
    // JSON export
    // -----------------------------------------------------------------------

    /// Export graph as a JSON adjacency list suitable for `--report dependencies`.
    ///
    /// Shape:
    /// ```json
    /// {
    ///   "nodes": [ { "id": "...", "asset_type": "..." }, ... ],
    ///   "adjacency": {
    ///     "<id>": [ { "to": "...", "edge_type": "depends-on" }, ... ],
    ///     ...
    ///   }
    /// }
    /// ```
    pub fn to_adjacency_json(&self) -> serde_json::Value {
        // Sorted nodes
        let mut node_ids: Vec<&str> = self.nodes.keys().map(|s| s.as_str()).collect();
        node_ids.sort_unstable();
        let nodes_array: Vec<serde_json::Value> = node_ids
            .iter()
            .map(|id| {
                let node = &self.nodes[*id];
                serde_json::json!({
                    "id": node.id,
                    "asset_type": node.asset_type,
                })
            })
            .collect();

        // Build adjacency map: from_id → sorted list of {to, edge_type}
        let mut adj_map: HashMap<&str, Vec<serde_json::Value>> = HashMap::new();
        for edge in &self.edges {
            adj_map
                .entry(edge.from.as_str())
                .or_default()
                .push(serde_json::json!({
                    "to": edge.to,
                    "edge_type": edge.edge_type,
                }));
        }
        // Sort each list by "to"
        for v in adj_map.values_mut() {
            v.sort_by(|a, b| {
                a["to"]
                    .as_str()
                    .unwrap_or("")
                    .cmp(b["to"].as_str().unwrap_or(""))
            });
        }

        // Build sorted adjacency object
        let mut adj_keys: Vec<&str> = adj_map.keys().copied().collect();
        adj_keys.sort_unstable();
        let mut adjacency = serde_json::Map::new();
        for key in adj_keys {
            adjacency.insert(
                key.to_string(),
                serde_json::Value::Array(adj_map[key].clone()),
            );
        }

        serde_json::json!({
            "nodes": nodes_array,
            "adjacency": adjacency,
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Helpers to build minimal graphs without a real CatalogStore
    // -----------------------------------------------------------------------

    /// Build a DependencyGraph directly from a list of edges for unit tests.
    fn graph_from_edges(edges: Vec<(String, String, EdgeType)>) -> DependencyGraph {
        let mut nodes: HashMap<String, DependencyNode> = HashMap::new();
        let mut dep_edges: Vec<DependencyEdge> = Vec::new();

        for (from, to, etype) in edges {
            nodes.entry(from.clone()).or_insert_with(|| DependencyNode {
                id: from.clone(),
                asset_type: AssetType::Unknown,
            });
            nodes.entry(to.clone()).or_insert_with(|| DependencyNode {
                id: to.clone(),
                asset_type: AssetType::Unknown,
            });
            dep_edges.push(DependencyEdge {
                from,
                to,
                edge_type: etype,
            });
        }
        dep_edges.sort_by(|a, b| a.from.cmp(&b.from).then(a.to.cmp(&b.to)));
        DependencyGraph {
            nodes,
            edges: dep_edges,
        }
    }

    // -----------------------------------------------------------------------
    // Basic correctness tests
    // -----------------------------------------------------------------------

    #[test]
    fn upstream_downstream_are_inverses() {
        let g = graph_from_edges(vec![
            ("agent-a".into(), "skill-x".into(), EdgeType::DependsOn),
            ("agent-a".into(), "skill-y".into(), EdgeType::DependsOn),
            ("role-r".into(), "agent-a".into(), EdgeType::Contains),
        ]);

        // upstream of agent-a → {skill-x, skill-y}
        let up = g.upstream_ids("agent-a");
        assert_eq!(up, vec!["skill-x", "skill-y"]);

        // downstream of agent-a → {role-r}
        let down = g.downstream_ids("agent-a");
        assert_eq!(down, vec!["role-r"]);

        // upstream of skill-x → nothing
        assert!(g.upstream_ids("skill-x").is_empty());

        // downstream of skill-x → {agent-a}
        assert_eq!(g.downstream_ids("skill-x"), vec!["agent-a"]);
    }

    #[test]
    fn blast_radius_transitive() {
        // role-r → agent-a → skill-x
        let g = graph_from_edges(vec![
            ("agent-a".into(), "skill-x".into(), EdgeType::DependsOn),
            ("role-r".into(), "agent-a".into(), EdgeType::Contains),
        ]);

        // Removing skill-x should implicate agent-a and role-r
        let br = g.blast_radius("skill-x");
        assert!(br.dependents.contains(&"agent-a".to_string()));
        assert!(br.dependents.contains(&"role-r".to_string()));
        assert!(!br.dependents.contains(&"skill-x".to_string()));
    }

    #[test]
    fn blast_radius_direct_only() {
        let g = graph_from_edges(vec![(
            "agent-a".into(),
            "skill-x".into(),
            EdgeType::DependsOn,
        )]);
        let br = g.blast_radius("skill-x");
        assert_eq!(br.dependents, vec!["agent-a"]);
    }

    #[test]
    fn find_cycles_detects_simple_cycle() {
        // a → b → c → a
        let g = graph_from_edges(vec![
            ("a".into(), "b".into(), EdgeType::DependsOn),
            ("b".into(), "c".into(), EdgeType::DependsOn),
            ("c".into(), "a".into(), EdgeType::DependsOn),
        ]);
        let cycles = g.find_cycles();
        assert!(!cycles.is_empty(), "expected at least one cycle");
        // All nodes in the graph should be part of a cycle
        let flat: HashSet<String> = cycles.iter().flatten().cloned().collect();
        assert!(flat.contains("a"));
        assert!(flat.contains("b"));
        assert!(flat.contains("c"));
    }

    #[test]
    fn find_cycles_dag_no_cycle() {
        let g = graph_from_edges(vec![
            ("role-r".into(), "agent-a".into(), EdgeType::Contains),
            ("agent-a".into(), "skill-x".into(), EdgeType::DependsOn),
        ]);
        assert!(g.find_cycles().is_empty());
    }

    #[test]
    fn render_ascii_tree_basic() {
        let g = graph_from_edges(vec![
            ("agent-a".into(), "skill-x".into(), EdgeType::DependsOn),
            ("agent-a".into(), "skill-y".into(), EdgeType::DependsOn),
        ]);
        let tree = g.render_ascii_tree("agent-a", 5);
        assert!(tree.contains("agent-a"));
        assert!(tree.contains("skill-x"));
        assert!(tree.contains("skill-y"));
    }

    #[test]
    fn adjacency_json_structure() {
        let g = graph_from_edges(vec![(
            "agent-a".into(),
            "skill-x".into(),
            EdgeType::DependsOn,
        )]);
        let json = g.to_adjacency_json();
        assert!(json["nodes"].is_array());
        assert!(json["adjacency"].is_object());
        let edges = &json["adjacency"]["agent-a"];
        assert_eq!(edges[0]["to"].as_str().unwrap(), "skill-x");
        assert_eq!(edges[0]["edge_type"].as_str().unwrap(), "depends-on");
    }

    // -----------------------------------------------------------------------
    // Property tests (proptest) — Property 15
    // -----------------------------------------------------------------------

    use proptest::prelude::*;

    /// Generate a random acyclic graph topology.
    ///
    /// `n_agents`, `n_skills`, `n_roles` are node counts; edges are drawn
    /// from a fixed seed strategy to keep the graph a DAG.
    fn arb_dag_edges(
        n_agents: usize,
        n_skills: usize,
        n_roles: usize,
    ) -> Vec<(String, String, EdgeType)> {
        let agents: Vec<String> = (0..n_agents).map(|i| format!("agent-{i}")).collect();
        let skills: Vec<String> = (0..n_skills).map(|i| format!("skill-{i}")).collect();
        let roles: Vec<String> = (0..n_roles).map(|i| format!("role-{i}")).collect();

        let mut edges = Vec::new();
        // Each agent depends on skills whose index ≤ agent index (keeps DAG)
        for (ai, agent) in agents.iter().enumerate() {
            for (si, skill) in skills.iter().enumerate() {
                if si <= ai {
                    edges.push((agent.clone(), skill.clone(), EdgeType::DependsOn));
                }
            }
        }
        // Each role contains agents whose index ≤ role index
        for (ri, role) in roles.iter().enumerate() {
            for (ai, agent) in agents.iter().enumerate() {
                if ai <= ri {
                    edges.push((role.clone(), agent.clone(), EdgeType::Contains));
                }
            }
        }
        edges
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// Property 15a — upstream/downstream are consistent inverses.
        ///
        /// For every edge (a → b) in the graph:
        ///   b ∈ upstream_ids(a)  AND  a ∈ downstream_ids(b)
        #[test]
        fn prop_upstream_downstream_inverses(
            n_agents in 1usize..5,
            n_skills in 1usize..5,
            n_roles in 1usize..5,
        ) {
            let edges = arb_dag_edges(n_agents, n_skills, n_roles);
            let g = graph_from_edges(edges.clone());

            for (from, to, _) in &edges {
                let up = g.upstream_ids(from);
                let down = g.downstream_ids(to);
                prop_assert!(up.contains(to),
                    "{to} should be in upstream({from})");
                prop_assert!(down.contains(from),
                    "{from} should be in downstream({to})");
            }
        }

        /// Property 15b — blast_radius of a skill includes exactly the agents
        /// referencing it (and their roles transitively).
        #[test]
        fn prop_blast_radius_includes_direct_agents(
            n_agents in 1usize..4,
            n_skills in 1usize..4,
            n_roles in 1usize..4,
        ) {
            let edges = arb_dag_edges(n_agents, n_skills, n_roles);
            let g = graph_from_edges(edges);

            for si in 0..n_skills {
                let skill_id = format!("skill-{si}");
                let br = g.blast_radius(&skill_id);
                // Every agent whose index ≥ si depends on this skill
                for ai in si..n_agents {
                    let agent_id = format!("agent-{ai}");
                    prop_assert!(
                        br.dependents.contains(&agent_id),
                        "blast_radius({skill_id}) missing {agent_id}"
                    );
                }
            }
        }

        /// Property 15c — injecting a cycle is reliably detected.
        #[test]
        fn prop_cycle_detected_not_infinite(
            n in 2usize..6,
        ) {
            // Build a chain a0 → a1 → ... → a(n-1) → a0  (a cycle)
            let mut edges: Vec<(String, String, EdgeType)> = (0..n)
                .map(|i| (
                    format!("node-{i}"),
                    format!("node-{}", (i + 1) % n),
                    EdgeType::DependsOn,
                ))
                .collect();
            // Add a DAG tail that does NOT participate in the cycle
            edges.push(("extra".into(), format!("node-{}", n - 1), EdgeType::DependsOn));
            let g = graph_from_edges(edges);
            let cycles = g.find_cycles();
            prop_assert!(!cycles.is_empty(), "cycle must be detected");
        }
    }
}
