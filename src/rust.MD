# Struktura `./src/` bez `./src/ui2/` i `./src/ui/`

```text
------------------------------------------------
├──┬ 📂 bin/
│  ├──┬ 📂 batch/
│  │  ├── 🦀 app_pobieracz_geneteka_zakres_html__get.rs
│  │  └── 🦀 app_pobieracz_geneteka_zakres_mapa__get.rs
│  ├──┬ 📂 cli/
│  │  └── 🦀 app_kartezjan_produkt__cli.rs
│  ├──┬ 📂 gui/
│  │  ├── 🦀 app_geneteka_zakres__gui.rs
│  │  └── 🦀 app_kartezjan_produkt__gui.rs
│  └──┬ 📂 tui/
│     └── 🦀 app_kartezjan_produkt__tui.rs
├──┬ 📂 lib/
│  ├── 🦀 mod.rs
│  ├── 🦀 window.rs
│  ├──┬ 📂 atlas/
│  │  ├── 🦀 generator.rs
│  │  ├── 🦀 mod.rs
│  │  ├── 🦀 modele.rs
│  │  ├── 🦀 renderer.rs
│  │  └──┬ 📂 projekcje/
│  │     ├── 🦀 mod.rs
│  │     ├── 🦀 proj_dynamiczna.rs
│  │     └── 🦀 proj_plate_carree.rs
│  ├──┬ 📂 cli/
│  │  ├── 🦀 menu.rs
│  │  └── 🦀 mod.rs
│  ├──┬ 📂 geneteka/
│  │  ├── 🦀 data_raw_modele.rs
│  │  ├── 🦀 mod.rs
│  │  ├── 🦀 modele.rs
│  │  ├── 🦀 parser_html.rs
│  │  ├── 🦀 parser_kml.rs
│  │  ├── 🦀 parser_rocznikow.rs
│  │  ├── 🦀 pobieracz_html.rs
│  │  ├── 🦀 pobieracz_kml.rs
│  │  └── 🦀 pobrany_najnowszy.rs
│  ├──┬ 📂 logic/
│  │  ├── 🦀 format.rs
│  │  ├── 🦀 mod.rs
│  │  ├── 🦀 morphology.rs
│  │  └── 🦀 utils.rs
│  └──┬ 📂 pliki/
│     ├── 🦀 mod.rs
│     └── 🦀 path_data_naturalearth.rs
├─── 📂 ui/
└─── 📂 ui2/
------------------------------------------------
```

## RUST `./Cargo.toml`

