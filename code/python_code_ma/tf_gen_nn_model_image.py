from tensorflow.keras.applications.vgg16 import preprocess_input
from tensorflow.keras.preprocessing.image import load_img
from tensorflow.keras.models import load_model

from tensorflow.keras import preprocessing
from tensorflow.keras import backend as K
from tensorflow.keras import models
from keras.utils import plot_model

import tensorflow as tf
import numpy as np

image_size = 154

# Load pre-trained Keras model and the image to classify
model_name = "fiducial_classifier_model.keras"
model = load_model(model_name)

plot_model(
    model,
    to_file= "./model_images/"+ model_name.split(".")[0] + ".png",
    show_shapes=True,
    show_dtype=True,
    show_layer_names=True,
    rankdir="TB",
    expand_nested=True,
    dpi=200,
    show_layer_activations=True,
    show_trainable=True,
)
