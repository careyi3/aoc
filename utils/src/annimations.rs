use celluloid_core::{AnimationData, GridConfig, Metadata};
use chrono::Utc;
use std::fs;

pub use celluloid_core::Frame;

pub fn save_annimation_data(
    output_name: String,
    width: usize,
    height: usize,
    frames: Vec<Frame>,
    frame_rate: f64,
) {
    let animation = AnimationData {
        name: output_name.clone(),
        created_at: Utc::now().to_rfc3339(),
        grid_config: GridConfig { width, height },
        metadata: Metadata {
            total_frames: frames.len(),
            has_path: false,
            frame_delay_ms: frame_rate,
        },
        frames,
    };

    let workspace_root = std::env::current_dir().expect("Failed to get current directory");
    let output_path = workspace_root
        .join("animations")
        .join(format!("{}.json", output_name));

    fs::create_dir_all(output_path.parent().unwrap())
        .expect("Failed to create animations directory");

    let json = serde_json::to_string(&animation).expect("Failed to serialize");
    fs::write(&output_path, json).expect("Failed to write file");
}
