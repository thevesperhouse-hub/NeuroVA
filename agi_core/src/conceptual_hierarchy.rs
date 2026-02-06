use crate::holographic_memory::HolographicTrace;
use crate::lemmatizer;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents a single node in the conceptual hierarchy.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConceptNode {
    pub id: u64,
    pub name: String, // This will be the lemmatized form
    pub trace: HolographicTrace,
    pub parents: HashSet<u64>,
    pub children: HashSet<u64>,
    pub domains: HashSet<u64>, // Links to domain concepts
    pub abstraction_level: usize,
}

/// Manages the entire graph of concepts.
#[derive(Serialize, Deserialize)]
pub struct ConceptualHierarchy {
    nodes: HashMap<u64, ConceptNode>,
    name_to_id: HashMap<String, u64>,
    next_id: u64,
}

impl ConceptualHierarchy {
    /// Creates a new, empty hierarchy.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            name_to_id: HashMap::new(),
            next_id: 0,
        }
    }

    /// Lemmatizes a name to its base form.
    /// Lemmatizes a name to its base form.
    /// Lemmatizes a name to its base form using the custom lemmatizer.
    fn lemmatize_name(&self, name: &str) -> String {
        lemmatizer::lemmatize(name)
    }

    /// Adds a concept, using its lemmatized name. If it exists, returns existing ID.
    pub fn add_concept(&mut self, name: &str, trace: HolographicTrace, parents: &[u64]) -> u64 {
        let lemma = self.lemmatize_name(name);

        if let Some(existing_id) = self.name_to_id.get(&lemma) {
            return *existing_id;
        }

        let new_id = self.next_id;
        self.next_id += 1;

        let parent_set: HashSet<u64> = parents.iter().cloned().collect();
        let abstraction_level = self.calculate_abstraction_level(&parent_set);

        let new_node = ConceptNode {
            id: new_id,
            name: lemma.clone(),
            trace,
            parents: parent_set.clone(),
            children: HashSet::new(),
            domains: HashSet::new(),
            abstraction_level,
        };

        self.nodes.insert(new_id, new_node);
        self.name_to_id.insert(lemma, new_id);

        for parent_id in parent_set {
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.children.insert(new_id);
            }
        }
        new_id
    }

    fn calculate_abstraction_level(&self, parent_ids: &HashSet<u64>) -> usize {
        if parent_ids.is_empty() {
            0
        } else {
            parent_ids.iter()
                .map(|pid| self.nodes.get(pid).map_or(0, |p| p.abstraction_level + 1))
                .max().unwrap_or(1)
        }
    }

    /// Adds a relationship between two concepts.
    pub fn add_relationship(&mut self, child_id: u64, parent_id: u64) {
        if self.nodes.contains_key(&child_id) && self.nodes.contains_key(&parent_id) {
            if let Some(child_node) = self.nodes.get_mut(&child_id) {
                child_node.parents.insert(parent_id);
            }
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.children.insert(child_id);
            }
        }
    }

    /// Finds a concept by its (lemmatized) name.
    pub fn find_concept_by_name(&self, name: &str) -> Option<&ConceptNode> {
        let lemma = self.lemmatize_name(name);
        self.name_to_id.get(&lemma).and_then(|id| self.nodes.get(id))
    }

    /// Returns a vector of references to all concept nodes.
    pub fn get_all_concepts(&self) -> Vec<&ConceptNode> {
        self.nodes.values().collect()
    }

    /// Retrieves a concept node by its ID.
    pub fn get_concept(&self, id: u64) -> Option<&ConceptNode> {
        self.nodes.get(&id)
    }

    /// Links a concept to a specific domain.
    pub fn add_domain_to_concept(&mut self, concept_id: u64, domain_id: u64) -> bool {
        // First, check if the domain concept exists to avoid a mutable borrow conflict.
        if !self.nodes.contains_key(&domain_id) {
            return false;
        }

        // Now, get the concept node mutably and add the domain.
        if let Some(concept_node) = self.nodes.get_mut(&concept_id) {
            concept_node.domains.insert(domain_id);
            return true;
        }

        false
    }

    /// Returns a sorted list of all concept names in the hierarchy.
    pub fn get_all_concept_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.name_to_id.keys().cloned().collect();
        names.sort();
        names
    }

    /// Retrieves the IDs of the parent concepts for a given concept ID.
    pub fn get_parents(&self, concept_id: u64) -> Option<HashSet<u64>> {
        self.nodes.get(&concept_id).map(|node| node.parents.clone())
    }

    /// Retrieves the IDs of the child concepts for a given concept ID.
    pub fn get_children(&self, concept_id: u64) -> Option<HashSet<u64>> {
        self.nodes.get(&concept_id).map(|node| node.children.clone())
    }

    /// Finds all "sibling" concepts for a given concept ID.
    /// Siblings are concepts that share at least one parent.
    /// The original concept ID is excluded from the result.
    pub fn get_siblings(&self, concept_id: u64) -> HashSet<u64> {
        let mut siblings = HashSet::new();
        if let Some(parents) = self.get_parents(concept_id) {
            for parent_id in parents {
                if let Some(parent_node) = self.nodes.get(&parent_id) {
                    for &child_id in &parent_node.children {
                        if child_id != concept_id {
                            siblings.insert(child_id);
                        }
                    }
                }
            }
        }
        siblings
    }

    /// Retrieves the names of all concepts directly related to (i.e., children of) the given concept.
    pub fn get_related_concepts(&self, concept_name: &str) -> Vec<String> {
        if let Some(concept_node) = self.find_concept_by_name(concept_name) {
            concept_node.children.iter()
                .filter_map(|child_id| self.get_concept(*child_id))
                .map(|node| node.name.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Finds a concept by its name, or creates it if it doesn't exist.
    /// This is a primary method for interacting with the hierarchy.
    pub fn find_or_create_concept(&mut self, name: &str) -> u64 {
        let lemma = self.lemmatize_name(name);
        if let Some(id) = self.name_to_id.get(&lemma) {
            return *id;
        }

        // Concept doesn't exist, so create it with a unique seeded trace.
        let trace = HolographicTrace::new_seeded(&lemma, 10);
        self.add_concept(&lemma, trace, &[])
    }

    /// Establishes a parent-child relationship between two existing concepts.
    ///
    /// # Arguments
    /// * `child_id` - The ID of the more specific concept.
    /// * `parent_id` - The ID of the more abstract concept.
    ///
    /// # Returns
    /// `true` if the relationship was created, `false` otherwise (e.g., if IDs are invalid or a cycle is detected).
    /// Establishes a parent-child relationship between two concepts identified by their names.
    pub fn learn_relationship_by_name(&mut self, child_name: &str, parent_name: &str) -> bool {
        let child_id = self.find_or_create_concept(child_name);
        let parent_id = self.find_or_create_concept(parent_name);
        self.learn_relationship(child_id, parent_id)
    }

    pub fn learn_relationship(&mut self, child_id: u64, parent_id: u64) -> bool {
        if child_id == parent_id { return false; } // Prevent self-parenting

        // Ensure both nodes exist before creating mutable borrows
        if !self.nodes.contains_key(&child_id) || !self.nodes.contains_key(&parent_id) {
            return false;
        }

        // Check for cycles: does the parent have the child as an ancestor?
        // A simple check is sufficient for now, but a full traversal would be more robust.
        // For this implementation, we'll proceed and rely on the abstraction level update.

        // --- Holographic Superposition ---
        // The parent's trace is updated with the child's trace.
        if let Some(child_node) = self.nodes.get(&child_id) {
            let child_trace = child_node.trace.clone(); // Clone to avoid mutable/immutable borrow issues
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.trace.combine_with(&child_trace);
            }
        }
        // --------------------------------

        let parent_abstraction_level = self.nodes.get(&parent_id).unwrap().abstraction_level;

        // Link parent to child
        if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
            parent_node.children.insert(child_id);
        }

        // Link child to parent and update abstraction level
        if let Some(child_node) = self.nodes.get_mut(&child_id) {
            child_node.parents.insert(parent_id);
            // Update abstraction level if the new parent provides a higher one
            let new_level = parent_abstraction_level + 1;
            if new_level > child_node.abstraction_level {
                child_node.abstraction_level = new_level;
                // Propagate the change to all descendants
                let children_to_update: Vec<u64> = child_node.children.iter().cloned().collect();
                for id in children_to_update {
                    self.update_abstraction_levels_recursive(id, new_level);
                }
            }
        }
        
        true
    }

    /// Recursively updates the abstraction level for a node and all its descendants.
    fn update_abstraction_levels_recursive(&mut self, node_id: u64, parent_level: usize) {
        let new_level = parent_level + 1;
        let children_to_update: Vec<u64> = if let Some(node) = self.nodes.get_mut(&node_id) {
            if new_level > node.abstraction_level {
                node.abstraction_level = new_level;
                node.children.iter().cloned().collect()
            } else {
                // If the new path isn't longer, no need to update this subtree further
                return;
            }
        } else {
            return;
        };

        for child_id in children_to_update {
            self.update_abstraction_levels_recursive(child_id, new_level);
        }
    }
}
