# 🎵 Orchestrator - WAV Audio Synthesizer built using Rust

A learning project that generates WAV audio files from JSON music notation by synthesizing sine waves and encoding them in PCM format. This is my first Rust application, built to understand how digital audio works at a low level!

## 🎯 What This Does

```
JSON Input → Parse Notes → Calculate Frequencies → Generate Sine Waves → PCM Encoding → WAV File
```

Takes musical notes described in JSON, calculates their frequencies, generates sine waves for each note, converts them to PCM (Pulse Code Modulation) format, and writes everything to a proper WAV file with RIFF headers.

## 🚀 Quick Start

### Build & Run

```bash
# Build the project
cargo build --release

# Run with an input JSON file
cargo run --release inputs/octave.json

# Output will be in: output/<filename>.wav
```

### Requirements

- Rust (2021 edition or later)
- Dependencies: `serde`, `serde_json`

## 📝 JSON Input Format

Create a JSON file with your musical composition:

```json
{
  "bpm": 130,
  "notes": [
    { "id": 0, "beats": 1.0, "octave": 4, "amplitude": 1 },
    { "id": 2, "beats": 1.0, "octave": 4, "amplitude": 1 }
  ]
}
```

### Note ID Reference

Each note has a unique ID (0-11) representing the 12 semitones in an octave:

```
0  = C
1  = C♯ / D♭
2  = D
3  = D♯ / E♭
4  = E
5  = F
6  = F♯ / G♭
7  = G
8  = G♯ / A♭
9  = A
10 = A♯ / B♭
11 = B
```

### Parameters

- **`bpm`** (beats per minute): Tempo of the music
- **`id`**: Note ID from 0-11 (see table above)
- **`beats`**: Duration of the note in beats
- **`octave`**: Which octave (typically 0-8, where 4 is middle octave)
- **`amplitude`**: Volume (0.0 to 1.0, where 1.0 is maximum)

## 🧠 Key Concepts & Implementation

### 1. Frequency Calculation (Equal Temperament Tuning)

Western music uses the **A440** standard, where the note A4 (middle A) vibrates at exactly 440 Hz. All other notes are calculated using the **equal temperament** formula:

```
f = 440 × 2^((n - 9) / 12)
```

Where:

- `f` = frequency in Hz
- `n` = semitone offset from C4
- `440` = A4 frequency (standard tuning)
- `9` = A is the 9th semitone in the octave (counting from C=0)
- `12` = semitones per octave

**Example**: To find C4 (middle C):

- C is ID 0, which is 9 semitones below A4
- `n = 0 - 9 = -9`
- `f = 440 × 2^(-9/12) = 440 × 0.5946 ≈ 261.63 Hz`

For different octaves, we add `12 × (octave - 4)` to the semitone count:

```rust
let n = (note_id - 9) + 12 × (octave - 4)
let frequency = 440.0 * 2.0_f64.powf(n / 12.0)
```

### 2. Sine Wave Generation

Sound is vibrations, and pure musical tones can be represented as **sine waves**. A sine wave oscillates smoothly between -1 and +1.

```
         1.0  ╱╲      ╱╲      ╱╲
              ╱  ╲    ╱  ╲    ╱  ╲
    ─────────╯────╲──╯────╲──╯────╲───── time
                   ╲╱      ╲╱      ╲╱
        -1.0

         |<─ Period ─>|
```

The formula to generate a sample at any point in time:

```
sample(t) = amplitude × sin(2π × frequency × t)
```

Where:

- `t` = time in seconds = sample_index / sample_rate
- `2π` = one complete cycle (360 degrees in radians)
- `frequency` = how many cycles per second (Hz)
- `amplitude` = volume (0.0 to 1.0)

**In code**:

```rust
let t = sample_index as f64 / sample_rate as f64;
let sample = amplitude * (2.0 * PI * frequency * t).sin();
```

### 3. PCM (Pulse Code Modulation)

Computers can't store continuous sine waves (-1.0 to +1.0), so we **digitize** them into discrete integer values. This is called **PCM encoding**.

**16-bit PCM** uses signed integers from **-32,767 to +32,767**:

| Sine Wave (float) | PCM (16-bit int) |
| ----------------- | ---------------- |
| 1.0               | +32,767          |
| 0.5               | +16,383          |
| 0.0               | 0                |
| -0.5              | -16,384          |
| -1.0              | -32,767          |

**Conversion formula**:

```rust
let pcm_value = (float_sample * 32767.0) as i16;
```

We also **clamp** values to prevent distortion:

```rust
let float_sample = sine_sample.clamp(-1.0, 1.0);
```

### 4. WAV File Format (RIFF/WAV)

A WAV file is a container format following the **RIFF** (Resource Interchange File Format) structure. It consists of **chunks** of data:

