use xdrk::Run;

pub fn display_laps_info(run: &Run) {
    println!(
        "{:<10} {:<20} {:<20} {}",
        "LAP".to_string(),
        "START".to_string(),
        "DURATION".to_string(),
        "LAP TIME".to_string()
    );

    for lap in 0..run.number_of_laps() {
        let info = run.lap_info(lap);
        match info {
            Ok(lap_info) => {
                let start_time = format!("{:.3}", lap_info.start());
                let duration = format!("{:.3}", lap_info.time());

                let minutes = (&lap_info.time() / 60.0).floor();
                let secs = &lap_info.time() % 60.0;
                let lap_time = format!("{:02}:{:06.3}", minutes as u64, secs);

                println!(
                    "{:<10} {:<20} {:<20} {}",
                    lap_info.number(),
                    start_time,
                    duration,
                    lap_time
                );
            }
            Err(e) => {
                continue;
            }
        }
    }
}
