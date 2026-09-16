// // build.rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 1. Define where the counter text file will live
    let counter_path = Path::new("build_counter.txt");

    // 2. Read the current count or default to 0
    let mut count = if counter_path.exists() {
        fs::read_to_string(counter_path)
            .unwrap_or_default()
            .trim()
            .parse::<u32>()
            .unwrap_or(0)
    } else {
        0
    };

    // 3. Increment the counter
    count += 1;

    // 4. Save the updated count back to the file
    fs::write(counter_path, count.to_string()).unwrap();

    // 5. Pass the value to your main code as an environment variable
    println!("cargo:rustc-env=BUILD_NUMBER={}", count);

    // 6. CRITICAL: Tell Cargo to ALWAYS rerun this script on every build.
    // By tracking a non-existent environment variable or a constantly changing metric,
    // we bypass Cargo's default incremental caching for the build script.
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD_COUNTER_RANDOM_VAL");
}
