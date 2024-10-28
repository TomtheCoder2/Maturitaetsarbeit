import os
import numpy as np
from tensorflow.keras.preprocessing.image import load_img, img_to_array
import tensorflow as tf
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Conv2D, MaxPooling2D, Flatten, Dense
from tf_explain.callbacks.activations_visualization import ActivationsVisualizationCallback
from tensorflow.keras.models import load_model



def load_data_from_filenames(image_folders, img_size=(154, 143)):
    images = []
    labels = []

    # iterate through all folders in image_folder
    for image_folder in os.listdir(image_folders):
        for img_name in sorted(os.listdir(os.path.join(image_folders, image_folder))):
            if img_name.endswith('.png'):
                # Parse the filename to extract coordinates
                try:
                    x_str, y_str = img_name.split('.')[0].split('_')[-2:]
                    x, y = int(x_str), int(y_str)
                    # print(f"Found coordinates: ({x}, {y})")
                except ValueError:
                    print(f"Skipping invalid file name: {img_name}")
                    continue  # Skip files that don't follow the format

                # Load and preprocess the image
                img_path = os.path.join(image_folders, image_folder, img_name)
                img = load_img(img_path, target_size=img_size)
                img = img_to_array(img)
                img /= 255.0  # Normalize pixel values

                images.append(img)
                labels.append([x, y])  # Store coordinates as labels

    images = np.array(images)
    labels = np.array(labels)
    return images, labels


# Example usage
# image_folder = '../java_ma/fiducial_training_sets/'
image_folder = '../java_ma/real_fiducial_sets/'
X, y = load_data_from_filenames(image_folder)

print("Data loaded successfully!")
print(f"Shape of images: {X.shape}")
print(f"Shape of labels: {y.shape}")


def create_model(input_shape):
    model = Sequential()

    # Convolutional layers
    model.add(Conv2D(32, (3, 3), activation='relu', input_shape=input_shape))
    model.add(MaxPooling2D(pool_size=(2, 2)))

    model.add(Conv2D(64, (3, 3), activation='relu'))
    model.add(MaxPooling2D(pool_size=(2, 2)))

    model.add(Conv2D(128, (3, 3), activation='relu'))
    model.add(MaxPooling2D(pool_size=(2, 2)))

    # Flatten and fully connected layers
    model.add(Flatten())
    model.add(Dense(128, activation='relu'))

    # Output layer: 2 units (x and y coordinates)
    model.add(Dense(2, activation='linear'))  # For regression, we use a linear output

    # Compile the model
    model.compile(optimizer='adam', loss='mean_squared_error', metrics=['mae'])

    return model

# Create the model
input_shape = (154, 143, 3)  # Example image shape
model = create_model(input_shape)

model = load_model('fiducial_coords_model.keras')

X_test, y_test = load_data_from_filenames('../java_ma/fiducial_test_sets/')


callbacks = [
    ActivationsVisualizationCallback(
        validation_data=(X_test, y_test),
        layers_name=["activation_1"],
        output_dir="./output",
    ),
]

# Train the model
history = model.fit(X, y, epochs=50, batch_size=16, validation_split=0.2)

# Save the trained model
model.save('fiducial_coords_model.keras')

