use xdrk::Run;

pub fn display_run_info(run: &Run) {
    println!(
        "{:<30}: {:?}",
        "DATETIME".to_string(),
        run.datetime().unwrap()
    );

    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");

    println!(
        "{:<30}: {}",
        "DRIVER".to_string(),
        run.racer().unwrap_or("Unknown".to_string())
    );
    println!(
        "{:<30}: {}",
        "VEHICLE".to_string(),
        run.vehicle().unwrap_or("Unknown".to_string())
    );
    println!(
        "{:<30}: {}",
        "TRACK".to_string(),
        run.track().unwrap_or("Unknown".to_string())
    );
    println!(
        "{:<30}: {} / {}",
        "CHAMPIONSHIP".to_string(),
        run.championship().unwrap_or("Unknown".to_string()),
        run.venue_type().unwrap_or("Unknown".to_string())
    );
    println!("{:<30}: {}", "LAPS".to_string(), run.number_of_laps());

    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");

    println!(
        "{:<30}: {:?}",
        "DATA CHANNELS".to_string(),
        run.channels_count()
    );
    println!(
        "{:<30}: {:?} (+{:?} raw)",
        "GPS DATA CHANNELS".to_string(),
        run.gps_channels_count(),
        run.gps_raw_channels_count()
    );
}
