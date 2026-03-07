#![windows_subsystem = "windows"]

use anyhow::Result;

use std::rc::Rc;
use slint::{ComponentHandle,  VecModel};

use fifak_lib::window; 
use fifak_lib::setup_window_ctrl_bindings;
use fifak_lib::atlas::{generate_map_data, MapProjection};

slint::include_modules!();

fn main() -> Result<()> {
    let sciezka_danych = "./data/genealodzy-geneteka/raw";

    let baza = fifak_lib::geneteka::pobrany_najnowszy::laduj_baze(sciezka_danych)
        .unwrap_or_else(|| {
            println!("Nie znaleziono danych w {}. Tworzę pustą mapę.", sciezka_danych);
            fifak_lib::geneteka::data_raw_modele::BazaGeneteki { rekord: vec![] }
        });

    println!("Wczytano {} parafii!", baza.rekord.len());

    let ui= GenetekaCoJestWindow::new()?;
    setup_window_ctrl_bindings!(ui);


    // ==========================================
    // LOGIKA MAPY -> przeniesiona do biblioteki atlas!
    // Wywołujemy z parametrem "Dynamic"
    // ==========================================
    let map_data = generate_map_data(&baza.rekord, MapProjection::Dynamic { margin: 0.05 });
    
    // Tłumaczymy niezależne punkty z modułu Atlas na typy rozumiane przez Slint (MapPoint)
    let mut slint_points = Vec::with_capacity(map_data.points.len());
    for pt in map_data.points {
        slint_points.push(MapPoint {
            x: pt.x,
            y: pt.y,
            nazwa: pt.name.into(),
        });
    }

    // Wypychamy wyliczone punkty prosto na front-end do Slinta
    let points_model = Rc::new(VecModel::from(slint_points));
    ui.set_map_points(points_model.into());

    // ------------------ TRANSFER GRANIC DO SLINTA ------------------
    ui.set_geo_min_lon(map_data.min_lon as f32);
    ui.set_geo_max_lon(map_data.max_lon as f32);
    ui.set_geo_min_lat(map_data.min_lat as f32);
    ui.set_geo_max_lat(map_data.max_lat as f32);
    // ---------------------------------------------------------------

    ui.on_search(|text| {
        println!("[*] Szukamy: {}", text);
    });

    ui.run()?;
    Ok(())
}