use lodepng::encode24;
use plotters::backend::{PixelFormat, RGBPixel};
use plotters::prelude::*;
use plotters::style::full_palette::{GREY_900, GREY_A100};

use crate::log::Log;
use crate::{Error, RESOLUTION};

const HEIGHT: u32 = 1000;
const WIDTH: u32 = 600;

/// Generates a graph counting the amount of logs between two points in time.
pub fn create_log_graph(
	mut logs: Vec<Log>,
	channel_name: &str,
	oldest_log_timestamp: i64,
	newest_log_timestamp: i64,
) -> Result<Vec<u8>, Error> {
	if logs.is_empty() {
		return Err("Logs vector empty, cannot generate graph".into());
	}

	logs.sort_by(|lhs, rhs| lhs.time.cmp(&rhs.time));

	// Calculates the highest count of messages.
	let max = logs.iter().fold(0, |max, current| {
		if current.count > max {
			current.count
		} else {
			max
		}
	});

	// Generate graph data.
	let buffer = generate_graph(
		&logs,
		channel_name,
		max.into(),
		oldest_log_timestamp,
		newest_log_timestamp,
	)?;

	// Encode data to png.
	let image = encode24(&buffer, HEIGHT as usize, WIDTH as usize)?;

	Ok(image)
}

/// Core logic for generating the graph for `create_log_graph`
fn generate_graph(
	logs: &[Log],
	channel_name: &str,
	max: i64,
	oldest: i64,
	newest: i64,
) -> Result<Vec<u8>, Error> {
	let mut buffer = vec![0; WIDTH as usize * HEIGHT as usize * RGBPixel::PIXEL_SIZE];

	let root = BitMapBackend::with_buffer(&mut buffer, (HEIGHT, WIDTH)).into_drawing_area();
	root.fill(&BLACK.mix(0.2))?;

	// Draw background and title,
	let mut chart = ChartBuilder::on(&root)
		.caption(
			format!("Logs for {channel_name}"),
			("sans-serif", 45).into_font().with_color(GREY_A100),
		)
		.margin(5)
		.x_label_area_size(65)
		.y_label_area_size(65)
		.margin_right(30)
		.build_cartesian_2d(oldest - RESOLUTION..newest + RESOLUTION, 0..max)?;

	// Draw axis labels.
	chart
		.configure_mesh()
		.bold_line_style(GREY_900)
		.x_desc("Time")
		.y_desc("Messages")
		.x_label_style(("sans-serif", 16).into_font().color(&GREY_A100))
		.y_label_style(("sans-serif", 20).into_font().color(&GREY_A100))
		.axis_desc_style(("sans-serif", 25).with_color(GREY_A100))
		.draw()?;

	// Draw data.
	chart.draw_series(
		Histogram::vertical(&chart)
			.style(GREEN.mix(0.9).filled())
			.data(logs.iter().map(|log| (log.time, log.count.into()))),
	)?;

	// Save data to buffer/
	root.present()?;

	// Drop chart and root to allow return of buffer
	drop(chart);
	drop(root);

	Ok(buffer)
}