```toml
[package]
name = "fifakom-genealogia"
version = "0.1.5"
readme = "README.md"
repository = "https://github.com/j-Cis/fifakom-genealogia"
description = "Zestaw narzędzi i sztuczek pomocnych zaawansowanym genealogom."
authors = ["Jan Roman Cisowski „Gepiden”"]
edition = "2024"       # Używana edycja języka Rust (nowoczesne funkcje i resolver)
rust-version = "1.93.1" # Minimalna wersja kompilatora wymagana do zbudowania projektu
resolver = "3"         # Sposób, w jaki Cargo rozwiązuje zależności (3 to standard dla edycji 2024)

# ZALEŻNOŚCI GŁÓWNE (Kompilowane do końcowego programu)
[dependencies]
# --- Narzędzia ogólne i obsługa błędów ---
anyhow = "1.0.102"      # Prosta i elastyczna obsługa błędów (typ anyhow::Result)
itertools = "0.14.0"    # Dodatkowe, potężne metody dla iteratorów (często używane przy kombinacjach/kartezjańskich)
regex = "1.12.3"        # Obsługa wyrażeń regularnych do analizy i formatowania tekstu
chrono = "0.4.44"       # Obsługa dat i czasu (używane np. do dodawania daty do nazw pobieranych plików)

# --- Interfejs Linii Poleceń (CLI) ---
clap = { version = "4.5.60", features = ["derive"] } # Potężny parser argumentów z wiersza poleceń
inquire = "0.9.4"       # Interaktywne prompty/menu w terminalu (pytania, listy wyboru)
colored = "3.1.1"       # Kolorowanie tekstu w konsoli (żeby komunikaty wyróżniały się wizualnie)
atty = "0.2.14"         # Sprawdza, czy program działa w prawdziwym terminalu (uwaga: w nowym Rust można użyć std::io::IsTerminal)

# --- Graficzny Interfejs Użytkownika (GUI) ---
slint = "1.15.1"               # Główna biblioteka frameworka GUI Slint
i-slint-backend-winit = "1.15.1" # Wewnętrzny backend Slinta oparty na Winit (obsługa okien i wejścia)
winit = "0.30.13"              # Niskopoziomowa biblioteka do tworzenia okien (używana pod spodem przez Slinta)
arboard = "3.6.1"              # Biblioteka do obsługi schowka (np. kopiowanie wyników działania programu)

# --- Sieć i Asynchroniczność (Pobieranie danych z Geneteki) ---
tokio = { version = "1.50.0", features = ["rt-multi-thread", "macros"] } # Środowisko asynchroniczne niezbędne do działania reqwest
reqwest = "0.13.2"      # Klient HTTP do pobierania plików KML (Google Maps) i stron HTML (Geneteka)
url = "2.5.8"           # Biblioteka do łatwej analizy i wyciągania parametrów z linków (np. rid i w)

# --- Przetwarzanie i Serializacja Danych (XML, HTML, JSON) ---
serde = { version = "1.0.228", features = ["derive"] } # Potężny framework do definiowania struktur danych (Serialize/Deserialize)
serde_json = "1.0.149"  # Narzędzie do konwersji naszych struktur w Rusta na pliki JSON i odwrotnie
roxmltree = "0.21.1"    # Ultraszybki parser drzewa XML – używany do czytania tagów z map KML
scraper = "0.25.0"      # Parser HTML z obsługą selektorów CSS (wyciąganie danych prosto z tabel na stronie WWW)

# ---- INNE NARZĘDZIA ----
toml = "1.0.6"
tiny-skia = "0.12.0"
geojson = "0.24.2"

# ZALEŻNOŚCI BUDOWANIA (Używane TYLKO przez plik build.rs przed główną kompilacją)
[build-dependencies]
slint-build = "1.15.1"  # Kompilator Slinta: tłumaczy pliki .slint na kod Rusta podczas etapu budowania

# KONFIGURACJA BIBLIOTEKI
[lib]
name = "fifak_lib"
path = "src/lib/mod.rs" # Wskazuje, gdzie znajduje się główne "jądro" Twojej aplikacji (biblioteka wewnętrzna)

# KONFIGURACJA PLIKÓW WYKONYWALNYCH (Pojedyncze programy bazujące na fifak_lib)

[[bin]]
name = "PobieraczGenetekaZakresMapaGET"  
path = "src/bin/batch/pobieracz_geneteka_zakres_mapa__get.rs"

[[bin]]
name = "PobieraczGenetekaZakresHtmlGET"  
path = "src/bin/batch/pobieracz_geneteka_zakres_html__get.rs"
[[bin]]
name = "AppKartezjanProduktCLI"                    
path = "src/bin/cli/app_kartezjan_produkt__cli.rs" 

[[bin]]
name = "AppKartezjanProduktTUI"                    
path = "src/bin/cli/app_kartezjan_produkt__tui.rs" 

[[bin]]
name = "AppKartezjanProduktGUI"                
path = "src/bin/gui/app_kartezjan_produkt__gui.rs"

[[bin]]
name = "AppGenetekaZakresGUI"  
path = "src/bin/gui/app_geneteka_zakres__gui.rs"
```

## RUST-A1a: `./src/batch/pobieracz_geneteka_zakres_mapa__get.rs`

```rust
use fifak_lib::geneteka::pobieracz_kml::pobierz_zakres;
use fifak_lib::geneteka::parser_kml::parsuj_do_json;
use fifak_lib::logic::utils::{obsluz_wynik_parsowania, obsluz_wynik_pobierania};

#[tokio::main]
async fn main() {
    println!("[*] Uruchamiam narzędzia Geneteki (Moduł Mapy)...");

    // 1. Pobieranie KML (lub potwierdzenie istnienia z dzisiaj)
    let sciezka_kml = obsluz_wynik_pobierania(pobierz_zakres().await);

    println!("[+] Plik źródłowy KML gotowy do analizy: {}", sciezka_kml);

    // 2. Generujemy ścieżkę do pliku JSON (podmieniamy końcówkę .kml na .json)
    let sciezka_json = sciezka_kml.replace(".kml", ".json");

    // 3. Odpalamy nasz "Pożeracz XML-a"
    obsluz_wynik_parsowania(
        parsuj_do_json(&sciezka_kml, &sciezka_json),
        "[*] Sukces! Cała operacja zakończona!",
        "[-] Błąd podczas parsowania KML: {:?}"
    );
}

```

## RUST-A1b: `./src/batch/pobieracz_geneteka_zakres_html__get.rs`

```rust
use fifak_lib::geneteka::pobieracz_html::pobierz_rejestry;
use fifak_lib::geneteka::parser_html::parsuj_html;
use fifak_lib::logic::utils::{obsluz_wynik_parsowania, obsluz_wynik_pobierania};

#[tokio::main]
async fn main() {
    println!("[*] Uruchamiam narzędzia Geneteki (Moduł Rejestrów HTML)...");

    // 1. Pobieranie lub wczytywanie pliku HTML
    let sciezka_html = obsluz_wynik_pobierania(pobierz_rejestry().await);

    println!("[+] Plik źródłowy HTML gotowy do analizy: {}", sciezka_html);

    // 2. Przygotowanie ścieżki docelowej
    // Wynik zapiszemy jako rejestry_YYYY-MM-DD.json
    let sciezka_json = sciezka_html.replace(".html", ".json");

    // 3. Odpalamy nasz "Kombajn do HTML-a"
    obsluz_wynik_parsowania(
        parsuj_html(&sciezka_html, &sciezka_json),
        "[*] Sukces! Cała operacja HTML zakończona! Możesz sprawdzić pliki wynikowe.",
        "[-] Błąd podczas parsowania HTML: {:?}"
    );
}
    
```

