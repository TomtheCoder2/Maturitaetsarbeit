import numpy as np
from PIL import Image, ImageOps
import os
import random
import math

class TrainingSet:
    def __init__(self, X, Y):
        self.tasks = X
        self.targets = Y

    def __str__(self):
        return f"TrainingSet{{tasks={self.tasks}, targets={self.targets}}}"

    def append(self, other):
        self.tasks = np.concatenate((self.tasks, other.tasks), axis=0)
        self.targets = np.concatenate((self.targets, other.targets), axis=0)

    def write_to_file(self, path):
        np.save(path + "_tasks.npy", self.tasks)
        np.save(path + "_targets.npy", self.targets)

def place_fiducial(number, image, degrees, n, m, x_offset, y_offset, skip):
    # Resize the image by skipping pixels
    new_image = image.resize((image.width // skip, image.height // skip))

    # Read the fiducial image and rotate it
    fiducial = Image.open(f"./fiducials/fiducial_{number}.png").convert("RGBA")
    fiducial = fiducial.rotate(degrees, expand=True)

    # Calculate the size and position of the fiducial
    width, height = new_image.size
    fiducial_width = int(width / n)
    fiducial_height = int(height / m)
    fiducial = fiducial.resize((fiducial_width, fiducial_height), Image.Resampling.LANCZOS)

    x = (width - fiducial_width) // 2 + x_offset
    y = (height - fiducial_height) // 2 + y_offset

    # Create a new image to paste the fiducial onto
    new_image = new_image.convert("RGBA")

    # Iterate over each pixel in the fiducial image
    for i in range(fiducial_width):
        for j in range(fiducial_height):
            try:
                fiducial_pixel = fiducial.getpixel((i, j))
                if fiducial_pixel[3] != 0:  # Check if the pixel is not transparent
                    original_pixel = new_image.getpixel((x + i, y + j))
                    fid = 0.8
                    org = 1 - fid

                    # Compute the new RGB values
                    red = int(org * original_pixel[0] + fid * fiducial_pixel[0])
                    green = int(org * original_pixel[1] + fid * fiducial_pixel[1])
                    blue = int(org * original_pixel[2] + fid * fiducial_pixel[2])

                    # Set the new pixel value
                    new_image.putpixel((x + i, y + j), (red, green, blue, 255))
            except Exception as e:
                print(f"Error processing pixel ({i}, {j}): {e}")

    # Calculate the midpoint of the fiducial
    mid_x = x + fiducial_width // 2
    mid_y = y + fiducial_height // 2

    return new_image, (mid_x, mid_y)

def gen_training_set(all_images, amount_per_image, prefix):
    inputs = []
    targets = []

    fiducial_limits = (1, 4)
    degree_limits = (-5, 5)
    n_limits = (1.8, 2.2)
    m_limits = (1.8, 2.2)
    x_offset_limits = (-20, 20)
    y_offset_limits = (-20, 20)

    if not os.path.exists("./training_set"):
        os.makedirs("./training_set")

    skip = 2
    side = math.ceil(math.sqrt(amount_per_image))
    whole_general_image = Image.new('RGB', (all_images[0].width // skip * side * 2 + 1, all_images[0].height // skip * side * 2 + 1))

    amount = 0
    for img in all_images:
        general_image = Image.new('RGB', (img.width // skip * side, img.height // skip * side))

        for i in range(amount_per_image):
            fiducial = random.randint(fiducial_limits[0], fiducial_limits[1])
            d = random.randint(degree_limits[0], degree_limits[1])
            n = random.uniform(n_limits[0], n_limits[1])
            m = random.uniform(m_limits[0], m_limits[1])
            x_offset = random.randint(x_offset_limits[0], x_offset_limits[1])
            y_offset = random.randint(y_offset_limits[0], y_offset_limits[1])

            placed_fiducial, (mid_x, mid_y) = place_fiducial(fiducial, img, d, n, m, x_offset, y_offset, skip)

            if i == 0:
                placed_fiducial.save(f"./training_set/img_{i}.png")

            copy_image = placed_fiducial.copy()
            radius = 5
            for j in range(-radius, radius + 1):
                for k in range(-radius, radius + 1):
                    if j * j + k * k <= radius * radius:
                        try:
                            copy_image.putpixel((mid_x + j, mid_y + k), (255, 0, 0))
                        except:
                            pass

            general_image.paste(copy_image, ((i % side) * placed_fiducial.width, (i // side) * placed_fiducial.height))

            input_data = []
            for j in range(placed_fiducial.width):
                for k in range(placed_fiducial.height):
                    r, g, b = placed_fiducial.getpixel((j, k))
                    input_data.extend([r, g, b])
            inputs.append(input_data)

            target = [0] * (4 + img.height + img.width)
            target[fiducial - 1] = 1
            target[4 + mid_x] = 1
            target[4 + img.width + mid_y] = 1
            targets.append(target)

        whole_general_image.paste(general_image, ((amount % 2) * general_image.width, (amount // 2) * general_image.height))
        amount += 1

        general_image.save(f"./training_set/{prefix}general_image.png")

    whole_general_image.save(f"./training_set/{prefix}whole_general_image.png")

    inputs_array = np.array(inputs)
    targets_array = np.array(targets)

    return TrainingSet(inputs_array, targets_array)

def main():
    image = Image.open("./base_img5.png")
    width, height = image.width, image.height
    print(f"Width: {width} Height: {height}")

    vertical_split = 4
    horizontal_split = 6
    top_left = image.crop((0, 0, width // horizontal_split, height // vertical_split))
    top_right = image.crop((width - width // horizontal_split, 0, width, height // vertical_split))
    bottom_left = image.crop((0, height - height // vertical_split, width // horizontal_split, height))
    bottom_right = image.crop((width - width // horizontal_split, height - height // vertical_split, width, height))

    test_set = gen_training_set([top_left, top_right, bottom_left, bottom_right], 2, "test_")
    test_set.write_to_file("test_set")

    n = 225
    training_set = gen_training_set([top_left, top_right, bottom_left, bottom_right], n, "training_")
    training_set.write_to_file("training_set")

    input_layer_size = len(test_set.tasks[0])
    output_layer_size = len(test_set.targets[0])
    print(f"Input layer size: {input_layer_size} Output layer size: {output_layer_size}")

    print(f"Amount of tasks: {len(test_set.tasks)} Amount of targets: {len(test_set.targets)}")

if __name__ == "__main__":
    main()
