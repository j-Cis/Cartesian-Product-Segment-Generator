use anyhow::Result;
slint::include_modules!();

fn main() -> Result<()> {
    let app = AppWindow::new()?;
    app.on_close_requested(move || { let _ = slint::quit_event_loop(); });
    app.run()?;
    Ok(())
}

// use cpsgen_lib::logic::morphology::generate_morphology;
// use slint::*;
// 
// slint::slint! {
//     export component MainWindow inherits Window {
//         width: 400px;
//         height: 300px;
//         title: "Generator Morfologiczny - GUI";
// 
//         VerticalLayout {
//             spacing: 10px;
//             padding: 10px;
// 
//             TextInput {
//                 id: pattern_input;
//                 placeholder-text: "Wzór morfologiczny, np (Zg|Sg|Zk|Sk)o(d|ds|dz)";
//             }
// 
//             PushButton {
//                 text: "Generuj";
//                 clicked => {
//                     let pattern = pattern_input.text();
//                     let result = generate_morphology(&pattern);
//                     result_text.set_text(&result.join("\n"));
//                 }
//             }
// 
//             Text {
//                 id: result_text;
//                 text: "";
//                 wrap: word;
//             }
//         }
//     }
// }
// 
// fn main() {
//     let app = MainWindow::new().unwrap();
//     app.run().unwrap();
// }