import sys
from PIL import Image
import io


def main():
    # Read the image data from stdin (pipe)
    img_data = sys.stdin.buffer.read(128 * 128 * 3)  # Read all binary data from stdin
    print("d")
    print("Data received: length", len(img_data))
    print("Done")

    # Open the image using the binary data
    image = Image.open(io.BytesIO(img_data))

    # Process the image (show it in this case)
    image.show()  # You can perform other operations on the image here


if __name__ == "__main__":
    main()
# main()
