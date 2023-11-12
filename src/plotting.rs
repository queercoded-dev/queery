use chrono::NaiveDateTime;
use lodepng::encode24;
use plotters::backend::{PixelFormat, RGBPixel};
use plotters::prelude::*;
use plotters::style::full_palette::{GREY_900, GREY_A100};
use tracing::info;

use crate::log::Log;
use crate::time_period::TimePeriod;
use crate::{Error, RESOLUTION};

const WIDTH: u32 = 1400;
const HEIGHT: u32 = 700;

#[allow(clippy::cognitive_complexity)]
/// Generates a graph counting the amount of logs between two points in time.
///
/// # Parameters
/// `channel_name` The name of the channel, used for the title of the graph.
///
/// `start_timestamp` `end_timestamp` The starting and ending UNIX timestamps to visualise.
pub fn create_log_graph(
	mut logs: Vec<Log>,
	channel_name: &str,
	start_timestamp: i64,
	end_timestamp: i64,
	time_period: TimePeriod,
) -> Result<Vec<u8>, Error> {
	info!("Creating log graph");

	logs.sort_by(|lhs, rhs| lhs.time.cmp(&rhs.time));

	if logs.is_empty() {
		info!("Logs vector empty, cannot generate graph");
		return Err("Logs vector empty, cannot generate graph".into());
	}

	// Calculates the highest count of messages.
	let max = logs.iter().map(|log| log.count).max().unwrap_or(0);

	// Text for X label
	let time_interval_text = format!(
		"Time (Per {})",
		match time_period {
			TimePeriod::Hour => "30 seconds",
			TimePeriod::HalfDay => "6 minutes",
			TimePeriod::Day => "12 minutes",
			TimePeriod::Week => "1.4 hours",
		}
	);

	// Generate graph data.
	let buffer = generate_graph(
		&logs,
		channel_name,
		max.into(),
		start_timestamp,
		end_timestamp,
		&time_interval_text,
	)?;

	info!("Graph image generated");

	// Encode data to png.
	let image = encode24(&buffer, WIDTH as usize, HEIGHT as usize)?;

	info!("Graph image encoded");

	Ok(image)
}

/// Core logic for generating the graph for `create_log_graph`.
///
/// # Parameters
/// `channel_name` The name of the channel, used for the title of the graph.
///
/// `max` The highest amount of messages to show on the graph.
///
/// `start_timestamp` `end_timestamp` The starting and ending UNIX timestamps to visualise.
fn generate_graph(
	logs: &[Log],
	channel_name: &str,
	max: i64,
	start_timestamp: i64,
	end_timestamp: i64,
	time_interval_text: &str,
) -> Result<Vec<u8>, Error> {
	let mut buffer = vec![0; WIDTH as usize * HEIGHT as usize * RGBPixel::PIXEL_SIZE];

	let root = BitMapBackend::with_buffer(&mut buffer, (WIDTH, HEIGHT)).into_drawing_area();
	root.fill(&BLACK.mix(0.2))?;

	// Draw background and title.
	let mut chart = ChartBuilder::on(&root)
		.caption(
			format!("Logs for {channel_name}"),
			("sans-serif", 45).into_font().with_color(GREY_A100),
		)
		.margin(5)
		.x_label_area_size(65)
		.y_label_area_size(65)
		.margin_right(30)
		.build_cartesian_2d(
			start_timestamp - RESOLUTION..end_timestamp + RESOLUTION,
			0..max,
		)?;

	// Draw axis labels.
	chart
		.configure_mesh()
		.bold_line_style(GREY_900)
		.x_desc(time_interval_text)
		.y_desc("Messages")
		.x_label_formatter(&|x| {
			let dt = NaiveDateTime::from_timestamp_opt(*x, 0).unwrap_or_default();
			dt.format("%H:%M").to_string()
		})
		.x_label_style(("sans-serif", 20).into_font().color(&GREY_A100))
		.y_label_style(("sans-serif", 25).into_font().color(&GREY_A100))
		.axis_desc_style(("sans-serif", 30).with_color(GREY_A100))
		.draw()?;

	// Draw data.
	chart.draw_series(
		Histogram::vertical(&chart)
			.style(GREEN.mix(0.9).filled())
			.data(logs.iter().map(|log| (log.time, log.count.into()))),
	)?;

	// Save data to buffer.
	root.present()?;

	// Drop chart and root to allow return of buffer.
	drop(chart);
	drop(root);

	Ok(buffer)
}
