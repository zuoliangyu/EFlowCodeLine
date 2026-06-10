use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId, TranscriptEntry};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

        // Prefer the model recorded in the transcript over Claude Code's reported
        // display_name. With a relay that maps model names, Claude Code only knows
        // the configured (often Claude) name, while each assistant message in the
        // transcript records the model that actually answered (gpt-*, deepseek-*,
        // glm-*, ...). Fall back to display_name/id when the transcript has no real
        // assistant message yet (e.g. first render of a fresh session).
        let display_name =
            real_model_from_transcript(&input.transcript_path).unwrap_or_else(|| {
                if input.model.display_name.is_empty() {
                    input.model.id.clone()
                } else {
                    input.model.display_name.clone()
                }
            });

        Some(SegmentData {
            primary: display_name,
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Model
    }
}

/// Read the model name from the last real assistant message in the transcript.
/// Returns `None` when the transcript is missing/empty or only contains entries
/// without a usable model (e.g. `<synthetic>` placeholders).
fn real_model_from_transcript<P: AsRef<Path>>(transcript_path: P) -> Option<String> {
    let file = fs::File::open(transcript_path.as_ref()).ok()?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default();

    for line in lines.iter().rev() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(line) {
            if entry.r#type.as_deref() == Some("assistant") {
                if let Some(model) = entry.message.as_ref().and_then(|m| m.model.as_ref()) {
                    if !model.is_empty() && model != "<synthetic>" {
                        return Some(model.clone());
                    }
                }
            }
        }
    }

    None
}
