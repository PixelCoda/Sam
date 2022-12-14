"""
Title: Speaker Recognition
Author: [Fadi Badine](https://twitter.com/fadibadine)
Date created: 14/06/2020
Last modified: 03/07/2020
Description: Classify speakers using Fast Fourier Transform (FFT) and a 1D Convnet.
"""
"""
## Introduction

This example demonstrates how to create a model to classify speakers from the
frequency domain representation of speech recordings, obtained via Fast Fourier
Transform (FFT).

It shows the following:

- How to use `tf.data` to load, preprocess and feed audio streams into a model
- How to create a 1D convolutional network with residual
connections for audio classification.

Our process:

- We prepare a dataset of speech samples from different speakers, with the speaker as label.
- We add background noise to these samples to augment our data.
- We take the FFT of these samples.
- We train a 1D convnet to predict the correct speaker given a noisy FFT speech sample.

Note:

- This example should be run with TensorFlow 2.3 or higher, or `tf-nightly`.
- The noise samples in the dataset need to be resampled to a sampling rate of 16000 Hz
before using the code in this example. In order to do this, you will need to have
installed `ffmpg`.
"""

"""
## Setup
"""

import sys
import os
import shutil
import numpy as np
import pickle
import tensorflow as tf
from tensorflow import keras
from tensorflow.python.ops.numpy_ops import np_config
np_config.enable_numpy_behavior()

# from pathlib import Path
# from IPython.display import display, Audio

# Get the data from https://www.kaggle.com/kongaevans/speaker-recognition-dataset/download
# and save it to the 'Downloads' folder in your HOME directory
DATASET_ROOT = os.path.join("/opt/sam/scripts/sprec/")

# The folders in which we will put the audio samples and the noise samples
AUDIO_SUBFOLDER = "audio"
NOISE_SUBFOLDER = "noise"

DATASET_AUDIO_PATH = os.path.join(DATASET_ROOT, AUDIO_SUBFOLDER)
DATASET_NOISE_PATH = os.path.join(DATASET_ROOT, NOISE_SUBFOLDER)

# Percentage of samples to use for validation
VALID_SPLIT = 0.1

# Seed to use when shuffling the dataset and the noise
SHUFFLE_SEED = 43

# The sampling rate to use.
# This is the one used in all of the audio samples.
# We will resample all of the noise to this sampling rate.
# This will also be the output size of the audio wave samples
# (since all samples are of 1 second long)
SAMPLING_RATE = 16000

# The factor to multiply the noise with according to:
#   noisy_sample = sample + noise * prop * scale
#      where prop = sample_amplitude / noise_amplitude
SCALE = 0.5

BATCH_SIZE = 128
EPOCHS = 100






# Split noise into chunks of 16,000 steps each
def load_noise_sample(path):
    sample, sampling_rate = tf.audio.decode_wav(
        tf.io.read_file(path), desired_channels=1
    )
    if sampling_rate == SAMPLING_RATE:
        # Number of slices of 16000 each that can be generated from the noise sample
        slices = int(sample.shape[0] / SAMPLING_RATE)
        sample = tf.split(sample[: slices * SAMPLING_RATE], slices)
        return sample
    else:
        print("Sampling rate for {} is incorrect. Ignoring it".format(path), file=sys.stdout)
        return None



def paths_and_labels_to_dataset(audio_paths, labels):
    """Constructs a dataset of audios and labels."""
    path_ds = tf.data.Dataset.from_tensor_slices(audio_paths)
    audio_ds = path_ds.map(
        lambda x: path_to_audio(x), num_parallel_calls=tf.data.AUTOTUNE
    )
    label_ds = tf.data.Dataset.from_tensor_slices(labels)
    return tf.data.Dataset.zip((audio_ds, label_ds))


def get_label(file_path):
    parts = tf.strings.split(
        input=file_path,
        sep=os.path.sep)
    # Note: You'll use indexing here instead of tuple unpacking to enable this
    # to work in a TensorFlow graph.
    return parts[-2]

def get_waveform_and_label(file_path):
    label = get_label(file_path)
    audio_binary = tf.io.read_file(file_path)
    waveform =  decode_audio(audio_binary)
    return waveform, label

def decode_audio(audio_binary):
    # Decode WAV-encoded audio files to `float32` tensors, normalized
    # to the [-1.0, 1.0] range. Return `float32` audio and a sample rate.
    audio, _ = tf.audio.decode_wav(contents=audio_binary)
    # Since all the data is single channel (mono), drop the `channels`
    # axis from the array.
    return tf.squeeze(audio, axis=-1)

