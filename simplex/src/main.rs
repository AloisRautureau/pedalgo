use simplex::app::SimplexVisualizer;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();

    native_options.maximized = true;

    eframe::run_native(
        "simplex",
        native_options,
        Box::new(|_cc| Box::<SimplexVisualizer>::default()),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() -> eframe::Result<()>{
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "simplex",
            eframe::WebOptions::default(),
            Box::new(|_cc| Box::<SimplexVisualizer>::default()),
        )
        .await
        .expect("could not start simplex visualizer");
    })
}
