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

def load_image(image_path, image_size=(128, 128)):
    # Load and preprocess the image
    img = image.load_img(image_path, target_size=image_size)
    img_array = image.img_to_array(img)
    img_array = np.expand_dims(img_array, axis=0)  # Add a batch dimension
    img_array = img_array / 255.0  # Normalize as done in training
    return img_array

# model = load_model('detect_player/saved_model.pb')
model = load_model('detect_player.keras')
path = '../player_training_images2/41_169_2_0104.png'
if len(sys.argv) > 1:
    path = sys.argv[1]
img = load_image(path)
# read img_array.npy
img2 = np.load('img_array.npy')
# check if they are the same
print(np.array_equal(img, img2))
predictions = model.predict(img)
print(predictions)

# draw red circle on image where ball is detected
def draw_circle(img, coords):
    # img = cv2.imread(image_path)
    # print(img.shape)
    # convert img to cv2 format
    img = cv2.cvtColor(np.array(img), cv2.COLOR_RGB2BGR)
    img = cv2.circle(img, (int(coords[0]), int(coords[1])), 2, (255, 0, 0), -1)
    # cv2.imshow('Image', img)
    # cv2.waitKey(50)
    # cv2.destroyAllWindows()
    cv2.imwrite("output.png", img)

# draw_circle(image.load_img(path, target_size=(128,128)), (predictions[0][0]/132*128, predictions[0][1]/190*128))
draw_circle(image.load_img(path), (predictions[0][0], predictions[0][1]))

print(path)