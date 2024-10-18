import tensorflow as tf
import numpy as np
from PIL import Image

np.set_printoptions(suppress=True, precision=1)

def load_and_predict(image_path, model_path='trained_model.keras'):
    # Load the trained model
    model = tf.keras.models.load_model(model_path)

    # Load the image
    img = Image.open(image_path)
    # img = img.resize((77, 71))  # Resize the image to 77x71 as per your input shape
    img_array = np.array(img)  # Convert image to a numpy array (no division by 255)
    print(img_array.shape)

    # Flatten the image array to a length of 16401 (77 * 71 * 3)
    img_array = img_array.flatten()
    img_array = img_array.reshape(1, -1)  # Add batch dimension (1, 16401)
    print(img_array.shape)
    print(img_array)

    # Predict using the model
    prediction = model.predict(img_array)

    return prediction


def predict(index, model_path='trained_model.keras'):
    model = tf.keras.models.load_model(model_path)
    X = np.load("../java_ma/training_tasks.npy")
    img_array = X[index]
    print(img_array.shape)
    print(img_array)
    Y = np.load("../java_ma/training_targets.npy")
    target = Y[index]

    prediction = model.predict(img_array.reshape(1, -1))
    print("prediction: " + str(prediction))
    print("target: " + str(target))
    error = prediction[0] - target
    print(error)
    for i in error:
        if abs(i) > 0.1:
            print(f'{i:.2f}', end="")
        else:
            print("  ", end="")
        print("|", end="")
    print()


# Example usage:
predict(0)
# predict(1)
# predict(2)
prediction = load_and_predict('../java_ma/training_set/img_0.png')
# now give the 3 largest number's index
# print(np.argsort(prediction[0])[-3:])
print(prediction[0])
# the target set was made like this:
# target.set(fiducial - 1, 1d);
# target.set(4 + midX, 1d);
# target.set(4 + img.getWidth() + midY, 1d);
# targets.add(target);
# so the first 3 indexes are the fiducial, midX and midY
# Image Dimensions: 77 71
# Extract fiducial
# target = prediction[0]
# fiducial = np.argmax(target[:4]) + 1
#
# # Extract midX
# midX = np.argmax(target[4:4 + 77])
#
# # Extract midY
# midY = np.argmax(target[4 + 77:])
#
# print(f"Fiducial: {fiducial}")
# print(f"midX: {midX - 3}")
# print(f"midY: {midY - 3 - 77}")
