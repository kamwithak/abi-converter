use serde_json::{Value, to_string_pretty};
use std::fs;
use std::error::Error;

#[derive(Debug)]
struct ABIFile {
    json: &'static str,
    ts: &'static str,
    export: &'static str,
}

fn main() {
    // ABI files to convert
    let abi_files = vec![
        ABIFile { json: "clob.json", ts: "clob.ts", export: "CLOB_ABI" },
    ];

    for file_config in abi_files {
        match convert_abi_file(&file_config) {
            Ok(_) => println!("✅ Converted {} to {}", file_config.json, file_config.ts),
            Err(e) => println!("❌ Error converting {}: {}", file_config.json, e),
        }
    }

    println!("🎉 ABI conversion complete!");
}

fn convert_abi_file(config: &ABIFile) -> Result<(), Box<dyn Error>> {
    // Read the JSON file
    let json_content = fs::read_to_string(config.json)
        .map_err(|e| format!("Failed to read file {}: {}", config.json, e))?;

    // Parse JSON to verify it's valid
    let abi_data: Value = serde_json::from_str(&json_content)
        .map_err(|e| format!("Invalid JSON in {}: {}", config.json, e))?;

    // Format JSON with indentation
    let formatted_json = to_string_pretty(&abi_data)
        .map_err(|e| format!("Failed to format JSON: {}", e))?;

    // Create TypeScript content
    let ts_content = format!("export const {} = {} as const;\n", config.export, formatted_json);

    // Write the TypeScript file
    fs::write(config.ts, ts_content)
        .map_err(|e| format!("Failed to write file {}: {}", config.ts, e))?;

    Ok(())
}
