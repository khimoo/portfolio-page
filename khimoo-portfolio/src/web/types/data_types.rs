use super::node_types::*;
use super::physics_types::Position;
use crate::config::NodeConfig;
use std::collections::HashMap;

/// ノードレジストリ - ノード管理の中心的な構造体
#[derive(Debug, Clone, PartialEq)]
pub struct NodeRegistry {
    pub positions: HashMap<NodeId, Position>,
    pub radii: HashMap<NodeId, i32>,
    pub contents: HashMap<NodeId, NodeContent>,
    pub edges: Vec<(NodeId, NodeId)>,
    pub node_types: HashMap<NodeId, NodeType>,
    pub connection_lines: Vec<ConnectionLine>,
    pub node_importance: HashMap<NodeId, u8>,
    pub node_inbound_counts: HashMap<NodeId, usize>,
    pub node_config: NodeConfig,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
            radii: HashMap::new(),
            contents: HashMap::new(),
            edges: Vec::new(),
            node_types: HashMap::new(),
            connection_lines: Vec::new(),
            node_importance: HashMap::new(),
            node_inbound_counts: HashMap::new(),
            node_config: NodeConfig::default(),
        }
    }

    pub fn new_with_config(node_config: NodeConfig) -> Self {
        Self {
            positions: HashMap::new(),
            radii: HashMap::new(),
            contents: HashMap::new(),
            edges: Vec::new(),
            node_types: HashMap::new(),
            connection_lines: Vec::new(),
            node_importance: HashMap::new(),
            node_inbound_counts: HashMap::new(),
            node_config,
        }
    }

    pub fn add_node(&mut self, id: NodeId, pos: Position, radius: i32, content: NodeContent) {
        let node_type = content.get_node_type();
        self.positions.insert(id, pos);
        self.radii.insert(id, radius);
        self.contents.insert(id, content);
        self.node_types.insert(id, node_type);
    }

    pub fn add_author_node(
        &mut self,
        pos: Position,
        name: String,
        image_url: String,
        bio: Option<String>,
    ) {
        let content = NodeContent::Author {
            name,
            image_url,
            bio,
        };
        let radius = self.node_config.author_node_radius;
        self.add_node(AUTHOR_NODE_ID, pos, radius, content);
    }

    pub fn get_node_type(&self, id: NodeId) -> Option<&NodeType> {
        self.node_types.get(&id)
    }

    pub fn is_author_node(&self, id: NodeId) -> bool {
        matches!(self.get_node_type(id), Some(NodeType::Author))
    }

    pub fn get_author_node_id(&self) -> Option<NodeId> {
        self.node_types
            .iter()
            .find(|(_, node_type)| matches!(node_type, NodeType::Author))
            .map(|(id, _)| *id)
    }

    pub fn add_connection_line(
        &mut self,
        from: NodeId,
        to: NodeId,
        connection_type: ConnectionLineType,
        strength: f32,
    ) {
        let line = ConnectionLine {
            from,
            to,
            connection_type,
            strength,
            visible: true,
        };
        self.connection_lines.push(line);
    }

    pub fn get_connection_lines(&self) -> &Vec<ConnectionLine> {
        &self.connection_lines
    }

    pub fn set_connection_line_visibility(&mut self, visible: bool) {
        for line in &mut self.connection_lines {
            line.visible = visible;
        }
    }

    pub fn update_node_radius(&mut self, node_id: NodeId, new_radius: i32) {
        self.radii.insert(node_id, new_radius);
    }

    pub fn get_node_importance(&self, node_id: NodeId) -> Option<u8> {
        self.node_importance.get(&node_id).copied()
    }

    pub fn set_node_importance(&mut self, node_id: NodeId, importance: u8) {
        self.node_importance.insert(node_id, importance);
    }

    pub fn calculate_dynamic_radius(
        &self,
        node_id: NodeId,
        importance: Option<u8>,
        inbound_count: usize,
    ) -> i32 {
        let base_size = if self.is_author_node(node_id) {
            self.node_config.author_node_radius
        } else {
            self.node_config.default_node_radius
        };

        if self.is_author_node(node_id) {
            return base_size;
        }

        let importance_multiplier = importance.unwrap_or(self.node_config.default_importance) as i32;
        let importance_bonus = (importance_multiplier - self.node_config.default_importance as i32) * self.node_config.importance_multiplier;

        // let inbound_multiplier = (inbound_count as f32).sqrt() as i32;
        // let inbound_bonus = inbound_multiplier * self.node_config.inbound_link_multiplier;

        // let calculated_size = base_size + importance_bonus + inbound_bonus;
        let calculated_size = base_size + importance_bonus;
        calculated_size.clamp(self.node_config.min_node_radius, self.node_config.max_node_radius)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&NodeId, &Position, &i32, &NodeContent)> {
        self.positions.iter().filter_map(move |(id, pos)| {
            let radius = self.radii.get(id)?;
            let content = self.contents.get(id)?;
            Some((id, pos, radius, content))
        })
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.edges.push((from, to));
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = &(NodeId, NodeId)> {
        self.edges.iter()
    }

    pub fn get_node_inbound_count(&self, node_id: NodeId) -> usize {
        self.node_inbound_counts.get(&node_id).copied().unwrap_or(0)
    }

    pub fn set_node_inbound_count(&mut self, node_id: NodeId, count: usize) {
        self.node_inbound_counts.insert(node_id, count);
    }

    pub fn calculate_physics_radius(&self, node_id: NodeId) -> f32 {
        let visual_radius = self.radii.get(&node_id).copied().unwrap_or(self.node_config.default_node_radius);
        let importance = self.get_node_importance(node_id);

        if let Some(imp) = importance {
            if imp >= self.node_config.high_importance_threshold {
                visual_radius as f32 * self.node_config.physics_radius_multiplier_high_importance
            } else {
                visual_radius as f32 * self.node_config.physics_radius_multiplier_default
            }
        } else {
            visual_radius as f32 * self.node_config.physics_radius_multiplier_default
        }
    }
}
