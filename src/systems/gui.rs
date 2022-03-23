use clockwork::{
    graphics::state::GuiState, kernel::standard_runtime::StandardRuntimeStatistics,
    prelude::MainLoopStatistics,
};
use legion::system;

#[system]
pub fn gui(#[resource] gui: &mut GuiState, #[resource] stat: &MainLoopStatistics) {
    // Setting up GUI
    let stat = stat.clone();
    gui.immediate_ui(move |ctx| {
        egui::Window::new("Main Loop Statistics object").show(&ctx, |ui| {
            ui.add(egui::widgets::Label::new(format!(
                "TPS: {}",
                stat.current_tps() as u32
            )));
            ui.add(egui::widgets::Label::new(format!(
                "FPS: {}",
                stat.current_fps() as u32
            )));
        });
    });
}
