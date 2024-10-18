import tensorflow as tf
from tensorflow.keras.preprocessing import image
import numpy as np
import os

# Load the saved model
model = tf.keras.models.load_model('fiducial_classifier_model.keras')


# Define a function to predict the fiducial class from a PNG image
def predict_fiducial(image_path):
    img_height = 154
    img_width = 143

    # Load and preprocess the image
    img = image.load_img(image_path, target_size=(img_height, img_width))
    img_array = image.img_to_array(img)
    img_array = np.expand_dims(img_array, axis=0)  # Add a batch dimension
    # img_array = img_array / 255.0  # Normalize as done in training

    # Make predictions
    predictions = model.predict(img_array)
    predicted_class = np.argmax(predictions, axis=1)

    # Class labels (assuming 0 for fiducial_1, 1 for fiducial_2, etc.)
    class_labels = ['fiducial_1', 'fiducial_2', 'fiducial_3', 'fiducial_4']

    # Return the predicted class label
    return class_labels[predicted_class[0]]


# get all files of the ./input folder
for root, dirs, files in os.walk("./inputs"):
    for file in files:
        if file.endswith(".png"):
            print(f"Predicted fiducial for {file}: {predict_fiducial(os.path.join(root, file))}")
# Example usage:
# image_path = '../java_ma/fiducial_sets/fiducial_1/img_2.png'
# predicted_fiducial = predict_fiducial(image_path)
# print(f"The predicted fiducial is: {predicted_fiducial}")
