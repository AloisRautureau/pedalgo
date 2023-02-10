use simplex::app::SimplexVisualizer;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    eframe::run_native(
        "simplex",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(SimplexVisualizer::init(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() -> eframe::Result<()> {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "simplex",
            eframe::WebOptions::default(),
            Box::new(|cc| Box::new(SimplexVisualizer::init(cc))),
        )
        .await
        .expect("could not start simplex visualizer");
    });
    Ok(())
}
