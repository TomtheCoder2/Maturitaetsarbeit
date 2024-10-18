import os
import numpy as np
from tensorflow.keras.preprocessing.image import load_img, img_to_array
import tensorflow as tf
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Conv2D, MaxPooling2D, Flatten, Dense
import cv2

# Load the trained model
model = tf.keras.models.load_model('fiducial_coords_model.keras')


# Predict the midpoint of a new fiducial image
def predict_midpoint(image_path, img_size=(154, 143)):
    img = load_img(image_path, target_size=img_size)
    img = img_to_array(img)
    img = np.expand_dims(img / 255.0, axis=0)  # Normalize and add batch dimension

    # # Load and preprocess the image
    # img = load_img(image_path, target_size=img_size)
    # img = img_to_array(img)
    # img /= 255.0  # Normalize pixel values

    # Predict x, y coordinates
    pred_coords = model.predict(img)
    return pred_coords[0]

for img in os.listdir('./inputs'):
    # Example usage:
    # image_path = './0_67_53.png'
    image_path = os.path.join('./inputs', img)
    predicted_coords = predict_midpoint(image_path)
    print(f"Predicted midpoint coordinates: {predicted_coords}")

    # draw the image with a cricle at the predicted midpoint
    def draw_circle(image_path, coords):
        img = cv2.imread(image_path)
        print(img.shape)
        img = cv2.circle(img, (int(coords[0]), int(coords[1])), 5, (0, 255, 0), -1)
        cv2.imshow('Image', img)
        cv2.waitKey(0)
        cv2.destroyAllWindows()

    draw_circle(image_path, predicted_coords)
