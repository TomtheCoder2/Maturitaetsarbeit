import numpy as np
import tensorflow as tf
from tensorflow.keras.preprocessing import image
from tensorflow.keras.preprocessing.image import img_to_array
import matplotlib.pyplot as plt
from tensorflow.keras import backend as K
import cv2
import os
print(tf.__version__)



def load_img(image_path, target_size=(154, 143)):
    img = image.load_img(image_path, target_size=target_size)
    return img


def visualize_intermediate_layers(image_path, model, layer_names, img_size=(154, 143)):
    img = load_img(image_path, target_size=img_size)
    img_array = img_to_array(img) / 255.0
    img_array = np.expand_dims(img_array, axis=0)

    # Create a model that will output the intermediate layers
    layer_outputs = [model.get_layer(name).output for name in layer_names]
    activation_model = tf.keras.models.Model(inputs=model.inputs, outputs=layer_outputs)

    # Get feature maps
    activations = activation_model.predict(img_array)

    # Plotting the feature maps
    for layer_name, activation in zip(layer_names, activations):
        n_features = activation.shape[-1]  # Number of features
        size = activation.shape[1]  # Size of each feature map

        # Plot feature maps
        plt.figure(figsize=(15, 15))
        for i in range(n_features):
            plt.subplot(n_features // 8 + 1, 8, i + 1)  # 8 columns
            plt.imshow(activation[0, :, :, i], cmap='viridis')
            plt.axis('off')
            plt.title(layer_name + f'_{i}')
        # plt.show()
        # save plt to file
        plt.savefig(f'./plot/{layer_name}.png')


model = tf.keras.models.load_model('fiducial_classifier_model.keras')
print("input shape: ", model.input_shape)
print("output shape: ", model.output_shape)
model.summary()
for layer in model.layers:
    print(layer.name)

# Example usage:
image_path = '0_67_53.png'
layer_names = ['conv2d', 'conv2d_1', 'conv2d_2']  # Adjust based on your model's layers
visualize_intermediate_layers(image_path, model, layer_names)


def load_img(image_path, target_size=(154, 143)):
    img = image.load_img(image_path, target_size=target_size)
    img_array = img_to_array(img) / 255.0
    return np.expand_dims(img_array, axis=0)  # Expand dims here


def grad_cam(image_path, model, layer_name, img_size=(154, 143)):
    img_array = load_img(image_path, target_size=img_size)

    # Use GradientTape to compute gradients
    with tf.GradientTape() as tape:
        preds = model(img_array)
        top_pred_index = tf.argmax(preds[0])
        class_channel = preds[0][top_pred_index]

        # Get the output of the specified layer
        layer_output = model.get_layer(layer_name).output

        # Check the shape of the layer output
        print(f"Layer output shape: {layer_output.shape}")  # Debugging output

    # Compute gradients of the class output with respect to the layer output
    grads = tape.gradient(class_channel, layer_output)

    # Check the shape of grads
    print(f"Shape of grads: {grads.shape}")  # Debugging output

    # Check if grads are None
    if grads is None:
        print("Gradients are None. Check the model configuration.")
        return

    # Pool the gradients across all the channels
    pooled_grads = K.mean(grads, axis=(0, 1))

    # Compute the heatmap
    heatmap = layer_output[0] @ pooled_grads[..., tf.newaxis]
    heatmap = K.relu(heatmap)

    # Normalize the heatmap
    heatmap /= K.max(heatmap)

    # Resize the heatmap to the original image size
    heatmap = cv2.resize(heatmap.numpy(), (img_size[1], img_size[0]))
    heatmap = np.uint8(255 * heatmap)

    # Load original image
    original_image = cv2.imread(image_path)

    # Apply colormap to the heatmap
    heatmap = cv2.applyColorMap(heatmap, cv2.COLORMAP_JET)
    heatmap = cv2.addWeighted(original_image, 0.6, heatmap, 0.4, 0)

    plt.imshow(cv2.cvtColor(heatmap, cv2.COLOR_BGR2RGB))
    plt.axis('off')
    plt.title('Grad-CAM')
    plt.savefig(f'./plot/{layer_name}_grad_cam.png')


# for layer in model.layers:
#     print(layer.name + ":")
#     try:
#         grad_cam(image_path, model, layer.name)
#         print(f"Layer {layer.name} completed")
#     except:
#         print(f"Layer {layer.name} failed")

# Example usage:
layer_name = 'conv2d'  # The last convolutional layer you want to visualize
grad_cam(image_path, model, layer_name)
