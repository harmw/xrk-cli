use xdrk::Run;

pub fn display_run_info(run: &Run) {
    println!(
        "{:<5} {:<20} {:<10} {:<20} {:<60} {:<50}",
        "LAP".to_string(),
        "CHANNEL".to_string(),
        "UNIT".to_string(),
        "COUNT".to_string(),
        "PREVIEW (TIMESTAMPS)".to_string(),
        "PREVIEW (DATA)".to_string()
    );

    // TODO: obviously, this should be a cmdline arg
    let lap = 3;
    let lap_index = lap - 1;

    // Regular channels
    for id in 0..run.channels_count() {
        let channel_data = run
            .lap_channel_samples(lap_index, id)
            .unwrap_or_else(|_| xdrk::ChannelData::default());

        let preview_data = &channel_data.samples()[..channel_data.samples().len().min(3)]
            .iter()
            .map(|&val| format!("{}", val))
            .collect::<Vec<String>>()
            .join(", ");

        let preview_timestamps = &channel_data.timestamps()
            [..channel_data.timestamps().len().min(3)]
            .iter()
            .map(|&val| format!("{}", val))
            .collect::<Vec<String>>()
            .join(", ");

        println!(
            "{:<5} {:<20} {:<10} {:<20} {:<60} {:<50}",
            lap,
            run.channel_name(id).unwrap(),
            run.channel_unit(id).unwrap(),
            run.channel_samples_count(id).unwrap_or_else(|e| { 0 }),
            preview_timestamps,
            preview_data
        );
    }

    // GPS channels
    for id in 0..run.gps_channels_count() {
        let channel_data = run
            .lap_channel_samples(lap_index, id)
            .unwrap_or_else(|_| xdrk::ChannelData::default());

        let preview_data = &channel_data.samples()[..channel_data.samples().len().min(3)]
            .iter()
            .map(|&val| format!("{}", val))
            .collect::<Vec<String>>()
            .join(", ");

        let preview_timestamps = &channel_data.timestamps()
            [..channel_data.timestamps().len().min(3)]
            .iter()
            .map(|&val| format!("{}", val))
            .collect::<Vec<String>>()
            .join(", ");

        println!(
            "{:<5} {:<20} {:<10} {:<20} {:<60} {:<50}",
            lap,
            run.gps_channel_name(id).unwrap(),
            run.gps_channel_unit(id).unwrap(),
            run.gps_channel_samples_count(id).unwrap_or_else(|e| 0),
            preview_timestamps,
            preview_data
        );
    }

    // GPS Raw channels
    for id in 0..run.gps_raw_channels_count() {
        let channel_data = run
            .lap_gps_raw_channel_samples(lap_index, id)
            .unwrap_or_else(|_| xdrk::ChannelData::default()); // Will panic if there's an error

        let preview_data = &channel_data.samples()[..channel_data.samples().len().min(3)]
            .iter()
            .map(|&val| format!("{}", val))
            .collect::<Vec<String>>()
            .join(", ");

        let preview_timestamps = &channel_data.timestamps()
            [..channel_data.timestamps().len().min(3)]
            .iter()
            .map(|&val| format!("{}", val))
            .collect::<Vec<String>>()
            .join(", ");

        println!(
            "{:<5} {:<20} {:<10} {:<20} {:<60} {:<50}",
            lap,
            run.gps_raw_channel_name(id).unwrap(),
            run.gps_raw_channel_unit(id).unwrap(),
            run.gps_raw_channel_samples_count(id).unwrap_or_else(|e| 0),
            preview_timestamps,
            preview_data
        );
    }
}
