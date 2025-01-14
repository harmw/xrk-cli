use xdrk::Run;

pub fn display_channels_list(run: &Run) {
    println!(
        "{:<20} {:<10} {:<20} {:<20} {:<30} {:<50}",
        "CHANNEL".to_string(),
        "UNIT".to_string(),
        "COUNT".to_string(),
        "FREQUENCY (Hz)".to_string(),
        "PREVIEW (TIMESTAMPS)".to_string(),
        "PREVIEW (DATA)".to_string()
    );

    for id in 0..run.channels_count() {
        let channel_data = run
            .channel_samples(id)
            .unwrap_or_else(|_| xdrk::ChannelData::default());

        let preview_data = &channel_data.samples()[..channel_data.samples().len().min(3)]
            .iter()
            .map(|&val| format!("{}", val))
            .collect::<Vec<String>>()
            .join(", ");

        let preview_timestamps = &channel_data.timestamps()
            [..channel_data.timestamps().len().min(3)]
            .iter()
            .map(|&val| format!("{:.3}", val))
            .collect::<Vec<String>>()
            .join(", ");

        println!(
            "{:<20} {:<10} {:<20} {:<20} {:<30} {:<50}",
            run.channel_name(id).unwrap(),
            run.channel_unit(id).unwrap(),
            run.channel_samples_count(id).unwrap_or_else(|e| { 0 }),
            calculate_frequency(&channel_data.timestamps()),
            preview_timestamps,
            preview_data
        );
    }

    for id in 0..run.gps_channels_count() {
        let channel_data = run
            .channel_samples(id)
            .unwrap_or_else(|_| xdrk::ChannelData::default());

        let preview_data = &channel_data.samples()[..channel_data.samples().len().min(3)]
            .iter()
            .map(|&val| format!("{}", val))
            .collect::<Vec<String>>()
            .join(", ");

        let preview_timestamps = &channel_data.timestamps()
            [..channel_data.timestamps().len().min(3)]
            .iter()
            .map(|&val| format!("{:.3}", val))
            .collect::<Vec<String>>()
            .join(", ");

        println!(
            "{:<20} {:<10} {:<20} {:<20} {:<30} {:<50}",
            run.gps_channel_name(id).unwrap(),
            run.gps_channel_unit(id).unwrap(),
            run.gps_channel_samples_count(id).unwrap_or_else(|e| 0),
            calculate_frequency(&channel_data.timestamps()),
            preview_timestamps,
            preview_data
        );
    }

    for id in 0..run.gps_raw_channels_count() {
        let channel_data = run
            .gps_raw_channel_samples(id)
            .unwrap_or_else(|_| xdrk::ChannelData::default()); // Will panic if there's an error

        let preview_data = &channel_data.samples()[..channel_data.samples().len().min(3)]
            .iter()
            .map(|&val| format!("{}", val))
            .collect::<Vec<String>>()
            .join(", ");

        let preview_timestamps = &channel_data.timestamps()
            [..channel_data.timestamps().len().min(3)]
            .iter()
            .map(|&val| format!("{:.3}", val))
            .collect::<Vec<String>>()
            .join(", ");

        println!(
            "{:<20} {:<10} {:<20} {:<20} {:<30} {:<50}",
            run.gps_raw_channel_name(id).unwrap(),
            run.gps_raw_channel_unit(id).unwrap(),
            run.gps_raw_channel_samples_count(id).unwrap_or_else(|e| 0),
            calculate_frequency(&channel_data.timestamps()),
            preview_timestamps,
            preview_data
        );
    }
}

fn calculate_frequency(timestamps: &[f64]) -> f64 {
    if timestamps.len() < 2 {
        return 0.0;
    }

    let intervals: Vec<f64> = timestamps
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect();

    let avg_interval = intervals.iter().sum::<f64>() / intervals.len() as f64;
    (1.0 / avg_interval).round()
}
