import os
import numpy as np
from tensorflow.keras.preprocessing.image import load_img, img_to_array
import tensorflow as tf
from tensorflow.keras.models import Sequential
from tf_explain.callbacks.activations_visualization import ActivationsVisualizationCallback
from tensorflow.keras.models import load_model
import random
from tensorflow.keras.applications import VGG16
from tensorflow.keras.preprocessing import image
import cv2
import sys
from PIL import Image
import io
import numpy
tf.get_logger().setLevel('CRITICAL')


print("hello")
import sys

def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)

def load_image(image_path, image_size=(128, 128)):
    # Load and preprocess the image
    img = image.load_img(image_path, target_size=image_size)
    img_array = image.img_to_array(img)
    img_array = np.expand_dims(img_array, axis=0)  # Add a batch dimension
    img_array = img_array / 255.0  # Normalize as done in training
    return img_array


# model = load_model('detect_player/saved_model.pb')
dir_path = os.path.dirname(os.path.realpath(__file__))
model = load_model(dir_path + '/detect_player.keras')
#  get input from user
while True:
    # Read the image data from stdin (pipe)
    img_data = sys.stdin.buffer.read(128 * 128 * 3)  # Read all binary data from stdin
    if not img_data:
        print("No data")
        print("c:0, 0")
        continue
    # eprint("Data received: length", len(img_data))

    # Convert the raw bytes into a NumPy array of uint8
    img_array = np.frombuffer(img_data, dtype=np.uint8)

    # Reshape the array to match the image dimensions (128, 128, 3)
    img_array = img_array.reshape((128, 128, 3))
    img_array = np.expand_dims(img_array, axis=0)  # Add a batch dimension

    # Normalize the values to the range [0, 1]
    img_array = img_array / 255.0  # Normalize as done in training

    # Now you have a 3D NumPy array with shape (128, 128, 3)
    # eprint("Shape of the image array:", img_array.shape)
    # save img_array to file
    # with open('img_array.npy', 'wb') as f:
    #     np.save(f, img_array)

    # dont print the loading stuff, hacky way to do it lol
    original_stdout = sys.stdout
    f = open('nul', 'w', encoding='utf-8')
    sys.stdout = f
    predictions = model.predict(img_array)
    sys.stdout = original_stdout
    # eprint(predictions)
    # print output to stdout
    print(f"c:{int(predictions[0][0])}, {int(predictions[0][1])}")
    # eprint(f"c:{int(predictions[0][0])}, {int(predictions[0][1])}")
    # print("Done")
    sys.stdout.flush()
    # break

print("Exit")