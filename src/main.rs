/* Copyright (c) 2025 Eric Hernandez  */

use std::fs;
use std::io;
use std::path::Path;

use chrono::{Datelike, Utc};
use clap::{Arg, ArgAction, Command};
use regex::Regex;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
	let matches = Command::new("License Updater")
		.version("1.0")
		.author("Your Name")
		.about("Updates copyright headers and license footers in source code files")
		.arg(
			Arg::new("author")
				.short('a')
				.long("author")
				.value_name("NAME")
				.help("Sets the copyright author name")
				.required(true),
		)
		.arg(
			Arg::new("path")
				.help("File or directory to process")
				.required(true)
				.index(1),
		)
		.arg(
			Arg::new("license")
				.short('l')
				.long("license")
				.value_name("FILE")
				.help("Path to license file (default: searches for LICENSE in project root)"),
		)
		.arg(
			Arg::new("dry-run")
				.long("dry-run")
				.help("Show what would be done without making changes")
				.action(ArgAction::SetTrue),
		)
		.get_matches();

	let author_name = matches
		.get_one::<String>("author")
		.expect("author is required");
	let path_str = matches.get_one::<String>("path").expect("path is required");
	let dry_run = matches.get_flag("dry-run");

	// Determine license content.
	let license_content = if let Some(license_path) = matches.get_one::<String>("license") {
		fs::read_to_string(license_path)?
	} else {
		find_and_read_license(path_str)?
	};

	let path = Path::new(path_str);
	if path.is_file() {
		update_file(path, author_name, &license_content, dry_run)?;
	} else if path.is_dir() {
		for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
			let entry_path = entry.path();
			if entry_path.is_file() && is_source_file(entry_path) {
				update_file(entry_path, author_name, &license_content, dry_run)?;
			}
		}
	} else {
		eprintln!(
			"Path does not exist or is not accessible: {}",
			path.display()
		);
	}

	Ok(())
}

/// Search for a LICENSE file in the current or parent directories.
fn find_and_read_license(start_path: &str) -> io::Result<String> {
	let start_path = Path::new(start_path);
	let mut current_dir = if start_path.is_file() {
		start_path.parent().unwrap_or(Path::new(".")).to_path_buf()
	} else {
		start_path.to_path_buf()
	};

	for _ in 0..100 {
		for license_filename in &["LICENSE", "LICENSE.md", "LICENSE.txt"] {
			let license_path = current_dir.join(license_filename);
			if license_path.exists() {
				return fs::read_to_string(license_path);
			}
		}
		if let Some(parent) = current_dir.parent() {
			current_dir = parent.to_path_buf();
		} else {
			break;
		}
	}

	Err(io::Error::new(
		io::ErrorKind::NotFound,
		"License file not found",
	))
}

/// Check whether a file is a source file based on its extension.
fn is_source_file(path: &Path) -> bool {
	let source_extensions = [
		".rs", ".py", ".js", ".jsx", ".ts", ".tsx", ".c", ".cpp", ".h", ".hpp", ".java", ".go",
		".rb", ".php", ".swift", ".kt", ".cs", ".sh", ".bash", ".pl", ".pm", ".lua", ".scala",
		".css", ".scss", ".html", ".xml", ".json",
	];

	if let Some(ext) = path.extension() {
		let ext = format!(".{}", ext.to_string_lossy().to_lowercase());
		source_extensions.contains(&ext.as_str())
	} else {
		false
	}
}

/// Returns a tuple of (block comment start, comment prefix, block comment end) for a file.
fn get_comment_style(path: &Path) -> (&'static str, &'static str, &'static str) {
	if let Some(ext) = path.extension() {
		match ext.to_string_lossy().to_lowercase().as_str() {
			// C-style comments.
			"rs" | "c" | "cpp" | "h" | "hpp" | "js" | "jsx" | "ts" | "tsx" | "go" | "java"
			| "swift" | "kt" | "scala" | "css" | "scss" | "cs" => ("/*", " * ", " */"),
			// Hash-style comments.
			"py" | "rb" | "sh" | "bash" | "pl" | "pm" | "php" => ("#", "# ", "#"),
			// Lua-style comments.
			"lua" => ("--[[", "-- ", "--]]"),
			// HTML/XML-style comments.
			"html" | "xml" => ("<!--", " ", "-->"),
			_ => ("/*", " * ", " */"),
		}
	} else {
		("#", "# ", "#")
	}
}

