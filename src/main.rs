use image::{ImageBuffer, Rgba, GenericImageView};
use std::fs::create_dir_all;
use std::path::Path;
use walkdir::WalkDir;
use exif::{Reader, In, Tag};
use chrono::NaiveDateTime;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    
    // Create converted directory if it doesn't exist
    create_dir_all("converted")?;

    // First count total files to process
    let total_files = WalkDir::new(".")
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            if let Some(ext) = e.path().extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                ext == "jpg" || ext == "jpeg"
            } else {
                false
            }
        })
        .count();

    // Create progress bar
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
        .unwrap()
        .progress_chars("#>-"));

    let mut processed_count = 0;
    
    // Walk through current directory
    for entry in WalkDir::new(".").min_depth(1).max_depth(1) {
        let entry = entry?;
        let path = entry.path();
        
        // Check if file is a jpeg
        if let Some(extension) = path.extension() {
            if extension.to_string_lossy().to_lowercase() == "jpg" 
               || extension.to_string_lossy().to_lowercase() == "jpeg" {
                process_image(path)?;
                processed_count += 1;
                pb.inc(1);
            }
        }
    }

    pb.finish_with_message("Processing complete");

    let elapsed = start_time.elapsed();
    eprintln!("\nProcessing completed successfully!");
    eprintln!("Total images processed: {}", processed_count);
    eprintln!("Total elapsed time: {:.2?}", elapsed);
    if processed_count > 0 {
        eprintln!("Average time per image: {:.2?}", elapsed / processed_count as u32);
    }
    //eprintln!("Output directory: {}", std::fs::canonicalize("converted")?.display());
    
    Ok(())
}

fn process_image(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Read the image
    let mut img = image::open(path)?;
    
    // Try to read EXIF data and apply orientation
    if let Ok(file) = std::fs::File::open(path) {
        if let Ok(exif) = Reader::new().read_from_container(&mut std::io::BufReader::new(&file)) {
            // Apply EXIF orientation if available
            if let Some(orientation) = exif.get_field(Tag::Orientation, In::PRIMARY) {
                match orientation.value.get_uint(0) {
                    Some(2) => img = img.fliph(),
                    Some(3) => img = img.rotate180(),
                    Some(4) => img = img.flipv(),
                    Some(5) => img = img.rotate90().fliph(),
                    Some(6) => img = img.rotate90(),
                    Some(7) => img = img.rotate270().fliph(),
                    Some(8) => img = img.rotate270(),
                    _ => {} // Default orientation (1) or unknown
                }
            }
        }
    }
    
    // Get original dimensions
    let (width, height) = img.dimensions();
    
    // Calculate scaling factors for UHD
    let scale_w = 3840.0 / width as f32;
    let scale_h = 2160.0 / height as f32;
    let scale = scale_w.min(scale_h);
    
    // Calculate new dimensions maintaining aspect ratio
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;
    
    // Resize image
    let resized = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
    
    // Create UHD canvas with black background
    let mut uhd_image = ImageBuffer::from_fn(3840, 2160, |_, _| {
        Rgba([0, 0, 0, 255])
    });
    
    // Calculate position to center the image
    let x_offset = (3840 - new_width) / 2;
    let y_offset = (2160 - new_height) / 2;
    
    // Copy resized image onto black canvas
    image::imageops::replace(
        &mut uhd_image, 
        &resized, 
        i64::from(x_offset), 
        i64::from(y_offset)
    );
    
    // Get timestamp from EXIF with enhanced error handling
    let datetime = get_image_datetime(path);
    
    // If EXIF fails, use file modification time as fallback (NOT current time)
    let datetime = datetime.or_else(|| {
        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                let system_time = std::time::UNIX_EPOCH + modified.duration_since(std::time::UNIX_EPOCH).ok()?;
                let datetime: chrono::DateTime<chrono::Utc> = system_time.into();
                Some(datetime.naive_utc())
            } else {
                None
            }
        } else {
            None
        }
    });
    
    // Generate filename based on datetime or file modification time
    let filename = match datetime {
        Some(dt) => {
            println!("Using EXIF datetime: {}", dt);
            dt.format("%Y%m%d_%H%M%S").to_string()
        }
        None => {
            // As absolute fallback, use original filename without extension
            let stem = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            println!("No datetime available, using filename: {}", stem);
            stem.to_string()
        }
    };
    
    // Save the image
    let output_path = format!("converted/{}_uhd.jpg", filename);
    uhd_image.save(output_path)?;
    
    Ok(())
}

// Function to extract datetime from EXIF data
fn get_image_datetime(path: &Path) -> Option<NaiveDateTime> {
    let file = std::fs::File::open(path).ok()?;
    let mut reader = std::io::BufReader::new(file);
    let exif = Reader::new().read_from_container(&mut reader).ok()?;
    
    // Try multiple EXIF datetime fields in order of preference
    let datetime_tags = [
        Tag::DateTimeOriginal,  // When the photo was taken
        Tag::DateTimeDigitized, // When the photo was digitized
        Tag::DateTime,          // When the file was last modified
    ];
    
    for &tag in &datetime_tags {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            let datetime_str = field.display_value().to_string();
            let datetime_str = datetime_str.trim_matches('"');
            
            // Debug output (comentado para produção)
            // println!("Found EXIF {:?}: '{}'", tag, datetime_str);
            
            // Try parsing with different formats
            let formats = [
                "%Y:%m:%d %H:%M:%S",  // Standard EXIF format
                "%Y-%m-%d %H:%M:%S",  // Alternative format
                "%Y/%m/%d %H:%M:%S",  // Another alternative
            ];
            
            for format in &formats {
                if let Ok(dt) = NaiveDateTime::parse_from_str(datetime_str, format) {
                    // println!("Successfully parsed datetime: {}", dt);
                    return Some(dt);
                }
            }
            
            // println!("Failed to parse datetime: '{}'", datetime_str);
        }
    }
    
    // println!("No valid EXIF datetime found in file: {}", path.display());
    None
}
