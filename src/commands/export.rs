use csv::Writer;
use serde::Serialize;
use std::collections::HashSet;
use xdrk::Run;

#[derive(Serialize)]
pub struct ExportData {
    pub laps: Vec<LapData>,
}

#[derive(Serialize)]
pub struct LapData {
    pub lap: usize,
    pub channels: Vec<ChannelData>,
}

#[derive(Serialize)]
struct ChannelData {
    name: String,
    unit: String,
    data: Vec<DataPoint>,
}

#[derive(Serialize, Clone)]
struct DataPoint {
    s: f64,
    v: f64,
}

/// Helper function to collect channel data for a lap.
fn collect_channel_data(
    lap_index: usize,
    channel_count: usize,
    name_fn: impl Fn(usize) -> Option<String>,
    unit_fn: impl Fn(usize) -> Option<String>,
    samples_fn: impl Fn(usize, usize) -> Option<xdrk::ChannelData>,
    desired_channels: &HashSet<&str>,
) -> Vec<ChannelData> {
    let mut channel_data_list = Vec::new();

    for channel_id in 0..channel_count {
        if let Some(channel_name) = name_fn(channel_id) {
            if !desired_channels.contains(channel_name.as_str()) {
                continue;
            }

            let channel_data =
                samples_fn(lap_index, channel_id).unwrap_or_else(|| xdrk::ChannelData::default());
            let data_points = channel_data
                .timestamps()
                .iter()
                .zip(channel_data.samples().iter())
                .map(|(&seconds, &value)| DataPoint {
                    s: seconds as f64,
                    v: value as f64,
                })
                .collect();

            channel_data_list.push(ChannelData {
                name: channel_name,
                unit: unit_fn(channel_id).unwrap_or_default(),
                data: data_points,
            });
        }
    }

    channel_data_list
}

/// Exports the laps and channel data to a CSV file.
fn export_to_csv(laps: &[LapData], file_path: &str) -> Result<(), std::io::Error> {
    let mut writer = Writer::from_path(file_path)?;

    eprintln!("[CSV] Creating header");

    // Create the CSV header
    let mut headers = vec![String::from("lap"), String::from("time")];
    if let Some(first_lap) = laps.first() {
        for channel in &first_lap.channels {
            headers.push(channel.name.clone());
            headers.push(format!("{}_UNIT", channel.name));
        }
    }

    writer.write_record(&headers)?;

    let mut row_counter = 0;

    // Write the data rows
    for lap in laps {
        eprintln!("[CSV] Processing lap {}", lap.lap + 1);

        // Gather all timestamps from all channels
        let mut all_timestamps: Vec<f64> = lap
            .channels
            .iter()
            .flat_map(|channel| channel.data.iter().map(|dp| dp.s))
            .collect();

        // Deduplicate and sort the timestamps
        all_timestamps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        all_timestamps.dedup();

        eprintln!("[CSV] Found {} unique timestamps", all_timestamps.len());

        for &timestamp in &all_timestamps {
            let mut row = vec![(lap.lap + 1).to_string(), format!("{:.3}", timestamp)];
            let mut has_data = false;

            for channel in &lap.channels {
                if let Some(data_point) = channel.data.iter().find(|dp| dp.s == timestamp) {
                    row.push(data_point.v.to_string());
                    row.push(channel.unit.clone());
                    has_data = true;
                } else {
                    row.push(String::new());
                    row.push(String::new());
                }
            }

            if has_data {
                writer.write_record(row)?;
                row_counter += 1;
            }
        }
    }

    eprintln!("[CSV] Done, created {} rows", row_counter);
    writer.flush()?;
    Ok(())
}

/// Main export function
pub fn export(run: &Run, desired_channels: Option<HashSet<&str>>) {
    let desired_channels = desired_channels.unwrap_or_else(|| {
        ["ECEF position_X", "ECEF position_Y", "ECEF position_Z"]
            .iter()
            .cloned()
            .collect()
    });

    eprintln!(
        "Preparing to export {} channel(s) per lap",
        desired_channels.len()
    );

    let mut laps = Vec::new();

    for lap_index in 0..run.number_of_laps() {
        eprintln!("Processing channel data for lap {}", lap_index + 1);

        let mut channel_data = Vec::new();

        // Collect regular channel data
        channel_data.extend(collect_channel_data(
            lap_index,
            run.channels_count(),
            |id| run.channel_name(id).ok(),
            |id| run.channel_unit(id).ok(),
            |lap, id| run.lap_channel_samples(lap, id).ok(),
            &desired_channels,
        ));

        // Collect GPS RAW channel data
        channel_data.extend(collect_channel_data(
            lap_index,
            run.gps_raw_channels_count(),
            |id| run.gps_raw_channel_name(id).ok(),
            |id| run.gps_raw_channel_unit(id).ok(),
            |lap, id| run.lap_gps_raw_channel_samples(lap, id).ok(),
            &desired_channels,
        ));

        laps.push(LapData {
            lap: lap_index,
            channels: channel_data,
        });
    }

    eprintln!("Building CSV payload");

    match export_to_csv(&laps, "export.csv") {
        Ok(_) => eprintln!("Export created successfully"),
        Err(e) => eprintln!("Export failed: {}", e),
    }
}
