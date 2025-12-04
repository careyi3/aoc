use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Empty = 0,
    Obstacle = 1,
    Start = 2,
    End = 3,
    Visited = 4,
    Path = 5,
}

#[derive(Serialize, Deserialize)]
pub struct AnimationData {
    name: String,
    algorithm: String,
    created_at: String,
    grid_config: GridConfig,
    metadata: Metadata,
    frames: Vec<Frame>,
}

#[derive(Serialize, Deserialize)]
pub struct GridConfig {
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    total_frames: usize,
    has_path: bool,
    frame_delay_ms: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Frame {
    step: usize,
    grid: Vec<Vec<u8>>,
    message: String,
    highlighted: Vec<(usize, usize)>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn save_annimation_data(output_name: String, width: usize, height: usize, frames: Vec<Frame>) {
    let animation = AnimationData {
        name: output_name.clone(),
        algorithm: "conways_game_of_life".to_string(),
        created_at: Utc::now().to_rfc3339(),
        grid_config: GridConfig { width, height },
        metadata: Metadata {
            total_frames: frames.len(),
            has_path: false,
            frame_delay_ms: 100.0,
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