## RUST-A2a: `./src/cli/app_kartezjan_produkt__cli.rs`

```rust
use clap::Parser;

use fifak_lib::logic::format::OutputFormat;
use fifak_lib::logic::format::format_result;
use fifak_lib::logic::morphology::generate_morphology;

#[derive(Parser)]
#[command(name = "Generator Morfologiczny - CLI")]
#[command(author = "Jaśko")]
#[command(version = "0.1.1")]
#[command(about = "Szybki generator wyrazów z wiersza poleceń")]
struct Cli {
    /// Wzór morfologiczny, np "(Zg|Sg|Zk|Sk)o(d|ds|dz)"
    #[arg(required = true)]
    pattern: String,

    /// Format wyjściowy
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Lista)]
    format: OutputFormat,
}

fn main() {
    // Od razu parsujemy i generujemy, bez zbędnych matchy
    let cli = Cli::parse();
    
    let names = generate_morphology(&cli.pattern);
    format_result(&names, &cli.format);
}
```

## RUST-A3a: `./src/tui/app_kartezjan_produkt__tui.rs`

```rust
use fifak_lib::cli::menu::run_menu;

fn main() {
    // Żadnego sprawdzania argumentów, od razu wchodzimy w tryb interaktywny
    run_menu();
}
```

## RUST-A4a: `./src/gui/app_kartezjan_produkt__gui.rs`

```rust
#![windows_subsystem = "windows"]

use anyhow::Result;
use arboard::Clipboard;
use fifak_lib::logic::morphology::generate_morphology;
use fifak_lib::window;
use fifak_lib::setup_window_ctrl_bindings;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() -> Result<()> {
    let ui= GenpkWindow::new()?; 
    setup_window_ctrl_bindings!(ui);

    // 1. Generowanie
    let ui_weak_send = ui.as_weak();
    ui.on_send(move |pattern| {
        if let Some(ui) = ui_weak_send.upgrade() {
            let results = generate_morphology(pattern.as_str());
            let model: Vec<SharedString> = results.iter().map(SharedString::from).collect();
            ui.set_results(ModelRc::new(VecModel::from(model)));
            
            let joined = results.join(", ");
            ui.set_joined_text(SharedString::from(joined));
        }
    });

    // 2. Schowek
    let ui_weak_copy = ui.as_weak();
    ui.on_copy_to_clipboard(move |mode| {
        if let Some(ui) = ui_weak_copy.upgrade() {
            let text_to_copy = if mode == "list" {
                let model = ui.get_results();
                let mut items: Vec<String> = Vec::new();
                for i in 0..model.row_count() {
                    if let Some(val) = model.row_data(i) {
                        items.push(val.to_string());
                    }
                }
                items.join("\n")
            } else {
                ui.get_joined_text().to_string()
            };

            if let Ok(mut cb) = Clipboard::new() {
                let _ = cb.set_text(text_to_copy);
            }
        }
    });

    ui.run()?;
    Ok(())
}
```

## RUST-A4b: `./src/gui/app_geneteka_zakres__gui.rs`

```rust
#![windows_subsystem = "windows"]

use anyhow::Result;

use std::rc::Rc;
use slint::ComponentHandle; // UWAGA: Usunąłem stąd VecModel, bo już go nie używamy!

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
    // NOWOŚĆ: SILNIK RENDERUJĄCY W RUŚCIE (TINY-SKIA)
    // To zastępuje całkowicie pętlę "for pt in map_data.points..."
    // ==========================================
    let ui_handle = ui.as_weak();
    let map_data_render = Rc::clone(&map_data); // Klonujemy referencję dla closure
    
    ui.on_camera_changed(move |w, h, offset_x, offset_y, zoom, rot| {
        if let Some(ui_ref) = ui_handle.upgrade() {
            // Rust błyskawicznie rysuje płótno na podstawie widoku z kamery:
            let image_frame = fifak_lib::atlas::renderer::render_frame(
                w as u32, 
                h as u32, 
                &map_data_render, 
                offset_x, 
                offset_y, 
                zoom,
                rot
            );
            // I wysyła jedną gotową, lekką klatkę do Slinta:
            ui_ref.set_map_frame(image_frame);
        }
    });
    // ==========================================

    ui.on_search(|text| {
        println!("[*] Szukamy: {}", text);
    });

    ui.run()?;
    Ok(())
}
```
