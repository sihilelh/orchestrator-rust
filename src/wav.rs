use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn generate_bytes(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    let mut bytes = Vec::new();

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
    bytes.extend_from_slice(b"RIFF"); // Chunk ID
    bytes.extend_from_slice(&file_size.to_le_bytes()); // File size - 8
    bytes.extend_from_slice(b"WAVE"); // Format

    // ===== fmt CHUNK (24 bytes) =====
    bytes.extend_from_slice(b"fmt "); // Chunk ID (note the space!)
    bytes.extend_from_slice(&16u32.to_le_bytes()); // Chunk size (16 for PCM)
    bytes.extend_from_slice(&1u16.to_le_bytes()); // Audio format (1 = PCM)
    bytes.extend_from_slice(&num_channels.to_le_bytes()); // Number of channels
    bytes.extend_from_slice(&sample_rate.to_le_bytes()); // Sample rate
    bytes.extend_from_slice(&byte_rate.to_le_bytes()); // Byte rate
    bytes.extend_from_slice(&block_align.to_le_bytes()); // Block align
    bytes.extend_from_slice(&bits_per_sample.to_le_bytes()); // Bits per sample

    // ===== data CHUNK (8 bytes + audio data) =====
    bytes.extend_from_slice(b"data"); // Chunk ID
    bytes.extend_from_slice(&data_size.to_le_bytes()); // Data size

    // Write all PCM samples as little-endian bytes
    for &sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }

    bytes
}

pub fn write(filename: &str, samples: &[i16], sample_rate: u32) -> Result<(), Box<dyn Error>> {
    let bytes = generate_bytes(samples, sample_rate);
    let mut file = File::create(filename)?;
    file.write_all(&bytes)?;
    Ok(())
}
