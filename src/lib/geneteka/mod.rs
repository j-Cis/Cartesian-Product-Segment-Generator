// 1. Wspólne fundamenty
pub mod modele;
pub mod data_raw_modele;
pub mod pobrany_najnowszy;
pub mod parser_rocznikow;

// 2. Moduł: Mapa Google (KML)
pub mod pobieracz_kml;
pub mod parser_kml;

// 3. Moduł: Rejestry Geneteki (HTML)
pub mod pobieracz_html;
pub mod parser_html;