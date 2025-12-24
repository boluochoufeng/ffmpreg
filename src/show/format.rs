pub fn format_size(bytes: u64) -> String {
	const KB: u64 = 1024;
	const MB: u64 = KB * 1024;
	const GB: u64 = MB * 1024;

	if bytes >= GB {
		return format!("{:.2} GB", bytes as f64 / GB as f64);
	}

	if bytes >= MB {
		return format!("{:.2} MB", bytes as f64 / MB as f64);
	}

	if bytes >= KB {
		return format!("{:.2} KB", bytes as f64 / KB as f64);
	}

	format!("{} B", bytes)
}

pub fn format_duration(seconds: f64) -> String {
	if seconds >= 3600.0 {
		let hours = (seconds / 3600.0) as u32;
		let minutes = ((seconds % 3600.0) / 60.0) as u32;
		let secs = seconds % 60.0;
		return format!("{}:{:02}:{:05.2}", hours, minutes, secs);
	}

	if seconds >= 60.0 {
		let minutes = (seconds / 60.0) as u32;
		let secs = seconds % 60.0;
		return format!("{}:{:05.2}", minutes, secs);
	}

	format!("{:.2} s", seconds)
}

pub fn bytes_to_hex(data: &[u8], limit: usize) -> String {
	let take = data.len().min(limit);
	let bytes = &data[..take];
	let hex_parts: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
	let hex_string = hex_parts.join(" ");

	if data.len() > limit {
		return format!("{} ...", hex_string);
	}

	hex_string
}

pub fn hex_without_spaces(hex: &str) -> String {
	hex.replace(' ', "").replace(" ...", "...")
}

pub fn hex_string_to_bytes(hex_str: &str) -> Vec<u8> {
	hex_str.split_whitespace().filter_map(|s| u8::from_str_radix(s, 16).ok()).collect()
}

pub fn format_xxd_style(hex_str: &str, max_bytes: usize) -> String {
	let bytes = hex_string_to_bytes(hex_str);
	let take = bytes.len().min(max_bytes);
	let data = &bytes[..take];

	let mut result = String::new();

	for (chunk_idx, chunk) in data.chunks(16).enumerate() {
		let offset = chunk_idx * 16;

		// Offset in hex: "00000000: "
		result.push_str(&format!("{:08x}: ", offset));

		// 16 bytes in two groups of 8
		for (i, byte) in chunk.iter().enumerate() {
			result.push_str(&format!("{:02x}", byte));
			if i == 7 && chunk.len() > 8 {
				result.push(' ');
			} else if i < 7 || (i == 7 && chunk.len() <= 8) {
				result.push(' ');
			}
		}

		// Padding if last chunk has less than 16 bytes
		if chunk.len() < 16 {
			let padding = 16 - chunk.len();
			for _ in 0..padding {
				result.push_str("   ");
			}
		}

		result.push(' ');

		// ASCII representation (or "." for non-printable)
		for byte in chunk {
			if *byte >= 32 && *byte <= 126 {
				result.push(*byte as char);
			} else {
				result.push('.');
			}
		}

		result.push('\n');
	}

	result.trim_end().to_string()
}
