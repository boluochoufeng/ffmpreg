use super::format::{format_duration, format_size, format_xxd_style};
use super::types::{
	AudioStreamInfo, FrameInfo, MediaInfo, ShowOptions, StreamInfo, VideoStreamInfo,
};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[36m";

pub fn render(info: &MediaInfo, opts: &ShowOptions) {
	render_file_header(info);
	render_streams(info, opts);
	render_frames(info, opts);
}

fn render_file_header(info: &MediaInfo) {
	let duration = format_duration(info.file.duration);
	let size = format_size(info.file.size);

	println!();
	println!("{}{}{}", CYAN, &info.file.path, RESET);
	println!(
		"{}  duration: {}  size: {}  streams: {}{}",
		DIM,
		duration,
		size,
		info.streams.len(),
		RESET
	);
	println!();
}

fn render_streams(info: &MediaInfo, opts: &ShowOptions) {
	for stream in &info.streams {
		let should_skip = opts.stream_filter.is_some_and(|f| f != stream.index());

		if should_skip {
			continue;
		}

		match stream {
			StreamInfo::Video(v) => render_video_stream(v),
			StreamInfo::Audio(a) => render_audio_stream(a),
		}
	}
}

fn render_video_stream(stream: &VideoStreamInfo) {
	let fps_decimal = calculate_fps(&stream.frame_rate);

	println!("{}Video Stream #{}{}", BOLD, stream.index, RESET);
	println!(
		"  codec: {}  resolution: {}x{}  fps: {:.2}",
		stream.codec, stream.width, stream.height, fps_decimal
	);
	println!("  format: {}  field: {}", stream.pix_fmt, stream.field_order);
	println!();
}

fn calculate_fps(frame_rate: &str) -> f64 {
	let parts: Vec<&str> = frame_rate.split('/').collect();

	if parts.len() != 2 {
		return 0.0;
	}

	let num: f64 = parts[0].parse().unwrap_or(0.0);
	let den: f64 = parts[1].parse().unwrap_or(1.0);

	if den == 0.0 {
		return 0.0;
	}

	num / den
}

fn render_audio_stream(stream: &AudioStreamInfo) {
	println!("{}Audio Stream #{}{}", BOLD, stream.index, RESET);
	println!(
		"  codec: {}  sample_rate: {} Hz  channels: {}  bit_depth: {}",
		stream.codec, stream.sample_rate, stream.channels, stream.bit_depth
	);
	println!();
}

fn render_frames(info: &MediaInfo, opts: &ShowOptions) {
	let has_frames = !info.frames.is_empty();

	if !has_frames {
		return;
	}

	render_frames_xxd(info, opts);
}

fn render_frames_xxd(info: &MediaInfo, opts: &ShowOptions) {
	println!("{}Frames{} (hex dump)", BOLD, RESET);
	println!();

	let frames_to_show = info.frames.iter().take(opts.frame_limit);

	for frame in frames_to_show {
		println!(
			"{}Frame {}  [pts={}  size={}]{}",
			DIM,
			frame.index,
			frame.pts,
			format_frame_size(frame.size),
			RESET
		);
		let xxd_output = format_xxd_style(&frame.hex, opts.hex_limit);
		println!("{}", xxd_output);
		println!();
	}

	render_remaining_count(&info.frames, opts.frame_limit);
}

fn format_frame_size(bytes: usize) -> String {
	const KB: usize = 1024;
	const MB: usize = KB * 1024;

	if bytes >= MB {
		return format!("{:.1} MB", bytes as f64 / MB as f64);
	}

	if bytes >= KB {
		return format!("{:.1} KB", bytes as f64 / KB as f64);
	}

	format!("{} B", bytes)
}

fn render_remaining_count(frames: &[FrameInfo], limit: usize) {
	let total = frames.len();
	let remaining = total.saturating_sub(limit);

	if remaining == 0 {
		return;
	}

	println!("... {} more frames", remaining);
}
