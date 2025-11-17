use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn write(filename: &str, samples: &[i16], sample_rate: u32) -> Result<()> {
    // Ensure output directory exists
    if let Some(parent) = Path::new(filename).parent() {
        fs::create_dir_all(parent).context(format!(
            "Failed to create output directory: {}",
            parent.display()
        ))?;
    }

    let mut file =
        File::create(filename).context(format!("Failed to create WAV file: {}", filename))?;

    // Audio format parameters
    let num_channels: u16 = 1; // Mono
    let bits_per_sample: u16 = 16; // 16-bit PCM
    let bytes_per_sample: u16 = bits_per_sample / 8;

    // Calculated values
    let byte_rate: u32 = sample_rate * num_channels as u32 * bytes_per_sample as u32;
    let block_align: u16 = num_channels * bytes_per_sample;
    let data_size: u32 = samples.len() as u32 * bytes_per_sample as u32;
    let file_size: u32 = 36 + data_size; // 36 = size of headers (44 total - 8 for RIFF header)

    // ===== RIFF HEADER (12 bytes) =====
    file.write_all(b"RIFF")
        .context("Failed to write RIFF header")?;
    file.write_all(&file_size.to_le_bytes())
        .context("Failed to write file size")?;
    file.write_all(b"WAVE")
        .context("Failed to write WAVE format")?;

    // ===== fmt CHUNK (24 bytes) =====
    file.write_all(b"fmt ")
        .context("Failed to write fmt chunk ID")?;
    file.write_all(&16u32.to_le_bytes())
        .context("Failed to write fmt chunk size")?;
    file.write_all(&1u16.to_le_bytes())
        .context("Failed to write audio format")?;
    file.write_all(&num_channels.to_le_bytes())
        .context("Failed to write number of channels")?;
    file.write_all(&sample_rate.to_le_bytes())
        .context("Failed to write sample rate")?;
    file.write_all(&byte_rate.to_le_bytes())
        .context("Failed to write byte rate")?;
    file.write_all(&block_align.to_le_bytes())
        .context("Failed to write block align")?;
    file.write_all(&bits_per_sample.to_le_bytes())
        .context("Failed to write bits per sample")?;

    // ===== data CHUNK (8 bytes + audio data) =====
    file.write_all(b"data")
        .context("Failed to write data chunk ID")?;
    file.write_all(&data_size.to_le_bytes())
        .context("Failed to write data size")?;

    // Write all PCM samples as little-endian bytes
    for &sample in samples {
        file.write_all(&sample.to_le_bytes())
            .context("Failed to write PCM sample data")?;
    }

    Ok(())
}
