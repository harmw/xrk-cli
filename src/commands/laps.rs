use xdrk::Run;

pub fn display_laps_info(run: &Run) {
    println!(
        "{:<10} {:<20} {:<20}",
        "LAP".to_string(),
        "START".to_string(),
        "DURATION".to_string()
    );

    for lap in 0..run.number_of_laps() {
        let info = run.lap_info(lap);
        match info {
            Ok(lap_info) => {
                println!(
                    "{:<10} {:<20} {:<20}",
                    lap_info.number(),
                    lap_info.start(),
                    lap_info.time()
                );
            }
            Err(e) => {
                continue;
            }
        }
    }
}
