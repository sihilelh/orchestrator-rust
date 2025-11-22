# 🎵 Orchestrator - WAV Audio Synthesizer built using Rust

A learning project that generates WAV audio files from JSON music notation by synthesizing sine waves and Bézier curves, with support for timeline-based composition, ADSR envelopes, and encoding them in PCM format. This is my first Rust application, built to understand how digital audio works at a low level!

## 🎯 What This Does

```
JSON Input → Parse Notes → Calculate Frequencies → Generate Waves (Sine/Bézier) → Apply ADSR Envelope → Mix & PCM Encoding → WAV File
```

Takes musical notes described in JSON, calculates their frequencies, generates sine waves or custom Bézier waveforms for each note, applies ADSR envelopes for natural sound shaping, mixes overlapping notes for chords, converts everything to PCM (Pulse Code Modulation) format, and writes to a proper WAV file with RIFF headers.

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

The orchestrator supports two input formats: **Regular** (sequential notes) and **Timeline** (overlapping notes with precise timing).

### Regular Format (Sequential Notes)

Notes play one after another in sequence. Perfect for simple melodies:

```json
{
  "bpm": 130,
  "notes": [
    { "id": 0, "beats": 1.0, "octave": 4, "amplitude": 1 },
    { "id": 2, "beats": 1.0, "octave": 4, "amplitude": 1 }
  ]
}
```

### Timeline Format (Overlapping Notes & Chords)

**Previously, the orchestrator could only play notes sequentially** - one note after another. The timeline format enables **simultaneous playback** of multiple notes, allowing for chords and complex arrangements.

Enable overlapping notes by setting `"timeline": true`. Notes can start at any time and overlap, creating rich harmonic textures:

```json
{
  "bpm": 145,
  "timeline": true,
  "adsr": {
    "attack": 0.1,
    "decay": 0.1,
    "sustain": 0.25,
    "release": 0.1
  },
  "notes": [
    {
      "id": 7,
      "octave": 4,
      "start_time": 0,
      "duration": 1,
      "amplitude": 0.8
    },
    {
      "id": 7,
      "octave": 5,
      "start_time": 0,
      "duration": 1,
      "amplitude": 0.8
    }
  ]
}
```

**Key differences:**

- Use `start_time` and `duration` (in beats) instead of `beats`
- Notes can overlap to create chords
- Optional `adsr` envelope for natural sound shaping
- Optional `control_points` for Bézier waveforms (see below)

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

**Common Parameters:**

- **`bpm`** (beats per minute): Tempo of the music
- **`id`**: Note ID from 0-11 (see table above)
- **`octave`**: Which octave (typically 0-8, where 4 is middle octave)
- **`amplitude`**: Volume (0.0 to 1.0, where 1.0 is maximum)

**Regular Format Parameters:**

- **`beats`**: Duration of the note in beats (notes play sequentially)

**Timeline Format Parameters:**

- **`start_time`**: When the note starts playing (in beats from the beginning)
- **`duration`**: How long the note plays (in beats)
- **`timeline`**: Set to `true` to enable timeline mode
- **`adsr`** (optional): ADSR envelope configuration (see below)
- **`control_points`** (optional): Array of 4 values [-1.0 to 1.0] for Bézier waveform shaping

### ADSR Envelope (Timeline Format Only)

ADSR (Attack, Decay, Sustain, Release) shapes how notes sound over time, making them sound more natural:

- **`attack`**: Time (in seconds) to reach full amplitude from silence
- **`decay`**: Time (in seconds) to drop from peak to sustain level
- **`sustain`**: Amplitude level (0.0 to 1.0) held during the note
- **`release`**: Time (in seconds) to fade from sustain to silence

**Important**: All ADSR values are optional. If the `adsr` object is not provided, or if individual values are missing, they use these defaults:

- **`attack`**: Defaults to `0.0` (instant start)
- **`decay`**: Defaults to `0.0` (no drop to sustain)
- **`sustain`**: Defaults to `1.0` (full amplitude during sustain - this is the most common case)
- **`release`**: Defaults to `0.0` (instant stop)

