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

/// Aligns channel data to the master channel using nearest-neighbor interpolation.
fn align_nearest(
    master_times: &[f64],
    channel_times: &[f64],
    channel_values: &[f64],
) -> Vec<Option<f64>> {
    master_times
        .iter()
        .map(|&master_time| {
            channel_times
                .iter()
                .zip(channel_values.iter())
                .min_by(|(time1, _), (time2, _)| {
                    (*time1 - master_time)
                        .abs()
                        .partial_cmp(&(*time2 - master_time).abs())
                        .unwrap()
                })
                .map(|(_, &value)| value) // Use the nearest value
        })
        .collect()
}

/// Exports the laps and channel data to a CSV file.
fn export_to_csv(laps: &[LapData], file_path: &str) -> bool {
    if let Ok(mut writer) = csv::Writer::from_path(file_path) {
        eprintln!("Building csv header");

        // Create the CSV header
        let mut headers = vec!["lap".to_string(), "time".to_string()];
        if let Some(first_lap) = laps.first() {
            for channel in &first_lap.channels {
                headers.push(channel.name.clone());
            }
        }
        let header_refs: Vec<&str> = headers.iter().map(String::as_str).collect();
        if writer.write_record(&header_refs).is_err() {
            return false;
        }

        let mut row_counter = 0;

        // Write data rows aligned to the master channel
        for lap in laps {
            eprintln!("Processing lap {}", lap.lap + 1);

            // Find the master channel
            let master_channel = lap
                .channels
                .iter()
                .find(|channel| channel.name == "ECEF position_X")
                .expect("Master channel not found");

            let master_times: Vec<f64> = master_channel
                .data
                .iter()
                .map(|data_point| data_point.s)
                .collect();

            // Align all channels to the master channel
            let mut aligned_data: Vec<Option<Vec<f64>>> = vec![None; lap.channels.len()];

            for (i, channel) in lap.channels.iter().enumerate() {
                if channel.name == "ECEF position_X" {
                    // Add the master channel directly
                    aligned_data[i] = Some(master_channel.data.iter().map(|dp| dp.v).collect());
                    continue;
                }

                println!("Aligning datapoints for channel {}", channel.name);

                let channel_times: Vec<f64> = channel.data.iter().map(|dp| dp.s).collect();
                let channel_values: Vec<f64> = channel.data.iter().map(|dp| dp.v).collect();
                let aligned_values = align_nearest(&master_times, &channel_times, &channel_values);
                aligned_data[i] = Some(aligned_values.into_iter().flatten().collect());
            }

            println!("Writing datapoints to file");
            for (i, &master_time) in master_times.iter().enumerate() {
                let mut row = vec![(lap.lap + 1).to_string(), format!("{:.3}", master_time)];

                for aligned_channel in &aligned_data {
                    if let Some(values) = aligned_channel {
                        if let Some(value) = values.get(i) {
                            row.push(value.to_string());
                        } else {
                            row.push(String::new()); // Missing values
                        }
                    } else {
                        row.push(String::new()); // Missing channel
                    }
                }

                if writer.write_record(&row).is_err() {
                    return false;
                } else {
                    row_counter += 1;
                }
            }
        }

        eprintln!("Created {} rows", row_counter);

        if writer.flush().is_err() {
            return false;
        }

        true
    } else {
        false
    }
}

/// Exports the data for a run to a CSV file.
pub fn export(run: &Run, desired_channels: Option<HashSet<&str>>) {
    let mut laps: Vec<LapData> = Vec::new();
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

    for lap_index in 0..run.number_of_laps() {
        eprintln!("Preparing channel data for lap {}", lap_index + 1);

        let mut channel_data_list: Vec<ChannelData> = Vec::new();

        // Process regular channels
        for channel_id in 0..run.channels_count() {
            let channel_name = run.channel_name(channel_id).unwrap_or_default();
            if !desired_channels.contains(channel_name.as_str()) {
                continue;
            }

            let channel_data = run
                .lap_channel_samples(lap_index, channel_id)
                .unwrap_or_else(|_| xdrk::ChannelData::default());

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
                unit: run.channel_unit(channel_id).unwrap_or_default(),
                data: data_points,
            });
        }

        // Process GPS RAW channels
        for gps_channel_id in 0..run.gps_raw_channels_count() {
            let channel_name = run.gps_raw_channel_name(gps_channel_id).unwrap_or_default();
            if !desired_channels.contains(channel_name.as_str()) {
                continue;
            }

            let channel_data = run
                .lap_gps_raw_channel_samples(lap_index, gps_channel_id)
                .unwrap_or_else(|_| xdrk::ChannelData::default());

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
                unit: run.gps_raw_channel_unit(gps_channel_id).unwrap_or_default(),
                data: data_points,
            });
        }

        // Store the lap data
        laps.push(LapData {
            lap: lap_index,
            channels: channel_data_list,
        });
    }

    if export_to_csv(&laps, "export.csv") {
        eprintln!("Export created successfully");
    } else {
        eprintln!("Failed to create export");
    }
}