```
┌─────────────────────────────────────┐
│      RIFF HEADER (12 bytes)         │
│  ┌─────────────────────────────┐   │
│  │ "RIFF" (4 bytes)            │   │  ← Identifies file type
│  │ File Size - 8 (4 bytes)     │   │  ← Total file size minus 8
│  │ "WAVE" (4 bytes)            │   │  ← Wave format identifier
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
┌─────────────────────────────────────┐
│      fmt CHUNK (24 bytes)           │
│  ┌─────────────────────────────┐   │
│  │ "fmt " (4 bytes)            │   │  ← Format chunk marker
│  │ Chunk Size: 16 (4 bytes)    │   │  ← Size of fmt data
│  │ Audio Format: 1 (2 bytes)   │   │  ← 1 = PCM
│  │ Channels: 1 (2 bytes)       │   │  ← Mono audio
│  │ Sample Rate: 44100 (4 bytes)│   │  ← 44.1 kHz
│  │ Byte Rate (4 bytes)         │   │  ← Bytes per second
│  │ Block Align (2 bytes)       │   │  ← Bytes per sample
│  │ Bits Per Sample: 16 (2 bytes)  │  ← 16-bit PCM
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
┌─────────────────────────────────────┐
│      data CHUNK (8 + N bytes)       │
│  ┌─────────────────────────────┐   │
│  │ "data" (4 bytes)            │   │  ← Data chunk marker
│  │ Data Size (4 bytes)         │   │  ← Size of audio data
│  │ ┌─────────────────────────┐ │   │
│  │ │  PCM Sample 1 (2 bytes) │ │   │  ← Actual audio data
│  │ │  PCM Sample 2 (2 bytes) │ │   │  ← One 16-bit int per
│  │ │  PCM Sample 3 (2 bytes) │ │   │  ← sample
│  │ │         ...             │ │   │
│  │ └─────────────────────────┘ │   │
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
```

**Key calculations**:

- `byte_rate` = sample_rate × channels × (bits_per_sample / 8)
- `block_align` = channels × (bits_per_sample / 8)
- `data_size` = number_of_samples × (bits_per_sample / 8)
- `file_size` = 36 + data_size

All multi-byte integers are stored in **little-endian** format (least significant byte first).

## 🏗️ Code Architecture

### Processing Pipeline

```
┌──────────────┐
│   main.rs    │  Entry point
└──────┬───────┘
       │
       ├─► get_filepath()         ─┐
       │                           │
       ├─► get_music_input()       ├─── cli.rs
       │                           │   (Argument parsing)
       ├─► get_filename()          ─┘
       │
       ▼
┌──────────────────┐
│  Orchestrator    │  orchestrator.rs
│  ┌────────────┐  │  (Music logic)
│  │ bpm: 130   │  │
│  │ notes: []  │  │
│  └────────────┘  │
└────────┬─────────┘
         │
         │ .pcm_samples(sample_rate)
         │
         ├─► For each Note:
         │    ├─► Calculate frequency
         │    │    (Equal temperament formula)
         │    │
         │    ├─► Calculate duration
         │    │    (beats × seconds_per_beat)
         │    │
         │    └─► Generate samples
         │         ┌──────────────────┐
         │         │  SinOscillator   │  oscillator.rs
         │         │                  │  (Wave generation)
         │         │  .sample(i)      │  ← Sine wave
         │         │  .pcm_sample(i)  │  ← PCM conversion
         │         └──────────────────┘
         │
         ▼
    Vec<i16> PCM samples
         │
         ▼
┌──────────────────┐
│     wav.rs       │  (File writing)
│                  │
│  write_wav()     │
│   ├─► RIFF header
│   ├─► fmt chunk
│   └─► data chunk + samples
└──────────────────┘
         │
         ▼
    📁 output/octave.wav
```

### Module Breakdown

**`main.rs`**: Orchestrates the entire flow

- Sets sample rate (44.1 kHz standard)
- Coordinates CLI, orchestrator, and WAV writer

**`cli.rs`**: Command-line interface

- Parses input file path from arguments
- Loads JSON and deserializes into Orchestrator
- Extracts filename for output

**`orchestrator.rs`**: Music representation

- `Note` struct: Holds note data (id, octave, beats, amplitude)
- `Note::frequency()`: Implements equal temperament calculation
- `Orchestrator::pcm_samples()`: Converts entire composition to PCM

**`oscillator.rs`**: Digital signal processing

- `SinOscillator` struct: Represents a sine wave generator
- `.sample()`: Generates floating-point sine wave sample
- `.pcm_sample()`: Converts to 16-bit PCM with clamping

**`wav.rs`**: File format encoding

- Writes RIFF header (12 bytes)
- Writes fmt chunk (24 bytes)
- Writes data chunk header (8 bytes)
- Writes all PCM samples as little-endian bytes

## 📚 Learn More

**WAV Format**:

- [WAV Specification](http://soundfile.sapp.org/doc/WaveFormat/)
- [RIFF Format](https://en.wikipedia.org/wiki/Resource_Interchange_File_Format)

**Digital Audio**:

- [PCM Encoding](https://en.wikipedia.org/wiki/Pulse-code_modulation)
- [Sample Rate & Bit Depth](https://www.izotope.com/en/learn/digital-audio-basics-sample-rate-and-bit-depth.html)

**Music Theory**:

- [Equal Temperament](https://en.wikipedia.org/wiki/Equal_temperament)
- [Musical Note Frequencies](https://pages.mtu.edu/~suits/notefreqs.html)

**Rust Audio**:

- [cpal](https://github.com/RustAudio/cpal) - Cross-platform audio I/O
- [rodio](https://github.com/RustAudio/rodio) - Audio playback library

## 🎓 What I Learned

- How WAV files are structured (RIFF format, chunks)
- Converting musical notation to frequencies
- Digital audio fundamentals (sampling, PCM, bit depth)
- Sine wave mathematics and synthesis
- Rust basics: modules, error handling, file I/O
- Working with binary data and byte ordering (little-endian)

## 🛠️ Possible Improvements

Want to contribute or experiment? Here are some ideas:

- Add support for different waveforms (square, sawtooth, triangle)
- Implement ADSR envelope (Attack, Decay, Sustain, Release)
- Support for chords (multiple simultaneous notes)
- Add effects (reverb, delay, filters)
- Stereo output support
- Real-time audio playback
- GUI for composing music

## 📄 License

This is a learning project - feel free to use, modify, and learn from it!

---

**Made with 🦀 Rust as a learning adventure into digital audio synthesis**
