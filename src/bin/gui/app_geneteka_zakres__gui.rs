#![windows_subsystem = "windows"]

use anyhow::Result;

use slint::ComponentHandle;
use std::rc::Rc; // UWAGA: Usunąłem stąd VecModel, bo już go nie używamy!

use fifak_lib::atlas::{MapProjection, generate_map_data};
use fifak_lib::setup_window_ctrl_bindings;

slint::include_modules!();

fn main() -> Result<()> {
    let sciezka_danych = "./data/genealodzy-geneteka/raw";

    let baza =
        fifak_lib::geneteka::pobrany_najnowszy::laduj_baze(sciezka_danych).unwrap_or_else(|| {
            println!(
                "Nie znaleziono danych w {}. Tworzę pustą mapę.",
                sciezka_danych
            );
            fifak_lib::geneteka::data_raw_modele::BazaGeneteki { rekord: vec![] }
        });

    println!("Wczytano {} parafii!", baza.rekord.len());

    let ui = AppGenetekaZakres::new()?;
    setup_window_ctrl_bindings!(ui, AppGenetekaZakres);

    // ==========================================
    // LOGIKA MAPY -> przeniesiona do biblioteki atlas!
    // Generujemy dane i od razu pakujemy w Rc, by były dostępne dla silnika rysującego
    //
    // ODKOMENTUJ TO, jeśli chcesz wrócić do dynamicznych marginesów:
    // let map_data = Rc::new(generate_map_data(&baza.rekord, MapProjection::Dynamic { margin: 0.05 }));
    //
    // ZOSTAW TO, jeśli używamy mapy całego świata:
    let map_data = Rc::new(generate_map_data(&baza.rekord, MapProjection::ProjWgs84));
    // ==========================================

    // ------------------ TRANSFER GRANIC DO SLINTA ------------------
    ui.set_geo_min_lon(map_data.min_lon as f32);
    ui.set_geo_max_lon(map_data.max_lon as f32);
    ui.set_geo_min_lat(map_data.min_lat as f32);
    ui.set_geo_max_lat(map_data.max_lat as f32);
    // ---------------------------------------------------------------

    // ==========================================
    // SILNIK RENDERUJĄCY W RUŚCIE (TINY-SKIA)
    // To zastępuje całkowicie pętlę "for pt in map_data.points..."
    // ==========================================
    let ui_handle = ui.as_weak();
    let map_data_render = Rc::clone(&map_data); // Klonujemy referencję dla closure

    ui.on_camera_changed(move |w, h, offset_x, offset_y, zoom, rot| {
        if let Some(ui_ref) = ui_handle.upgrade() {
            // 1. ZCZYTYWANIE WSZYSTKICH WARSTW ZE SLINTA (W CZASIE RZECZYWISTYM)
            let b_res = ui_ref.get_bbox_res().to_string();
            let c_res = ui_ref.get_coastline_res().to_string();
            let o_res = ui_ref.get_ocean_res().to_string();
            let r_res = ui_ref.get_rivers_res().to_string();
            let l_res = ui_ref.get_lakes_res().to_string();
            let g_res = ui_ref.get_glaciers_res().to_string();
            let g30_res = ui_ref.get_graticules_30_res().to_string();
            let g10_res = ui_ref.get_graticules_10_res().to_string();
            let o_parafie = ui_ref.get_opracowane_parafie().to_string();

            // 2. WYSYŁKA DO RENDERERA
            let image_frame = fifak_lib::atlas::renderer::render_frame(
                w as u32,
                h as u32,
                &map_data_render,
                offset_x,
                offset_y,
                zoom,
                rot,
                &b_res,
                &c_res,
                &o_res,
                &r_res,
                &l_res,
                &g_res,
                &g30_res,
                &g10_res,
                &o_parafie,
            );

            ui_ref.set_map_frame(image_frame);
        }
    });
    // ==========================================
    // ==========================================

    ui.on_search(|text| {
        println!("[*] Szukamy: {}", text);
    });

    ui.run()?;
    Ok(())
}