def path_to_audio(path):
    """Reads and decodes an audio file."""
    audio_binary = tf.io.read_file(path)
    audio, _ = tf.audio.decode_wav(audio_binary, 1, SAMPLING_RATE)
    print("loaded_audio:", file=sys.stdout)
    print(audio, file=sys.stdout)
    return audio


def audio_to_fft(audio):
    shape = audio.shape
    # Since tf.signal.fft applies FFT on the innermost dimension,
    # we need to squeeze the dimensions and then expand them again
    # after FFT
    audio = tf.squeeze(audio, axis=-1)
    fft = tf.signal.fft(
        tf.cast(tf.complex(real=audio, imag=tf.zeros_like(audio)), tf.complex64)
    )
    fft = tf.expand_dims(fft, axis=-1)

    # audio = audio.reshape((1, audio.shape[0]))      

    # print(fft)
    print("audio_shape:", file=sys.stdout)
    print(audio.shape, file=sys.stdout)

    # Return the absolute value of the first half of the FFT
    # which represents the positive frequencies
    return tf.math.abs(fft[:, : (audio.shape[1] // 2), :])


"""
## Model Definition
"""


def residual_block(x, filters, conv_num=3, activation="relu"):
    # Shortcut
    s = keras.layers.Conv1D(filters, 1, padding="same")(x)
    for i in range(conv_num - 1):
        x = keras.layers.Conv1D(filters, 3, padding="same")(x)
        x = keras.layers.Activation(activation)(x)
    x = keras.layers.Conv1D(filters, 3, padding="same")(x)
    x = keras.layers.Add()([x, s])
    x = keras.layers.Activation(activation)(x)
    return keras.layers.MaxPool1D(pool_size=2, strides=2)(x)


def build_model(input_shape, num_classes):
    inputs = keras.layers.Input(shape=input_shape, name="input")

    x = residual_block(inputs, 16, 2)
    x = residual_block(x, 32, 2)
    x = residual_block(x, 64, 3)
    x = residual_block(x, 128, 3)
    x = residual_block(x, 128, 3)

    x = keras.layers.AveragePooling1D(pool_size=3, strides=3)(x)
    x = keras.layers.Flatten()(x)
    x = keras.layers.Dense(256, activation="relu")(x)
    x = keras.layers.Dense(128, activation="relu")(x)

    outputs = keras.layers.Dense(num_classes, activation="softmax", name="output")(x)

    return keras.models.Model(inputs=inputs, outputs=outputs)



class_names = pickle.loads(open('/opt/sam/scripts/sprec/labels.pickle', "rb").read())


model = keras.models.load_model('/opt/sam/scripts/sprec/model.h5')


model.summary()

# Compile the model using Adam's default learning rate
model.compile(
    optimizer="Adam", loss="sparse_categorical_crossentropy", metrics=["accuracy"]
)


paths = ["/opt/sam/scripts/sprec/test.wav"]
labels = ["Unknown"]

train_ds = paths_and_labels_to_dataset(paths, labels)
train_ds = train_ds.prefetch(tf.data.AUTOTUNE)
train_ds = train_ds.shuffle(buffer_size=BATCH_SIZE * 8, seed=SHUFFLE_SEED).batch(
    BATCH_SIZE
)


valid_ds = paths_and_labels_to_dataset(paths, labels)
valid_ds = valid_ds.shuffle(buffer_size=32 * 8, seed=SHUFFLE_SEED).batch(32)
valid_ds = valid_ds.map(
    lambda x, y: (audio_to_fft(x), y), num_parallel_calls=tf.data.AUTOTUNE
)
valid_ds = valid_ds.prefetch(tf.data.AUTOTUNE)


# audio_to_check = path_to_audio("/home/kal/sprec/1.wav")

# # path_ds = tf.data.Dataset.from_tensor_slices(audio_paths)
# # audio_ds = path_ds.map(
# #     lambda x: path_to_audio("/home/kal/sprec/test.wav"), num_parallel_calls=tf.data.AUTOTUNE
# # )

# ffts = audio_to_fft(audio_to_check)
# # Predict
# y_pred = model.predict(ffts)

# print(model.evaluate(valid_ds))


for audios, labels in train_ds.take(1):
    # Get the signal FFT
    ffts = audio_to_fft(audios)
    
    # Predict
    y_pred = model.predict(ffts)
    for result in y_pred:
        print(str(result), file=sys.stdout)


    confidence = y_pred[0][y_pred.argmax(axis=1)][0] * 100
    

    result = class_names[y_pred.argmax(axis=1)[0]]

    print(":::::" + result + ":::::" + str(confidence) + ":::::", file=sys.stdout)
    # print("Confidence: " + str(confidence))

    

    
    

