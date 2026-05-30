use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct ModelSegment;

impl ModelSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for ModelSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let mut metadata = HashMap::new();
        metadata.insert("model_id".to_string(), input.model.id.clone());
        metadata.insert("display_name".to_string(), input.model.display_name.clone());

        // Use Claude Code's display_name directly — no remapping needed.
        // This avoids stale hardcoded mappings when vendors release new models.
        let display_name = if input.model.display_name.is_empty() {
            &input.model.id
        } else {
            &input.model.display_name
        };

        Some(SegmentData {
            primary: display_name.to_string(),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Model
    }
}
