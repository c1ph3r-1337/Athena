use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MemoryItem {
    pub id: String,
    pub content: String,
    pub importance: f32,
    pub tags: Vec<String>,
}

pub struct MemoryManager {
    items: HashMap<String, MemoryItem>,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: MemoryItem) {
        self.items.insert(item.id.clone(), item);
    }

    pub fn retrieve(&self, _query: &str) -> Vec<MemoryItem> {
        // Mock retrieval. In a real system this would use embeddings/BM25
        self.items.values().cloned().collect()
    }
}
