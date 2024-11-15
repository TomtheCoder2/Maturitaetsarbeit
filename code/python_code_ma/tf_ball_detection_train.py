import os
import numpy as np
from tensorflow.keras.preprocessing.image import load_img, img_to_array
import tensorflow as tf
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Conv2D, MaxPooling2D, Flatten, Dense
from tf_explain.callbacks.activations_visualization import ActivationsVisualizationCallback
from tensorflow.keras.models import load_model
from tensorflow.keras import layers, models
import random
from tensorflow.keras.applications import MobileNetV2
from keras.layers import Input




img_size = (928, 572)
input_shape = (128, 128, 3)
img_size = (input_shape[0], input_shape[1])

gpus = tf.config.experimental.list_physical_devices('GPU')
if gpus:
    # Restrict TensorFlow to only allocate 1GB of memory on the first GPU
    try:
        tf.config.experimental.set_virtual_device_configuration(gpus[0],
       [tf.config.experimental.VirtualDeviceConfiguration(memory_limit=8*1024)])
        logical_gpus = tf.config.experimental.list_logical_devices('GPU')
        print(len(gpus), "Physical GPUs,", len(logical_gpus), "Logical GPUs")
    except RuntimeError as e:
        # Virtual devices must be set before GPUs have been initialized
        print(e)


def load_data_from_filenames(image_folders, img_size=img_size):
    images = []
    labels = []

    counter = 0
    files = []
    for img_name in sorted(os.listdir(image_folders)):
        counter += 1
        if img_name.endswith('.png'):
            files.append(img_name)
    # shuffle the files
    random.shuffle(files)
    print(f"Total images: {counter}")
    counter = 0
    # iterate through all folders in image_folder
    # for image_folder in os.listdir(image_folders):
    for img_name in files:
        if img_name.endswith('.png'):
            # Parse the filename to extract coordinates
            try:
                x_str, y_str = img_name.split('.')[0].split('_')[0:2]
                x, y = int(x_str), int(y_str)
                x = 0
                # print(f"Found coordinates: ({x}, {y}) in {img_name}")
            except ValueError:
                print(f"Skipping invalid file name: {img_name}")
                continue  # Skip files that don't follow the format

            # Load and preprocess the image
            img_path = os.path.join(image_folders, img_name)
            img = load_img(img_path, target_size=img_size)
            img = img_to_array(img)
            img /= 255.0  # Normalize pixel values

            images.append(img)
            labels.append([x, y])  # Store coordinates as labels
        counter += 1
        if counter % 100 == 0:
            print(f"Processed {counter} images")
        if counter > 800:
            break

    images = np.array(images)
    labels = np.array(labels)
    return images, labels


# Example usage
# image_folder = '../java_ma/fiducial_training_sets/'
# image_folder = '../java_ma/real_fiducial_sets/'
# image_folder = '../recording_sub_2024-11-05_19-01-50'
# image_folder = '../ball_training_images'
image_folder = '../player_training_images2'
x, y = load_data_from_filenames(image_folder, img_size)

print("Data loaded successfully length: ", len(x))

# convert 80 percent of the data to training data
# x = x[:int(len(x) * 0.8)]
# y = y[:int(len(y) * 0.8)]
# convert 20 percent of the data to testing data
x_test = x[int(len(x) * 0.8):]
y_test = y[int(len(y) * 0.8):]

print("Data loaded successfully!")
print(f"Shape of images: {x.shape}")
print(f"Shape of labels: {y.shape}")

# input_shape = (512, 512, 3)
base_model = MobileNetV2(weights='imagenet', include_top=False, input_shape=input_shape)

# Freeze base model layers
base_model.trainable = False

def create_model(input_shape):
    # model = Sequential()
    #
    # # Convolutional layers
    # model.add(Conv2D(32, (3, 3), activation='relu', input_shape=input_shape))
    # model.add(MaxPooling2D(pool_size=(2, 2)))
    #
    # model.add(Conv2D(64, (3, 3), activation='relu'))
    # model.add(MaxPooling2D(pool_size=(2, 2)))
    #
    # model.add(Conv2D(128, (3, 3), activation='relu'))
    # model.add(MaxPooling2D(pool_size=(2, 2)))
    #
    # # Flatten and fully connected layers
    # model.add(Flatten())
    # model.add(Dense(128, activation='relu'))
    #
    # # Output layer: 2 units (x and y coordinates)
    # model.add(Dense(2, activation='linear'))  # For regression, we use a linear output
    #
    # # Compile the model
    # model.compile(optimizer='adam', loss='mean_squared_error', metrics=['mae'])
    #
    # return model
    model = models.Sequential([
        # First convolutional block
        layers.Conv2D(32, (3, 3), activation='relu', input_shape=input_shape),
        layers.MaxPooling2D((2, 2)),

        # Second convolutional block
        layers.Conv2D(64, (3, 3), activation='relu'),
        layers.MaxPooling2D((2, 2)),

        # dropout layer
        layers.Dropout(0.2),

        # Third convolutional block
        layers.Conv2D(128, (3, 3), activation='relu'),
        layers.MaxPooling2D((2, 2)),

        layers.Dropout(0.2),

        # Fourth convolutional block
        layers.Conv2D(128, (3, 3), activation='relu'),
        layers.MaxPooling2D((2, 2)),

        # Flatten the output
        layers.Flatten(),

        # Fully connected layers
        layers.Dense(256, activation='relu'),
        layers.Dropout(0.2),
        layers.Dense(128, activation='relu'),

        # Output layer for x and y coordinates
        layers.Dense(2, activation='linear')  # 'linear' for regression
    ])

    fine_tuning = True
    if fine_tuning:
        # Unfreeze some layers for fine-tuning
        base_model.trainable = True
        for layer in base_model.layers[:-4]:  # Only fine-tune the last few layers
            layer.trainable = False

    model = models.Sequential([
        Input(input_shape, name='test_in_input'),
        # Base model with pre-trained weights
        base_model,

        layers.Flatten(),  # Flatten the output of VGG16

        # Custom dense layers for regression
        layers.Dense(512, activation='relu'),
        layers.Dropout(0.2),
        layers.Dense(256, activation='relu'),
        layers.Dense(128, activation='relu'),

        # Output layer for x and y coordinates
        layers.Dense(2, activation='linear', name='test_out')  # 'linear' activation for regression
    ])

    model.compile(optimizer='adam', loss='mse', metrics=['mae'])
    # model.name = "ball_detector"
    return model


# Create the model
# input_shape = (512, 512, 3)  # Example image shape
model = create_model(input_shape)

# model = load_model('detect_player.keras')
model.name = "player_detector"

fine_tuning = False
if fine_tuning:
    # Unfreeze some layers for fine-tuning
    base_model.trainable = True
    # for layer in model.layers:
    #     layer.trainable = True
    # for layer in base_model.layers[:-4]:  # Only fine-tune the last few layers
    #     layer.trainable = False


# X_test, y_test = load_data_from_filenames('../java_ma/fiducial_test_sets/')


callbacks = [
    ActivationsVisualizationCallback(
        validation_data=(x_test, y_test),
        layers_name=["activation_1"],
        output_dir="./output",
    ),
]
model.summary()

# Train the model
history = model.fit(x, y, epochs=50, batch_size=4, validation_split=0.2)

# Save the trained model
model.save('detect_player.keras')
tf.saved_model.save(model, "detect_player")
