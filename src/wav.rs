use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn write(filename: &str, samples: &[i16], sample_rate: u32) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;

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
    file.write_all(b"RIFF")?; // Chunk ID
    file.write_all(&file_size.to_le_bytes())?; // File size - 8
    file.write_all(b"WAVE")?; // Format

    // ===== fmt CHUNK (24 bytes) =====
    file.write_all(b"fmt ")?; // Chunk ID (note the space!)
    file.write_all(&16u32.to_le_bytes())?; // Chunk size (16 for PCM)
    file.write_all(&1u16.to_le_bytes())?; // Audio format (1 = PCM)
    file.write_all(&num_channels.to_le_bytes())?; // Number of channels
    file.write_all(&sample_rate.to_le_bytes())?; // Sample rate
    file.write_all(&byte_rate.to_le_bytes())?; // Byte rate
    file.write_all(&block_align.to_le_bytes())?; // Block align
    file.write_all(&bits_per_sample.to_le_bytes())?; // Bits per sample

    // ===== data CHUNK (8 bytes + audio data) =====
    file.write_all(b"data")?; // Chunk ID
    file.write_all(&data_size.to_le_bytes())?; // Data size

    // Write all PCM samples as little-endian bytes
    for &sample in samples {
        file.write_all(&sample.to_le_bytes())?;
    }

    Ok(())
}
