import os
import numpy as np
from tensorflow.keras.preprocessing.image import load_img, img_to_array
import tensorflow as tf
import math
import numpy as np
from PIL import Image, ImageOps
import os
import random
import math
import cv2
import sys

def predict_midpoint(model, image_path, img_size=(154, 143)):
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

# Define a function to predict the fiducial class from a PNG image
def predict_fiducial(model, image_path, img_size=(154, 143)):
    img = load_img(image_path, target_size=img_size)
    img_array = img_to_array(img)
    img_array = np.expand_dims(img_array, axis=0)  # Add a batch dimension
    # img_array = img_array / 255.0  # Normalize as done in training

    # Make predictions
    predictions = model.predict(img_array)
    predicted_class = np.argmax(predictions, axis=1)

    # Class labels (assuming 0 for fiducial_1, 1 for fiducial_2, etc.)
    class_labels = ['fiducial_1', 'fiducial_2', 'fiducial_3', 'fiducial_4']

    # Return the predicted class label
    return class_labels[predicted_class[0]]

# get first argument (image path) (or else base_img.png)
image_path = sys.argv[1] if len(sys.argv) > 1 else "base_img.png"
image = Image.open(image_path)
width, height = image.width, image.height
print(f"Width: {width} Height: {height}")

vertical_split = 4
horizontal_split = 6
top_left = image.crop((0, 0, width // horizontal_split, height // vertical_split))
top_right = image.crop((width - width // horizontal_split, 0, width, height // vertical_split))
bottom_left = image.crop((0, height - height // vertical_split, width // horizontal_split, height))
bottom_right = image.crop((width - width // horizontal_split, height - height // vertical_split, width, height))
# save the images in ./inputs
top_left.save("./inputs/top_left.png")
top_right.save("./inputs/top_right.png")
bottom_left.save("./inputs/bottom_left.png")
bottom_right.save("./inputs/bottom_right.png")
# and also in ./tmp
top_left.save("./tmp/top_left.png")
top_right.save("./tmp/top_right.png")
bottom_left.save("./tmp/bottom_left.png")
bottom_right.save("./tmp/bottom_right.png")

def draw_circle(img, coords):
    # img = cv2.imread(image_path)
    # print(img.shape)
    # convert img to cv2 format
    img = cv2.cvtColor(np.array(img), cv2.COLOR_RGB2BGR)
    img = cv2.circle(img, (int(coords[0]), int(coords[1])), 5, (0, 255, 0), -1)
    cv2.imshow('Image', img)
    cv2.waitKey(50)
    cv2.destroyAllWindows()

classify_model = tf.keras.models.load_model("fiducial_classifier_model.keras")
coords_model = tf.keras.models.load_model("fiducial_coords_model.keras")
#  classify the images
print("Classifying images")
top_left_fiducial_number = predict_fiducial(classify_model, "./tmp/top_left.png")
print("Top Left: ",top_left_fiducial_number)
top_left.save("./tmp/" + top_left_fiducial_number + ".png")

top_right_fiducial_number = predict_fiducial(classify_model, "./tmp/top_right.png")
print("Top Right: ", top_right_fiducial_number)
top_right.save("./tmp/" + top_right_fiducial_number + ".png")

bottom_left_fiducial_number = predict_fiducial(classify_model, "./tmp/bottom_left.png")
print("Bottom Left: ", bottom_left_fiducial_number)
bottom_left.save("./tmp/" + bottom_left_fiducial_number + ".png")

bottom_right_fiducial_number = predict_fiducial(classify_model, "./tmp/bottom_right.png")
print("Bottom Right: ", bottom_right_fiducial_number)
bottom_right.save("./tmp/" + bottom_right_fiducial_number + ".png")

#  get the coords
print("Getting coords")
top_left_coords = predict_midpoint(coords_model, "./tmp/top_left.png")
draw_circle(top_left, top_left_coords)
print("Top Left: ", top_left_coords)

top_right_coords = predict_midpoint(coords_model, "./tmp/top_right.png")
draw_circle(top_right, top_right_coords)
print("Top Right: ", top_right_coords)

bottom_left_coords = predict_midpoint(coords_model, "./tmp/bottom_left.png")
draw_circle(bottom_left, bottom_left_coords)
print("Bottom Left: ", bottom_left_coords)

bottom_right_coords = predict_midpoint(coords_model, "./tmp/bottom_right.png")
draw_circle(bottom_right, bottom_right_coords)
print("Bottom Right: ", bottom_right_coords)

real_top_left_coords = (top_left_coords[0], top_left_coords[1])
real_top_right_coords = (top_right_coords[0] + width - width // horizontal_split, top_right_coords[1])
real_bottom_left_coords = (bottom_left_coords[0], bottom_left_coords[1] + height - height // vertical_split)
real_bottom_right_coords = (bottom_right_coords[0] + width - width // horizontal_split, bottom_right_coords[1] + height - height // vertical_split)
# draw the midpoints on the original image
img = cv2.cvtColor(np.array(image), cv2.COLOR_RGB2BGR)
img = cv2.circle(img, (int(real_top_left_coords[0]), int(real_top_left_coords[1])), 7, (0, 255, 0), -1)
img = cv2.circle(img, (int(real_top_right_coords[0]), int(real_top_right_coords[1])), 7, (255, 0, 0), -1)
img = cv2.circle(img, (int(real_bottom_left_coords[0]), int(real_bottom_left_coords[1])), 7, (0, 255, 255), -1)
img = cv2.circle(img, (int(real_bottom_right_coords[0]), int(real_bottom_right_coords[1])), 7, (255, 255, 0), -1)
# cv2.imshow('Whole Image', img)
# cv2.waitKey(0)
# cv2.destroyAllWindows()
cv2.imwrite("whole_image.png", img)
# write the midpoints to a file
with open("midpoints.txt", "w") as f:
    f.write(f"Top Left: {real_top_left_coords} {top_left_fiducial_number}\n")
    f.write(f"Top Right: {real_top_right_coords} {top_right_fiducial_number}\n")
    f.write(f"Bottom Left: {real_bottom_left_coords} {bottom_left_fiducial_number}\n")
    f.write(f"Bottom Right: {real_bottom_right_coords} {bottom_right_fiducial_number}\n")
    f.write(f"Width: {width}\n")
    f.write(f"Height: {height}\n")

# Read the pixel coordinates from the file
# pixel_points = read_midpoints(file_path)
pixel_points = np.array([real_top_left_coords, real_top_right_coords, real_bottom_left_coords, real_bottom_right_coords], dtype=np.float32)

# Define the real-world coordinates of the points (in mm)
real_points = np.array([
    [78, 65],  # Top Left fiducial_2
    [1225 - 90, 65],  # Top Right fiducial_1
    [85, 723 - 87],  # Bottom Left fiducial_3
    [1225 - 72, 723 - 70]  # Bottom Right fiducial_4
], dtype=np.float32)

# Compute the homography matrix
homography_matrix, status = cv2.findHomography(pixel_points, real_points)

np.savetxt('homography_matrix.csv', homography_matrix, delimiter=',')