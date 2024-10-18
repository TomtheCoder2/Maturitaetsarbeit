import tensorflow as tf
import numpy as np
import ast



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


# Load your trained model
model = tf.keras.models.load_model('trained_model.h5')

# file_path = '../java_ma/training_set.txt'  # Path to your test data file
# X, Y = load_training_data(file_path)
X = np.load('X.npy')
Y = np.load('Y.npy')

# Evaluate the model
loss, accuracy = model.evaluate(X, Y)

print(f"Test Loss: {loss}")
print(f"Test Accuracy: {accuracy}")
