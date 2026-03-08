use super::modele::{MapProcessedData, MapProjection, NormalizedPoint};
use super::projekcje::{proj_dynamiczna, proj_plate_carree};
use crate::geneteka::data_raw_modele::Rekord;
use crate::pliki::path_data_naturalearth as paths;
use geojson::{GeoJson, Value};
use std::fs;

// --- FUNKCJA POMOCNICZA DO ŁADOWANIA (Bez panikowania, jeśli brak pliku!) ---
fn laduj_warstwe(
    sciezka: &str,
    out: &mut Vec<Vec<(f32, f32)>>,
    min_lon: f64,
    max_lon: f64,
    min_lat: f64,
    max_lat: f64,
) {
    if let Ok(data) = fs::read_to_string(sciezka)
        && let Ok(res) = data.parse::<GeoJson>()
    {
        wyciagnij_linie(&res, out, min_lon, max_lon, min_lat, max_lat);
    }
}

pub fn generate_map_data(records: &[Rekord], projection: MapProjection) -> MapProcessedData {
    let (min_lon, max_lon, min_lat, max_lat) = match projection {
        MapProjection::ProjWgs84 => proj_plate_carree::get_bounds(),
        MapProjection::ProjDynamic { margin } => proj_dynamiczna::get_bounds(records, margin),
    };

    let lon_range = max_lon - min_lon;
    let lat_range = max_lat - min_lat;

    // --- TWORZENIE WEKTORÓW DLA WSZYSTKICH WARSTW ---
    let mut bbox_110m = Vec::new();
    let mut bbox_50m = Vec::new();
    let mut bbox_10m = Vec::new();
    let mut coastline_110m = Vec::new();
    let mut coastline_50m = Vec::new();
    let mut coastline_10m = Vec::new();
    let mut ocean_110m = Vec::new();
    let mut ocean_50m = Vec::new();
    let mut ocean_10m = Vec::new();
    let mut rivers_110m = Vec::new();
    let mut rivers_50m = Vec::new();
    let mut rivers_10m = Vec::new();
    let mut lakes_110m = Vec::new();
    let mut lakes_50m = Vec::new();
    let mut lakes_10m = Vec::new();
    let mut glaciers_110m = Vec::new();
    let mut glaciers_50m = Vec::new();
    let mut glaciers_10m = Vec::new();
    let mut graticules_30_110m = Vec::new();
    let mut graticules_30_50m = Vec::new();
    let mut graticules_30_10m = Vec::new();
    let mut graticules_10_110m = Vec::new();
    let mut graticules_10_50m = Vec::new();
    let mut graticules_10_10m = Vec::new();

    // --- ŁADOWANIE PLIKÓW ---
    laduj_warstwe(
        paths::PATH_BBOX_110M,
        &mut bbox_110m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_BBOX_50M,
        &mut bbox_50m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_BBOX_10M,
        &mut bbox_10m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );

    laduj_warstwe(
        paths::PATH_COASTLINE_110M,
        &mut coastline_110m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_COASTLINE_50M,
        &mut coastline_50m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_COASTLINE_10M,
        &mut coastline_10m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );

    laduj_warstwe(
        paths::PATH_OCEAN_110M,
        &mut ocean_110m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_OCEAN_50M,
        &mut ocean_50m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_OCEAN_10M,
        &mut ocean_10m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );

    laduj_warstwe(
        paths::PATH_RIVERS_110M,
        &mut rivers_110m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_RIVERS_50M,
        &mut rivers_50m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_RIVERS_10M,
        &mut rivers_10m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );

    laduj_warstwe(
        paths::PATH_LAKES_110M,
        &mut lakes_110m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_LAKES_50M,
        &mut lakes_50m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_LAKES_10M,
        &mut lakes_10m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );

    laduj_warstwe(
        paths::PATH_GLACIERS_110M,
        &mut glaciers_110m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_GLACIERS_50M,
        &mut glaciers_50m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_GLACIERS_10M,
        &mut glaciers_10m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );

    laduj_warstwe(
        paths::PATH_GRATICULES_30_110M,
        &mut graticules_30_110m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_GRATICULES_30_50M,
        &mut graticules_30_50m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_GRATICULES_30_10M,
        &mut graticules_30_10m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );

    laduj_warstwe(
        paths::PATH_GRATICULES_10_110M,
        &mut graticules_10_110m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_GRATICULES_10_50M,
        &mut graticules_10_50m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );
    laduj_warstwe(
        paths::PATH_GRATICULES_10_10M,
        &mut graticules_10_10m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    );

    // --- WYLICZANIE PUNKTÓW GENETEKI ---
    let mut points = Vec::new();
    for rek in records {
        if rek.miejsce.lonlat.len() == 2 {
            let lon = rek.miejsce.lonlat[0];
            let lat = rek.miejsce.lonlat[1];
            let name = rek.miejsce.parafia.first().cloned().unwrap_or_default();

            let x_norm = (lon - min_lon) / lon_range;
            let y_norm = (lat - min_lat) / lat_range;

            if (0.0..=1.0).contains(&x_norm) && (0.0..=1.0).contains(&y_norm) {
                points.push(NormalizedPoint {
                    x: x_norm as f32,
                    y: (1.0 - y_norm) as f32,
                    name,
                });
            }
        }
    }

    // --- PAKOWANIE DO ZWRACANEGO STRUKTU ---
    MapProcessedData {
        points,
        bbox_110m,
        bbox_50m,
        bbox_10m,
        coastline_110m,
        coastline_50m,
        coastline_10m,
        ocean_110m,
        ocean_50m,
        ocean_10m,
        rivers_110m,
        rivers_50m,
        rivers_10m,
        lakes_110m,
        lakes_50m,
        lakes_10m,
        glaciers_110m,
        glaciers_50m,
        glaciers_10m,
        graticules_30_110m,
        graticules_30_50m,
        graticules_30_10m,
        graticules_10_110m,
        graticules_10_50m,
        graticules_10_10m,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    }
}

fn wyciagnij_linie(
    gj: &GeoJson,
    out: &mut Vec<Vec<(f32, f32)>>,
    min_lon: f64,
    max_lon: f64,
    min_lat: f64,
    max_lat: f64,
) {
    let lon_range = max_lon - min_lon;
    let lat_range = max_lat - min_lat;

    if let GeoJson::FeatureCollection(fc) = gj {
        for feature in &fc.features {
            if let Some(geom) = &feature.geometry {
                match &geom.value {
                    Value::LineString(ls) => {
                        let coords: Vec<(f32, f32)> = ls
                            .iter()
                            .map(|p| {
                                let x = (p[0] - min_lon) / lon_range;
                                let y = (max_lat - p[1]) / lat_range;
                                (x as f32, y as f32)
                            })
                            .collect();
                        out.push(coords);
                    }
                    Value::MultiLineString(mls) => {
                        for ls in mls {
                            let coords: Vec<(f32, f32)> = ls
                                .iter()
                                .map(|p| {
                                    let x = (p[0] - min_lon) / lon_range;
                                    let y = (max_lat - p[1]) / lat_range;
                                    (x as f32, y as f32)
                                })
                                .collect();
                            out.push(coords);
                        }
                    }
                    Value::Polygon(p) => {
                        for ring in p {
                            let coords: Vec<(f32, f32)> = ring
                                .iter()
                                .map(|p| {
                                    let x = (p[0] - min_lon) / lon_range;
                                    let y = (max_lat - p[1]) / lat_range;
                                    (x as f32, y as f32)
                                })
                                .collect();
                            out.push(coords);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
