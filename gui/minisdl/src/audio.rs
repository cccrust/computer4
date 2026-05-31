//! Minimal software audio subsystem.
//!
//! Provides sine-wave oscillators and a simple mixer that produces an
//! interleaved stereo PCM buffer – ready to feed into any real audio backend.

use std::f32::consts::TAU;

use crate::error::Result;

// ── Waveform ──────────────────────────────────────────────────────────────────

/// Simple waveform types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

impl Waveform {
    fn sample(self, phase: f32) -> f32 {
        let p = phase % 1.0;
        match self {
            Self::Sine     => (p * TAU).sin(),
            Self::Square   => if p < 0.5 { 1.0 } else { -1.0 },
            Self::Sawtooth => 2.0 * p - 1.0,
            Self::Triangle => if p < 0.5 { 4.0 * p - 1.0 } else { 3.0 - 4.0 * p },
        }
    }
}

// ── Oscillator ────────────────────────────────────────────────────────────────

/// A single oscillator voice.
pub struct Oscillator {
    pub frequency: f32,
    pub amplitude: f32,
    pub waveform:  Waveform,
    pub pan:       f32,   // -1.0 (left) … +1.0 (right)
    phase:         f32,
    active:        bool,
}

impl Oscillator {
    pub fn new(frequency: f32, waveform: Waveform) -> Self {
        Self {
            frequency,
            amplitude: 0.5,
            waveform,
            pan: 0.0,
            phase: 0.0,
            active: true,
        }
    }

    pub fn sine(frequency: f32) -> Self {
        Self::new(frequency, Waveform::Sine)
    }

    pub fn with_amplitude(mut self, amplitude: f32) -> Self {
        self.amplitude = amplitude.clamp(0.0, 1.0);
        self
    }

    pub fn with_pan(mut self, pan: f32) -> Self {
        self.pan = pan.clamp(-1.0, 1.0);
        self
    }

    pub fn start(&mut self) { self.active = true; }
    pub fn stop(&mut self)  { self.active = false; }
    pub fn is_active(&self) -> bool { self.active }

    /// Advance the oscillator and return `(left_sample, right_sample)`.
    fn next_sample(&mut self, sample_rate: f32) -> (f32, f32) {
        if !self.active {
            return (0.0, 0.0);
        }
        let s = self.waveform.sample(self.phase) * self.amplitude;
        self.phase += self.frequency / sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }
        let l = s * (1.0 - self.pan.max(0.0));
        let r = s * (1.0 + self.pan.min(0.0));
        (l, r)
    }
}

// ── AudioDevice ───────────────────────────────────────────────────────────────

/// Software audio mixer.  Add oscillators, then call [`AudioDevice::fill_buffer`]
/// to get PCM data that you can forward to the OS audio API.
pub struct AudioDevice {
    pub sample_rate: u32,
    pub channels:    u8,
    oscillators:     Vec<Oscillator>,
    master_volume:   f32,
}

impl AudioDevice {
    pub fn new(sample_rate: u32) -> Result<Self> {
        Ok(Self {
            sample_rate,
            channels:      2,
            oscillators:   Vec::new(),
            master_volume: 1.0,
        })
    }

    pub fn add_oscillator(&mut self, osc: Oscillator) -> usize {
        let idx = self.oscillators.len();
        self.oscillators.push(osc);
        idx
    }

    pub fn oscillator(&self, idx: usize) -> Option<&Oscillator> {
        self.oscillators.get(idx)
    }

    pub fn oscillator_mut(&mut self, idx: usize) -> Option<&mut Oscillator> {
        self.oscillators.get_mut(idx)
    }

    pub fn set_master_volume(&mut self, v: f32) {
        self.master_volume = v.clamp(0.0, 1.0);
    }

    /// Fill `out` with interleaved stereo f32 PCM samples.  
    /// `out.len()` must be a multiple of 2 (left, right pairs).
    pub fn fill_buffer(&mut self, out: &mut [f32]) {
        let sr = self.sample_rate as f32;
        for frame in out.chunks_exact_mut(2) {
            let (mut l, mut r) = (0.0f32, 0.0f32);
            for osc in &mut self.oscillators {
                let (sl, sr) = osc.next_sample(sr);
                l += sl;
                r += sr;
            }
            frame[0] = (l * self.master_volume).clamp(-1.0, 1.0);
            frame[1] = (r * self.master_volume).clamp(-1.0, 1.0);
        }
    }

    /// Generate `num_frames` stereo frames as interleaved f32 samples.
    pub fn generate(&mut self, num_frames: usize) -> Vec<f32> {
        let mut buf = vec![0.0f32; num_frames * 2];
        self.fill_buffer(&mut buf);
        buf
    }

    /// Convert f32 samples → i16 PCM (for WAV output etc.).
    pub fn f32_to_i16(samples: &[f32]) -> Vec<i16> {
        samples
            .iter()
            .map(|&s| (s * i16::MAX as f32) as i16)
            .collect()
    }

    /// Generate a raw WAV file in memory (44-byte header + i16 PCM data).
    pub fn generate_wav(&mut self, duration_secs: f32) -> Vec<u8> {
        let num_frames = (self.sample_rate as f32 * duration_secs) as usize;
        let pcm_f32    = self.generate(num_frames);
        let pcm_i16    = Self::f32_to_i16(&pcm_f32);
        let data_bytes = (pcm_i16.len() * 2) as u32;

        let mut wav = Vec::with_capacity(44 + data_bytes as usize);
        // RIFF header
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&(36 + data_bytes).to_le_bytes());
        wav.extend_from_slice(b"WAVE");
        // fmt  chunk
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());       // chunk size
        wav.extend_from_slice(&1u16.to_le_bytes());        // PCM
        wav.extend_from_slice(&2u16.to_le_bytes());        // stereo
        wav.extend_from_slice(&self.sample_rate.to_le_bytes());
        wav.extend_from_slice(&(self.sample_rate * 4).to_le_bytes()); // byte rate
        wav.extend_from_slice(&4u16.to_le_bytes());        // block align
        wav.extend_from_slice(&16u16.to_le_bytes());       // bits per sample
        // data chunk
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&data_bytes.to_le_bytes());
        for s in pcm_i16 {
            wav.extend_from_slice(&s.to_le_bytes());
        }
        wav
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wav_header_magic() {
        let mut dev = AudioDevice::new(44100).unwrap();
        dev.add_oscillator(Oscillator::sine(440.0));
        let wav = dev.generate_wav(0.01);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
    }
}
