use simplex::app::SimplexVisualizer;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    tracing_subscriber::fmt::init();

    eframe::run_native(
        "simplex",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(SimplexVisualizer::default())),
    );
    // Initialisation of the simplex
    // historique = init_historique()
    // s = init (lecture dans le terminal)
    // simplex(s)
    // modifier s et afficher les résultats

    // if TOUCH : NEXT_STEP
    //     function : next_step

    // if TOUCH : LAST_STEP
    //     function : last_step
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "simplex",
            eframe::WebOptions::default(),
            Box::new(|_cc| Box::new(SimplexVisualizer::default())),
        )
        .await
        .expect("could not start simplex visualizer");
    })
}
