import tensorflow as tf
from tensorflow.keras import layers, models
from tensorflow.keras.preprocessing import image_dataset_from_directory
from tensorflow.keras.models import load_model

# Set the directory where the images are stored (organized by folders)
# dataset_directory = "../java_ma/fiducial_training_sets/"  # Make sure it's structured as: fiducial_1, fiducial_2, fiducial_3, fiducial_4
dataset_directory = "../java_ma/real_fiducial_sets/"

# Load the dataset, automatically splitting it into training and validation sets
batch_size = 64
# 154 143
img_height = 154
img_width = 143

train_dataset = image_dataset_from_directory(
    dataset_directory,
    # validation_split=0.2,
    # subset="training",
    seed=123,
    image_size=(img_height, img_width),
    batch_size=batch_size
)

val_dataset = image_dataset_from_directory(
    dataset_directory,
    # validation_split=0.2,
    # subset="validation",
    seed=123,
    image_size=(img_height, img_width),
    batch_size=batch_size
)

data_augmentation = tf.keras.Sequential([
    layers.RandomFlip("horizontal_and_vertical"),
    layers.RandomRotation(0.2),
])


# Create the CNN model
model = models.Sequential([
    data_augmentation,
    layers.Rescaling(1./255, input_shape=(img_height, img_width, 3)),  # Normalize pixel values
    layers.Conv2D(32, (3, 3), activation='relu'),
    layers.MaxPooling2D((2, 2)),
    layers.Conv2D(64, (3, 3), activation='relu'),
    layers.MaxPooling2D((2, 2)),
    layers.Conv2D(128, (3, 3), activation='relu'),
    layers.MaxPooling2D((2, 2)),
    layers.Flatten(),
    layers.Dense(128, activation='relu'),
    layers.Dense(4, activation='softmax')  # 4 classes for the 4 fiducials
])

model = load_model('fiducial_classifier_model.keras')

# Compile the model
model.compile(optimizer='adam',
              loss='sparse_categorical_crossentropy',
              metrics=['accuracy'])

# Train the model
epochs = 50  # You can adjust based on your dataset size
history = model.fit(
    train_dataset,
    validation_data=val_dataset,
    epochs=epochs
)

# Save the model for future use
model.save('fiducial_classifier_model.keras')

# Evaluate the model
test_loss, test_acc = model.evaluate(val_dataset, verbose=2)
print(f"Validation accuracy: {test_acc:.4f}")
