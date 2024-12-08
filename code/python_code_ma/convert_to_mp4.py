import os
import subprocess
import sys

# Path to the folder containing images
image_folder = sys.argv[1]

# Output video file
output_video = "output_video.mp4"

# Check if ffmpeg is installed
def check_ffmpeg():
    try:
        subprocess.run(["ffmpeg", "-version"], check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        print("FFmpeg is installed.")
    except FileNotFoundError:
        print("FFmpeg is not installed. Please install FFmpeg to use this script.")
        exit(1)

# Generate the MP4 video from images
def create_video(image_folder, output_video):
    try:
        # Ensure the folder exists
        if not os.path.exists(image_folder):
            raise FileNotFoundError(f"Image folder not found: {image_folder}")

        # Run the ffmpeg command
        command = [
            "ffmpeg",
            "-framerate", "60",  # Frame rate
            "-i", f"{image_folder}/%04d.png",  # Input images with 4-digit numbering
            "-c:v", "libx264",  # Video codec
            "-pix_fmt", "yuv420p",  # Pixel format for compatibility
            "-crf", "23",  # Quality (lower is better, 0-51 range)
            output_video
        ]
        subprocess.run(command, check=True)
        print(f"Video created successfully: {output_video}")
    except subprocess.CalledProcessError as e:
        print(f"FFmpeg error: {e}")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    check_ffmpeg()
    create_video(image_folder, output_video)
