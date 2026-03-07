#[derive(Debug, Clone, Copy)]
pub enum MapProjection {
    Dynamic { margin: f64 },
    PlateCarree,
    // W przyszłości:
    // Dymaxion,
    // AuthaGraph,
}

pub struct NormalizedPoint {
    pub x: f32,
    pub y: f32,
    pub name: String,
}

pub struct MapProcessedData {
    pub points: Vec<NormalizedPoint>,
    pub min_lon: f64,
    pub max_lon: f64,
    pub min_lat: f64,
    pub max_lat: f64,
}