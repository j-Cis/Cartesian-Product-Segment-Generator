pub mod generator;
pub mod modele;
pub mod projekcje;

// Dzięki temu w main.rs po prostu wpiszesz `use fifak_lib::atlas::generate_map_data;`
pub use generator::generate_map_data;
pub use modele::{MapProcessedData, MapProjection, NormalizedPoint};