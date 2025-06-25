use serde_json::{Value, to_string_pretty};
use std::fs;
use std::error::Error;
use std::path::{Path, PathBuf};
use clap::{Arg, Command};
use glob::glob;

fn main() {
    let matches = Command::new("abi-converter")
        .version("0.1.0")
        .about("Convert ABI JSON files to TypeScript definitions")
        .arg(
            Arg::new("input")
                .help("Input ABI file(s) or glob pattern")
                .required(true)
                .num_args(1..)
                .value_name("INPUT")
        )
        .arg(
            Arg::new("outDir")
                .long("outDir")
                .help("Output directory for TypeScript files")
                .value_name("DIR")
                .default_value(".")
        )
        .get_matches();

    let input_patterns: Vec<&String> = matches.get_many::<String>("input").unwrap().collect();
    let out_dir = matches.get_one::<String>("outDir").unwrap();

    let mut converted_count = 0;
    let mut error_count = 0;

    for pattern in input_patterns {
        // Handle glob patterns
        let files = if pattern.contains('*') || pattern.contains('?') {
            match glob(pattern) {
                Ok(paths) => paths.filter_map(Result::ok).collect(),
                Err(e) => {
                    println!("❌ Invalid glob pattern '{}': {}", pattern, e);
                    error_count += 1;
                    continue;
                }
            }
        } else {
            vec![PathBuf::from(pattern)]
        };

        for file_path in files {
            match convert_abi_file(&file_path, out_dir) {
                Ok(output_file) => {
                    println!("✅ Converted {} to {}", file_path.display(), output_file);
                    converted_count += 1;
                }
                Err(e) => {
                    println!("❌ Error converting {}: {}", file_path.display(), e);
                    error_count += 1;
                }
            }
        }
    }

    if converted_count > 0 {
        println!("🎉 Conversion complete! {} files converted", converted_count);
    }
    if error_count > 0 {
        println!("⚠️  {} files failed to convert", error_count);
        std::process::exit(1);
    }
}

fn convert_abi_file(input_path: &Path, out_dir: &str) -> Result<String, Box<dyn Error>> {
    // Read the JSON file
    let json_content = fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read file {}: {}", input_path.display(), e))?;

    // Parse JSON to verify it's valid
    let abi_data: Value = serde_json::from_str(&json_content)
        .map_err(|e| format!("Invalid JSON in {}: {}", input_path.display(), e))?;

    // Format JSON with indentation
    let formatted_json = to_string_pretty(&abi_data)
        .map_err(|e| format!("Failed to format JSON: {}", e))?;

    // Generate output filename and export name
    let file_stem = input_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid input filename")?;
    
    // Remove common ABI suffixes for cleaner names
    let clean_name = file_stem
        .replace(".abi", "")
        .replace("-abi", "")
        .replace("_abi", "");
    
    let export_name = clean_name.to_uppercase().replace("-", "_").replace(".", "_") + "_ABI";
    let output_filename = format!("{}.ts", clean_name.replace(".", "-"));
    let output_path = Path::new(out_dir).join(output_filename);

    // Create output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    // Create TypeScript content
    let ts_content = format!("export const {} = {} as const;\n", export_name, formatted_json);

    // Write the TypeScript file
    fs::write(&output_path, ts_content)
        .map_err(|e| format!("Failed to write file {}: {}", output_path.display(), e))?;

    Ok(output_path.display().to_string())
}
