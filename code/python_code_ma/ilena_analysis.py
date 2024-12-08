from pydub import AudioSegment
import numpy as np
import matplotlib.pyplot as plt
import wave
import sys



# Load MP3 file
def read_mp3(file_path):
    audio = AudioSegment.from_file(file_path)
    # Convert to mono for simplicity
    audio = audio.set_channels(1)
    # Get raw data and sample rate
    samples = np.array(audio.get_array_of_samples())
    sample_rate = audio.frame_rate
    return samples, sample_rate

# Load WAV file
def read_wav(file_path):
    with wave.open(file_path, 'rb') as wav_file:
        # Extract audio data
        n_channels = wav_file.getnchannels()
        sample_width = wav_file.getsampwidth()
        sample_rate = wav_file.getframerate()
        n_frames = wav_file.getnframes()

        audio_data = wav_file.readframes(n_frames)
        samples = np.frombuffer(audio_data, dtype=np.int16)
        # Convert to mono if necessary
        if n_channels > 1:
            samples = samples[::n_channels]
    return samples, sample_rate


# Perform Fourier analysis
def fourier_analysis(samples, sample_rate):
    n = len(samples)
    # Perform FFT
    fft_result = np.fft.fft(samples)
    # Get frequencies
    freqs = np.fft.fftfreq(n, d=1 / sample_rate)
    # Take absolute value for amplitude spectrum
    amplitudes = np.abs(fft_result)
    return freqs, amplitudes


# Plot the results
def plot_frequencies(freqs, amplitudes, max_freq=96000):
    # Only keep frequencies in the desired range
    mask = (freqs >= 0) & (freqs <= max_freq)
    plt.figure(figsize=(12, 6))
    plt.plot(freqs[mask], amplitudes[mask])
    plt.title('Frequency Spectrum (0-50 kHz)')
    plt.xlabel('Frequency (Hz)')
    plt.ylabel('Amplitude')
    plt.grid()
    plt.show()


# Main function
def main():
    file_name = sys.argv[1]
    samples, sample_rate = [], []
    if file_name.endswith(".mp3"):
        samples, sample_rate = read_mp3(file_name)
    elif file_name.endswith(".wav"):
        samples, sample_rate = read_wav(file_name)
    else:
        print("Unsupported file format")
        return

    print(f"Sample rate: {sample_rate} Hz, Number of samples: {len(samples)}")

    freqs, amplitudes = fourier_analysis(samples, sample_rate)
    plot_frequencies(freqs, amplitudes)


if __name__ == "__main__":
    main()
