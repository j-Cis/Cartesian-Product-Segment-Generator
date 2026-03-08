#[derive(Debug, Clone, Copy)]
pub enum MapProjection {
    ProjDynamic { margin: f64 },
    ProjWgs84,
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

    // 1. RAMKI (BBOX)
    pub bbox_110m: Vec<Vec<(f32, f32)>>,
    pub bbox_50m: Vec<Vec<(f32, f32)>>,
    pub bbox_10m: Vec<Vec<(f32, f32)>>,

    // 2. LINIE BRZEGOWE
    pub coastline_110m: Vec<Vec<(f32, f32)>>,
    pub coastline_50m: Vec<Vec<(f32, f32)>>,
    pub coastline_10m: Vec<Vec<(f32, f32)>>,

    // 3. OCEANY
    pub ocean_110m: Vec<Vec<(f32, f32)>>,
    pub ocean_50m: Vec<Vec<(f32, f32)>>,
    pub ocean_10m: Vec<Vec<(f32, f32)>>,

    // 4. RZEKI
    pub rivers_110m: Vec<Vec<(f32, f32)>>,
    pub rivers_50m: Vec<Vec<(f32, f32)>>,
    pub rivers_10m: Vec<Vec<(f32, f32)>>,

    // 5. JEZIORA
    pub lakes_110m: Vec<Vec<(f32, f32)>>,
    pub lakes_50m: Vec<Vec<(f32, f32)>>,
    pub lakes_10m: Vec<Vec<(f32, f32)>>,

    // 6. LODOWCE
    pub glaciers_110m: Vec<Vec<(f32, f32)>>,
    pub glaciers_50m: Vec<Vec<(f32, f32)>>,
    pub glaciers_10m: Vec<Vec<(f32, f32)>>,

    // 7. SIATKI 30°
    pub graticules_30_110m: Vec<Vec<(f32, f32)>>,
    pub graticules_30_50m: Vec<Vec<(f32, f32)>>,
    pub graticules_30_10m: Vec<Vec<(f32, f32)>>,

    // 8. SIATKI 10°
    pub graticules_10_110m: Vec<Vec<(f32, f32)>>,
    pub graticules_10_50m: Vec<Vec<(f32, f32)>>,
    pub graticules_10_10m: Vec<Vec<(f32, f32)>>,

    pub min_lon: f64,
    pub max_lon: f64,
    pub min_lat: f64,
    pub max_lat: f64,
}