This means notes will play at full volume throughout their duration (since sustain=1.0), with instant start and stop unless you specify attack/release times.

Example with ADSR enabled:

```json
{
  "adsr": {
    "attack": 0.1, // Quick attack
    "decay": 0.1, // Quick decay
    "sustain": 0.7, // 70% volume during sustain
    "release": 0.2 // Gentle fade out
  }
}
```

### Bézier Waveforms (Optional)

Instead of pure sine waves, you can use **Bézier curves** to create custom waveforms with unique harmonic characteristics. This allows you to approximate square waves, sawtooth waves, triangle waves, or create entirely new timbres.

#### How Bézier Curves Work for Waveforms

A cubic Bézier curve uses 4 control points to define a smooth curve. For audio waveforms, we use the curve to define the **amplitude (Y-axis) over one complete cycle (X-axis = phase from 0.0 to 1.0)**.

**Why only Y values?**

In traditional 2D Bézier curves, you'd need both X and Y coordinates for each control point. However, for periodic waveforms, the X-axis is always the **phase** (position in the cycle from 0.0 to 1.0), which is automatically calculated based on:

- The sample index
- The frequency
- The sample rate

So we only need to specify the **Y values (amplitudes)** for the 4 control points. The X positions are fixed at:

- P₀ at phase 0.0 (start of cycle)
- P₁ at phase ~0.33 (first control point)
- P₂ at phase ~0.67 (second control point)
- P₃ at phase 1.0 (end of cycle, which wraps to start)

The Bézier formula interpolates between these Y values:

```
B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
```

Where `t` is the phase (0.0 to 1.0) and `P₀, P₁, P₂, P₃` are the Y-amplitude control points.

**Usage:**

Provide 4 control points (each between -1.0 and 1.0):

```json
{
  "control_points": [0.1, 0.2, 0.3, 0.4]
}
```

This works in both Regular and Timeline formats. If not provided, the orchestrator uses sine waves.

## 🎵 Example Outputs

This repository includes two example WAV files demonstrating the synthesizer's capabilities:

### 🎼 octave.wav

A simple C major scale demonstration - plays the notes C-D-E-F-G-A-B-C ascending and then back down. Perfect for understanding basic note sequencing and how the synthesizer handles different pitches.

