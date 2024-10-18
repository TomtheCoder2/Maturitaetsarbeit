import ast
import numpy as np
from numpy import save


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

def load_training_data_x(file_path):
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

        # Convert lists into numpy arrays
        X = np.array(X, dtype=np.float32)

    return X

def load_training_data_y(file_path):
    """
    Load training data from a file.
    The file is expected to have two lines:
    - First line: tasks (X values) as a 2D array.
    - Second line: targets (Y values) as a 2D array.
    """
    with open(file_path, 'r') as f:
        lines = f.readlines()

        # Parse the string data into Python lists
        Y = ast.literal_eval(lines[1].strip())

        # Convert lists into numpy arrays
        Y = np.array(Y, dtype=np.float32)

    return Y

file_path = '../java_ma/training_set.txt'
X = load_training_data_x(file_path)

save('x.npy', X)

Y = load_training_data_y(file_path)
save('y.npy', Y)