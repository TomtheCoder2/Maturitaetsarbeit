import numpy as np
import tensorflow as tf
import ast
from tensorflow import keras
from tensorflow.keras.callbacks import TensorBoard, Callback
import datetime
# from tensorflow.keras.mixed_precision import experimental as mixed_precision
# policy = mixed_precision.Policy('mixed_float16')
# mixed_precision.set_policy(policy)
from numpy import save
import os
from tensorflow.keras.callbacks import EarlyStopping


def load_training_data(file_path):
    """
    Load training data from a file.
    The file is expected to have two lines:
    - First line: tasks (X values) as a 2D array.
    - Second line: targets (Y values) as a 2D array.
    """
    with open(file_path, 'r') as f:
        lines = f.readlines()

        # Parse the string data into Python lists
        X = ast.literal_eval(lines[0].strip())  # Convert the string back into a list
        Y = ast.literal_eval(lines[1].strip())

        # Convert lists into numpy arrays
        X = np.array(X, dtype=np.float32)
        Y = np.array(Y, dtype=np.float32)

    return X, Y


class PrintEpochProgress(Callback):
    def __init__(self, print_every):
        super(PrintEpochProgress, self).__init__()
        self.print_every = print_every

    def on_epoch_end(self, epoch, logs=None):
        if (epoch + 1) % self.print_every == 0:
            print(f'Epoch {epoch + 1}: logs={logs}')


def build_model(input_shape, output_shape):
    model = tf.keras.models.Sequential([
        tf.keras.layers.InputLayer(input_shape=input_shape),
        tf.keras.layers.BatchNormalization(),
        tf.keras.layers.Dense(2048, activation='relu', kernel_regularizer=tf.keras.regularizers.l2(0.05)),
        tf.keras.layers.Dropout(0.3),
        tf.keras.layers.Dense(512, activation='relu', kernel_regularizer=tf.keras.regularizers.l2(0.05)),
        tf.keras.layers.Dropout(0.3),
        # tf.keras.layers.Dense(512, activation='relu', kernel_regularizer=tf.keras.regularizers.l2(0.01)),
        # tf.keras.layers.Dense(300, activation='relu', kernel_regularizer=tf.keras.regularizers.l2(0.01)),
        tf.keras.layers.Dense(8, activation='relu', kernel_regularizer=tf.keras.regularizers.l2(0.05)),
        # tf.keras.layers.Dropout(0.5),
        tf.keras.layers.Dense(output_shape, activation='linear')
    ])

    return model


def train_model(X, Y, model=None, print_every=50):
    log_dir = "logs/fit/" + datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
    tensorboard_callback = TensorBoard(log_dir=log_dir, histogram_freq=1)
    early_stopping = EarlyStopping(monitor='val_loss', patience=50, restore_best_weights=True)

    print("Number of samples in X:", X.shape[0])
    print("Number of samples in Y:", Y.shape[0])
    print("Number of inputs: ", X.shape[1])
    print("Number of outputs: ", Y.shape[1])
    input_shape = (X.shape[1],)
    output_shape = Y.shape[1]

    # check that the first layer contains the correct number of input features
    if model is None:
        print("Building a new model...")
        model = build_model(input_shape, output_shape)
    # else:
    optimizer = tf.keras.optimizers.Adam(learning_rate=0.001)
    model.compile(optimizer="adam",
                  loss='mean_squared_error',
                  metrics=['mae'])

    # Create an instance of the custom callback
    progress_callback = PrintEpochProgress(print_every)

    with tf.device('/GPU:0'):
        model.fit(X, Y, epochs=1000, batch_size=128, validation_split=0.2, callbacks=[progress_callback], verbose=0)

    return model


def test_model(model, X, Y):
    """
    Evaluate the model on the test data.
    """
    loss, accuracy = model.evaluate(X, Y)
    print(f"Test Loss: {loss}")
    print(f"Test Accuracy: {accuracy}")


if __name__ == "__main__":
    physical_devices = tf.config.list_physical_devices('GPU')
    if len(physical_devices) > 0:
        print(f"Using GPU: {physical_devices[0].name}")
    else:
        print("No GPU found, running on CPU.")

    tf.keras.backend.clear_session()

    try:
        model = tf.keras.models.load_model("trained_model.keras")
        print("Model loaded from trained_model.keras")
    except Exception as e:
        print("Model not found, training a new model...")
        print(e)
        model = None

    # Load training data
    X = np.load('../java_ma/training_tasks.npy')
    Y = np.load('../java_ma/training_targets.npy')

    # Train the model
    model = train_model(X, Y, model)

    # Load test data
    # file_path = '../java_ma/test_set.txt'
    # X_test, Y_test = load_training_data(file_path)
    X_test = np.load('../java_ma/test_tasks.npy')
    Y_test = np.load('../java_ma/test_targets.npy')

    # Test the model
    test_model(model, X_test, Y_test)

    # Save the trained model
    model.save("trained_model.keras")
    print("Model trained and saved as trained_model.keras")
