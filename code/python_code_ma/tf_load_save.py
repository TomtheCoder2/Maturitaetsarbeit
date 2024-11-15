from tensorflow.keras.models import load_model
import tensorflow as tf

model = load_model('detect_player.keras')
tf.saved_model.save(model, "detect_player")

# model = load_model('detect_player')

