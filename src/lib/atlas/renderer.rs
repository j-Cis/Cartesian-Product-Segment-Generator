use super::modele::MapProcessedData;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

#[allow(clippy::too_many_arguments)]
pub fn render_frame(
    width: u32,
    height: u32,
    map_data: &MapProcessedData,
    offset_x: f32,
    offset_y: f32,
    zoom: f32,
    rotation_deg: f32,
    bbox_res: &str,
    coastline_res: &str,
    ocean_res: &str,
    rivers_res: &str,
    lakes_res: &str,
    glaciers_res: &str,
    grat_30_res: &str,
    grat_10_res: &str,
    opracowane_parafie: &str,
) -> Image {
    if width == 0 || height == 0 {
        return Image::default();
    }

    let mut pixmap = Pixmap::new(width, height).unwrap();
    pixmap.fill(Color::from_rgba8(30, 30, 35, 255)); // Ciemne, "matriksowe" tło

    let rot_rad = rotation_deg.to_radians();
    let cos_a = rot_rad.cos();
    let sin_a = rot_rad.sin();
    let world_w = width as f32 * zoom;
    let world_h = height as f32 * zoom;
    let pivot_x = world_w / 2.0;
    let pivot_y = world_h / 2.0;

    // --- GENERYCZNA FUNKCJA RYSUJĄCA WARSTWĘ ---
    let mut draw_layer = |linie: &Vec<Vec<(f32, f32)>>, paint: &Paint| {
        let stroke = Stroke {
            width: 1.0,
            ..Default::default()
        };
        for linia in linie {
            if linia.is_empty() {
                continue;
            }
            let mut pb = PathBuilder::new();
            for (i, p) in linia.iter().enumerate() {
                let rot_x = (p.0 * world_w - pivot_x) * cos_a - (p.1 * world_h - pivot_y) * sin_a;
                let rot_y = (p.0 * world_w - pivot_x) * sin_a + (p.1 * world_h - pivot_y) * cos_a;
                let sx = pivot_x + rot_x + offset_x;
                let sy = pivot_y + rot_y + offset_y;
                if i == 0 {
                    pb.move_to(sx, sy);
                } else {
                    pb.line_to(sx, sy);
                }
            }
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, paint, &stroke, Transform::identity(), None);
            }
        }
    };

    // 1. OCEANY (Granatowe kontury z lekkim wypełnieniem koloru)
    let mut p_ocean = Paint::default();
    p_ocean.set_color_rgba8(20, 40, 70, 255);
    p_ocean.anti_alias = true;
    match ocean_res {
        "110m" => draw_layer(&map_data.ocean_110m, &p_ocean),
        "50m" => draw_layer(&map_data.ocean_50m, &p_ocean),
        "10m" => draw_layer(&map_data.ocean_10m, &p_ocean),
        _ => {}
    }

    // 2. SIATKI KARTOGRAFICZNE (Półprzezroczysty, delikatny biały)
    let mut p_grat = Paint::default();
    p_grat.set_color_rgba8(255, 255, 255, 30);
    p_grat.anti_alias = true;
    match grat_30_res {
        "110m" => draw_layer(&map_data.graticules_30_110m, &p_grat),
        "50m" => draw_layer(&map_data.graticules_30_50m, &p_grat),
        "10m" => draw_layer(&map_data.graticules_30_10m, &p_grat),
        _ => {}
    }
    match grat_10_res {
        "110m" => draw_layer(&map_data.graticules_10_110m, &p_grat),
        "50m" => draw_layer(&map_data.graticules_10_50m, &p_grat),
        "10m" => draw_layer(&map_data.graticules_10_10m, &p_grat),
        _ => {}
    }

    // 3. RAMKA / BOUNDING BOX (Wyraźna ramka brzegowa projekcji)
    let mut p_bbox = Paint::default();
    p_bbox.set_color_rgba8(255, 255, 255, 100);
    p_bbox.anti_alias = true;
    match bbox_res {
        "110m" => draw_layer(&map_data.bbox_110m, &p_bbox),
        "50m" => draw_layer(&map_data.bbox_50m, &p_bbox),
        "10m" => draw_layer(&map_data.bbox_10m, &p_bbox),
        _ => {}
    }

    // 4. LINIE BRZEGOWE KONTYNENTÓW (Charakterystyczna, Matrixowa Zieleń)
    let mut p_coast = Paint::default();
    p_coast.set_color_rgba8(0, 180, 70, 255);
    p_coast.anti_alias = true;
    match coastline_res {
        "110m" => draw_layer(&map_data.coastline_110m, &p_coast),
        "50m" => draw_layer(&map_data.coastline_50m, &p_coast),
        "10m" => draw_layer(&map_data.coastline_10m, &p_coast),
        _ => {}
    }

    // 5. LÓD I LODOWCE (Mroźny błękit/biel)
    let mut p_ice = Paint::default();
    p_ice.set_color_rgba8(180, 220, 255, 150);
    p_ice.anti_alias = true;
    match glaciers_res {
        "110m" => draw_layer(&map_data.glaciers_110m, &p_ice),
        "50m" => draw_layer(&map_data.glaciers_50m, &p_ice),
        "10m" => draw_layer(&map_data.glaciers_10m, &p_ice),
        _ => {}
    }

    // 6. RZEKI I JEZIORA (Czysta, przezroczysta woda)
    let mut p_water = Paint::default();
    p_water.set_color_rgba8(60, 160, 255, 180);
    p_water.anti_alias = true;
    match rivers_res {
        "110m" => draw_layer(&map_data.rivers_110m, &p_water),
        "50m" => draw_layer(&map_data.rivers_50m, &p_water),
        "10m" => draw_layer(&map_data.rivers_10m, &p_water),
        _ => {}
    }
    match lakes_res {
        "110m" => draw_layer(&map_data.lakes_110m, &p_water),
        "50m" => draw_layer(&map_data.lakes_50m, &p_water),
        "10m" => draw_layer(&map_data.lakes_10m, &p_water),
        _ => {}
    }

    // 7. PUNKTY GENETEKI (Na samej górze - czerwone kropki)
    if opracowane_parafie != "brak" {
        let mut p_dot = Paint::default();
        p_dot.set_color_rgba8(255, 0, 85, 255);
        p_dot.anti_alias = true;
        for pt in &map_data.points {
            let rot_x = (pt.x * world_w - pivot_x) * cos_a - (pt.y * world_h - pivot_y) * sin_a;
            let rot_y = (pt.x * world_w - pivot_x) * sin_a + (pt.y * world_h - pivot_y) * cos_a;
            let sx = pivot_x + rot_x + offset_x;
            let sy = pivot_y + rot_y + offset_y;

            if sx >= -5.0
                && sx <= width as f32 + 5.0
                && sy >= -5.0
                && sy <= height as f32 + 5.0
                && let Some(rect) = Rect::from_xywh(sx - 2.0, sy - 2.0, 4.0, 4.0)
            {
                pixmap.fill_rect(rect, &p_dot, Transform::identity(), None);
            }
        }
    }

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(pixmap.data(), width, height);
    Image::from_rgba8(buffer)
}