- **Listen**: [Download octave.wav](https://raw.githubusercontent.com/sihilelh/orchestrator-rust/refs/heads/main/output/octave.wav)
- **Source**: [View octave.json](https://github.com/sihilelh/orchestrator-rust/blob/main/inputs/octave.json) for technical details (BPM, note timings, amplitudes)

### 🐉 test_drive.wav

A simplified interpretation of the opening melody from "Test Drive" - the iconic theme from _How to Train Your Dragon_ that plays during Hiccup's first flight with Toothless. This piece holds special meaning as it represents my first flight with Rust! 🦀

- **Listen**: [Download test_drive.wav](https://raw.githubusercontent.com/sihilelh/orchestrator-rust/refs/heads/main/output/test_drive.wav)
- **Source**: [View test_drive.json](https://github.com/sihilelh/orchestrator-rust/blob/main/inputs/test_drive.json) for technical details (BPM, note timings, amplitudes)

### 🎹 test_drive_timeline.wav

The same "Test Drive" melody, but using the timeline format with ADSR envelopes and overlapping notes for richer, more natural-sounding chords. Demonstrates the power of timeline-based composition!

- **Listen**: [Download test_drive_timeline.wav](https://raw.githubusercontent.com/sihilelh/orchestrator-rust/refs/heads/main/output/test_drive_timeline.wav)
- **Source**: [View test_drive_timeline.json](https://github.com/sihilelh/orchestrator-rust/blob/main/inputs/test_drive_timeline.json) - Features ADSR envelope and overlapping notes

**How to play**: Download the WAV files and open them in any audio player (Windows Media Player, VLC, iTunes, QuickTime, etc.)

### 📊 Format Comparison

| Feature               | Regular Format                           | Timeline Format                                |
| --------------------- | ---------------------------------------- | ---------------------------------------------- |
| **Note Timing**       | Sequential (`beats` field)               | Absolute timing (`start_time` + `duration`)    |
| **Overlapping Notes** | ❌ No - notes play one after another     | ✅ Yes - notes can overlap for chords          |
| **ADSR Envelope**     | ❌ Not supported                         | ✅ Optional - natural sound shaping            |
| **Bézier Waveforms**  | ✅ Supported via `control_points`        | ✅ Supported via `control_points`              |
| **Use Case**          | Simple melodies, sequential compositions | Complex arrangements, chords, polyphonic music |
| **Activation**        | Default (no special field)               | Set `"timeline": true`                         |

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

### 2. Waveform Generation

Sound is vibrations, and musical tones can be represented as waveforms. The orchestrator supports two types:

#### Sine Waves (Default)

Pure sine waves oscillate smoothly between -1 and +1, producing clean, pure tones.

![sin](https://github.com/user-attachments/assets/208d9802-d65d-4e08-adda-78218231e7a8)

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

#### Bézier Curves (Optional)

Bézier curves allow custom waveform shapes by defining 4 control points (Y-amplitude values). This creates more complex, harmonically rich sounds compared to pure sine waves.

**How it works:**

1. **Phase Calculation**: For each sample, we calculate the phase (position in the waveform cycle from 0.0 to 1.0):

   ```rust
   phase = ((sample_index * frequency) / sample_rate).fract()
   ```

2. **Bézier Interpolation**: The phase (0.0 to 1.0) becomes the parameter `t` in the cubic Bézier formula:

   ```
   B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
   ```

3. **Why Only Y Values?**: The X-axis is always the phase (automatically calculated), so we only need Y-amplitude values for the 4 control points. The X positions are implicitly:
   - P₀ at phase 0.0 (cycle start)
   - P₁ influences the first third of the cycle
   - P₂ influences the second third of the cycle
   - P₃ at phase 1.0 (cycle end, wraps to start)

This creates a periodic waveform that repeats smoothly, with the control points shaping the amplitude curve over each cycle. Different control point values create different timbres - for example, values like `[1, -1, 1, -1]` approximate a square wave, while `[0, 1, -1, 0]` creates a triangle-like wave.

### 3. ADSR Envelope (Timeline Format)

**What is ADSR?** ADSR (Attack, Decay, Sustain, Release) is an envelope that shapes how a note's volume changes over time, making synthesized sounds more natural and musical. Think of it like how a piano key press creates a sound that builds up, holds, and fades away - ADSR replicates this behavior digitally.

The envelope has four phases:

- **Attack**: Time to ramp from silence (0) to full amplitude. A quick attack (0.01s) sounds like a pluck, while a slow attack (0.5s) creates a gradual swell.
- **Decay**: Time to drop from peak amplitude down to the sustain level. This creates the initial "punch" or "bite" of the note.
- **Sustain**: The steady amplitude level (0.0 to 1.0) that the note maintains while held. Unlike attack/decay/release (which are times), sustain is an amplitude value.
- **Release**: Time to fade from the sustain level back to silence after the note ends. A longer release creates a smooth tail, preventing abrupt cutoffs.

**Why it matters**: Without ADSR, notes start and stop instantly, creating harsh "clicks" and unnatural sounds. With ADSR, notes have smooth attacks and graceful releases, making the synthesis sound more like real instruments.

The envelope is applied to each note in timeline format. The release phase extends beyond the note's duration to ensure a smooth fade-out, preventing audio artifacts.

### 4. Note Mixing (Timeline Format)

In timeline format, multiple notes can overlap in time. The orchestrator mixes them by summing their sample values at each point in time. A `CONDENSE_CONSTANT` (0.9) is applied to prevent clipping when many notes overlap. Finally, soft clipping using `tanh()` is applied to smoothly handle any remaining peaks.

### 5. PCM (Pulse Code Modulation)

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

### 6. WAV File Format (RIFF/WAV)

A WAV file is a container format following the **RIFF** (Resource Interchange File Format) structure. It consists of **chunks** of data:

![wav_headers](https://github.com/user-attachments/assets/38b9f1d7-422f-42b3-9287-9e75fa8ad4a2)

**Key calculations**:

- `byte_rate` = sample_rate × channels × (bits_per_sample / 8)
- `block_align` = channels × (bits_per_sample / 8)
- `data_size` = number_of_samples × (bits_per_sample / 8)
- `file_size` = 36 + data_size

All multi-byte integers are stored in **little-endian** format (least significant byte first).

## 🏗️ Code Architecture

### Processing Pipeline

![pipeline](https://github.com/user-attachments/assets/7b2d0445-f36a-4a9d-8308-09b5be305b4e)

### Module Breakdown

**`main.rs`**: Orchestrates the entire flow

- Sets sample rate (44.1 kHz standard)
- Coordinates CLI, orchestrator, and WAV writer

**`cli.rs`**: Command-line interface

- Parses input file path from arguments
- Detects timeline vs regular format based on `"timeline"` field
- Loads JSON and deserializes into appropriate Orchestrator type
- Extracts ADSR and control_points from JSON
- Provides user feedback with colored logging
- Extracts filename for output

**`orchestrator.rs`**: Regular (sequential) music representation

- `Note` struct: Holds note data (id, octave, beats, amplitude)
- `Note::frequency()`: Implements equal temperament calculation
- `Orchestrator::pcm_samples()`: Converts entire composition to PCM sequentially
- Supports sine waves and Bézier curves via `control_points`

**`timeline_orchestrator.rs`**: Timeline-based music representation

- `TimelineNote` struct: Holds note data with `start_time` and `duration` (allows overlapping)
- `TimelineOrchestrator::pcm_samples()`: Mixes overlapping notes, applies ADSR envelopes
- Supports both sine waves and Bézier curves
- Uses sample mixing for chords and complex arrangements

**`oscillator.rs`**: Digital signal processing

- `SinOscillator` struct: Represents a sine wave generator
- `BezierOscillator` struct: Generates custom waveforms using Bézier curves
- `.sample()`: Generates floating-point wave sample
- `.pcm_sample()`: Converts to 16-bit PCM with clamping

**`adsr.rs`**: Envelope shaping

- `ADSREnvelope` struct: Manages attack, decay, sustain, release phases
- `.apply()`: Applies envelope to samples over time
- State machine tracks current envelope phase

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

## 🎓 What I Learned

- How WAV files are structured (RIFF format, chunks)
- Converting musical notation to frequencies
- Digital audio fundamentals (sampling, PCM, bit depth)
- Sine wave mathematics and synthesis
- Bézier curve interpolation for custom waveforms
- ADSR envelope implementation for natural sound shaping
- Audio mixing and soft clipping techniques
- Timeline-based composition and overlapping note handling
- Rust basics: modules, error handling, file I/O, enums, state machines
- Working with binary data and byte ordering (little-endian)

## 🛠️ Possible Improvements

Want to contribute or experiment? Here are some ideas:

[x] Add support for different waveforms (square, sawtooth, triangle) - ✅ **Completed!** Using Bézier curves
[x] Support for chords (multiple simultaneous notes) - ✅ **Completed!** Timeline-based system with overlapping notes
[x] GUI for composing music (using WebAssembly and React) - ✅ **Completed!** Visit [https://orchestrator.sihilel.com](https://orchestrator.sihilel.com)
[x] Implement ADSR envelope (Attack, Decay, Sustain, Release) - ✅ **Completed!** Available in timeline format with optional configuration
[] Add effects (reverb, delay, filters)
[] Stereo output support

## 📄 License

This is a learning project - feel free to use, modify, and learn from it!

---

**Made with 🦀 Rust as a learning adventure into digital audio synthesis**