/// Update a single file with the copyright header at the top and license footer
/// at the bottom.
fn update_file(
	file_path: &Path,
	author_name: &str,
	license_content: &str,
	dry_run: bool,
) -> io::Result<()> {
	// Skip very large files.
	let metadata = fs::metadata(file_path)?;
	if metadata.len() > 1_000_000 {
		println!("Skipping large file: {}", file_path.display());
		return Ok(());
	}

	// Read the file as text.
	let content = match fs::read_to_string(file_path) {
		Ok(c) => c,
		Err(_) => {
			println!("Skipping binary file: {}", file_path.display());
			return Ok(());
		}
	};

	let (comment_start, comment_prefix, comment_end) = get_comment_style(file_path);
	let current_year = Utc::now().year();

	// Create a regex to match an existing copyright header.
	let copyright_pattern = format!(
		r"{}\s*Copyright \(c\) (\d{{4}}(?:-\d{{4}})?)(?: {}\s*.*?){}",
		regex::escape(comment_start),
		regex::escape(author_name),
		regex::escape(comment_end)
	);
	let copyright_regex = Regex::new(&copyright_pattern).unwrap();

	let updated_content = if let Some(caps) = copyright_regex.captures(&content) {
		let year_str = caps.get(1).unwrap().as_str();
		if year_str.contains('-') {
			let parts: Vec<&str> = year_str.split('-').collect();
			let start_year: i32 = parts[0].parse().unwrap();
			let end_year: i32 = parts[1].parse().unwrap();
			if end_year == current_year {
				content.clone()
			} else {
				let new_copyright = format!(
					"{} Copyright (c) {}-{} {} {}",
					comment_start, start_year, current_year, author_name, comment_end
				);
				copyright_regex.replace(&content, new_copyright).to_string()
			}
		} else {
			let year: i32 = year_str.parse().unwrap();
			if year == current_year {
				content.clone()
			} else {
				let new_copyright = format!(
					"{} Copyright (c) {}-{} {} {}",
					comment_start, year, current_year, author_name, comment_end
				);
				copyright_regex.replace(&content, new_copyright).to_string()
			}
		}
	} else {
		format!(
			"{} Copyright (c) {} {} {}\n\n{}",
			comment_start, current_year, author_name, comment_end, content
		)
	};

	// Format the license text using the file's comment style.
	let formatted_license = license_content
		.lines()
		.map(|line| {
			if line.trim().is_empty() {
				comment_prefix.trim_end().to_string()
			} else {
				format!("{}{}", comment_prefix, line)
			}
		})
		.collect::<Vec<String>>()
		.join("\n");

	let license_footer = format!(
		"\n\n{}\n{}License:\n{}\n{}",
		comment_start, comment_prefix, formatted_license, comment_end
	);

	// Use a dot-all regex that matches:
	// - Two newlines
	// - The comment-start line
	// - Some intervening lines (including one that contains "License:")
	// - And ending with the comment-end at the end of the file.
	let license_pattern = format!(
		r"(?s)\n\n{}\n.*?License:.*?\n.*?{}\s*$",
		regex::escape(comment_start),
		regex::escape(comment_end)
	);
	let license_regex = Regex::new(&license_pattern).unwrap();

	let final_content = if license_regex.is_match(&updated_content) {
		// Replace the identified license footer with our new footer.
		license_regex
			.replace(&updated_content, license_footer.as_str())
			.to_string()
	} else {
		// No license footer found; append the new footer.
		format!("{}{}", updated_content.trim_end(), license_footer)
	};

	if dry_run {
		println!("Would update: {}", file_path.display());
		if content != final_content {
			println!("  Changes would be made.");
		} else {
			println!("  No changes needed.");
		}
	} else if content != final_content {
		fs::write(file_path, final_content)?;
		println!("Updated: {}", file_path.display());
	} else {
		println!("No changes needed: {}", file_path.display());
	}

	Ok(())
}

/*
 * License:
 * Copyright (c) 2025 Eric Hernandez
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
