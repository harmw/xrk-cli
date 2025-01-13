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

fn export_to_csv(laps: &[LapData], file_path: &str) -> bool {
    if let Ok(mut writer) = csv::Writer::from_path(file_path) {
        eprintln!("[CSV] Creating header");

        let mut headers = vec![String::from("lap")];

        if let Some(first_lap) = laps.first() {
            for channel in &first_lap.channels {
                headers.push(channel.name.clone());
                headers.push(format!("{}_UNIT", channel.name));
            }
        }

        // Convert headers to &str for CSV writer
        let header_refs: Vec<&str> = headers.iter().map(String::as_str).collect();
        if writer.write_record(&header_refs).is_err() {
            return false;
        }

        for lap in laps {
            // Find the maximum number of data points across all channels in this lap
            let max_data_points = lap
                .channels
                .iter()
                .map(|channel| channel.data.len())
                .max()
                .unwrap_or(0);

            eprintln!(
                "[CSV] Processing {} datapoints in lap with index {}",
                max_data_points, lap.lap
            );

            for i in 0..max_data_points {
                let mut row = vec![lap.lap.to_string()];

                // For each channel, write the data point value and unit
                for channel in &lap.channels {
                    if let Some(data_point) = channel.data.get(i) {
                        row.push(data_point.v.to_string());
                        row.push(channel.unit.clone());
                    } else {
                        row.push(String::new());
                        row.push(String::new());
                    }
                }

                // Write the row to the CSV
                if writer.write_record(row).is_err() {
                    return false;
                }
            }
        }

        eprintln!("[CSV] Done");

        if writer.flush().is_err() {
            return false;
        }

        true
    } else {
        false
    }
}

pub fn export(run: &Run) {
    let mut laps: Vec<LapData> = Vec::new();
    let desired_channels: HashSet<&str> = ["OIL", "WAT", "gLon", "gLat", "pBrakeF", "pBrakeR"]
        .iter()
        .cloned()
        .collect();

    let desired_channels_len = desired_channels.len();
    eprintln!("Preparing to export {desired_channels_len} channels per lap");

    for lap_index in 0..run.number_of_laps() {
        eprintln!("Processing channel data for lap with index {lap_index}");

        let mut channel_data_list: Vec<ChannelData> = Vec::new();

        // TODO: include GPS and GPS RAW channels
        for channel_id in 0..run.channels_count() {
            let channel_name = run.channel_name(channel_id).unwrap_or_default();

            if !desired_channels.contains(channel_name.as_str()) {
                continue;
            }

            let channel_data = run
                .lap_channel_samples(lap_index, channel_id)
                .unwrap_or_else(|_| xdrk::ChannelData::default());

            let timestamps = channel_data.timestamps();
            let samples = channel_data.samples();

            let data_points: Vec<DataPoint> = timestamps
                .iter()
                .zip(samples.iter())
                .map(|(&seconds, &value)| DataPoint {
                    s: seconds as f64,
                    v: value as f64,
                })
                .collect();

            channel_data_list.push(ChannelData {
                name: run.channel_name(channel_id).unwrap(),
                unit: run.channel_unit(channel_id).unwrap_or_default(),
                data: data_points,
            });
        }

        laps.push(LapData {
            lap: lap_index,
            channels: channel_data_list,
        });
    }

    // eprintln!("Building JSON payload");
    //
    // // Wrap all laps into ExportData
    // let export_data = ExportData { laps };
    //
    // // TODO: optionally use to_string_pretty() instead of to_string()
    // let json_string = serde_json::to_string(&export_data)
    //     .expect("Failed to serialize channel data to JSON");
    //
    // // TODO: optionally export directly to file instead of stdout
    // println!("{}", json_string);

    eprintln!("Building CSV payload");

    if export_to_csv(&laps, "export.csv") {
        let total_data_points: usize = laps
            .iter()
            .flat_map(|lap| &lap.channels)
            .map(|channel| channel.data.len())
            .sum();

        eprintln!("Successfully exported {} data points", total_data_points);
    }
}
