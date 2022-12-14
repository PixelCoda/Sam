from PIL import Image
# from imutils import paths
import face_recognition
from face_recognition.face_recognition_cli import image_files_in_folder
import pickle
import argparse
import os
import math
from sklearn import neighbors
import random
import string
from shutil import copyfile
import ntpath
import json

def VGG16_predict(img_path, VGG16):
    '''
    Use pre-trained VGG-16 model to obtain index corresponding to 
    predicted ImageNet class for image at specified path
    '''
    ## Return the *index* of the predicted class for that image
    img = Image.open(img_path)
    min_img_size = 224
    transform_pipeline = transforms.Compose([transforms.Resize((min_img_size, min_img_size)),
                                             transforms.ToTensor(),
                                             transforms.Normalize(mean=[0.485, 0.456, 0.406],
                                                                  std=[0.229, 0.224, 0.225])])
    img = transform_pipeline(img)
    img = img.unsqueeze(0)
    # if use_cuda:
    #     img = img.to('cuda')
    prediction = VGG16(img)
    prediction = prediction.argmax()
    return prediction # predicted class index

ALLOWED_EXTENSIONS = {'png', 'jpg', 'jpeg'}


def train(train_dir, model_save_path=None, n_neighbors=None, knn_algo='ball_tree', verbose=False):
    """
    Trains a k-nearest neighbors classifier for face recognition.
    :param train_dir: directory that contains a sub-directory for each known person, with its name.
     (View in source code to see train_dir example tree structure)
     Structure:
        <train_dir>/
        ├── <person1>/
        │   ├── <somename1>.jpeg
        │   ├── <somename2>.jpeg
        │   ├── ...
        ├── <person2>/
        │   ├── <somename1>.jpeg
        │   └── <somename2>.jpeg
        └── ...
    :param model_save_path: (optional) path to save model on disk
    :param n_neighbors: (optional) number of neighbors to weigh in classification. Chosen automatically if not specified
    :param knn_algo: (optional) underlying data structure to support knn.default is ball_tree
    :param verbose: verbosity of training
    :return: returns knn classifier that was trained on the given data.
    """
    X = []
    y = []

    # Loop through each person in the training set
    for class_dir in os.listdir(train_dir):
        if not os.path.isdir(os.path.join(train_dir, class_dir)):
            continue

        # Loop through each training image for the current person
        for img_path in image_files_in_folder(os.path.join(train_dir, class_dir)):
            image = face_recognition.load_image_file(img_path)
            face_bounding_boxes = face_recognition.face_locations(image)

            if len(face_bounding_boxes) != 1:
                # If there are no people (or too many people) in a training image, skip the image.
                if verbose:
                    print("Image {} not suitable for training: {}".format(img_path, "Didn't find a face" if len(face_bounding_boxes) < 1 else "Found more than one face"))
            else:
                # Add face encoding for current image to the training set
                X.append(face_recognition.face_encodings(image, known_face_locations=face_bounding_boxes)[0])
                y.append(class_dir)

    # Determine how many neighbors to use for weighting in the KNN classifier
    if n_neighbors is None:
        n_neighbors = int(round(math.sqrt(len(X))))
        if verbose:
            print("Chose n_neighbors automatically:", n_neighbors)

    # Create and train the KNN classifier
    knn_clf = neighbors.KNeighborsClassifier(n_neighbors=n_neighbors, algorithm=knn_algo, weights='distance')
    knn_clf.fit(X, y)

    # Save the trained KNN classifier
    if model_save_path is not None:
        with open(model_save_path, 'wb') as f:
            pickle.dump(knn_clf, f)

    return knn_clf


def predict(X_img_path, knn_clf=None, model_path=None, distance_threshold=0.6):
    """
    Recognizes faces in given image using a trained KNN classifier
    :param X_img_path: path to image to be recognized
    :param knn_clf: (optional) a knn classifier object. if not specified, model_save_path must be specified.
    :param model_path: (optional) path to a pickled knn classifier. if not specified, model_save_path must be knn_clf.
    :param distance_threshold: (optional) distance threshold for face classification. the larger it is, the more chance
           of mis-classifying an unknown person as a known one.
    :return: a list of names and face locations for the recognized faces in the image: [(name, bounding box), ...].
        For faces of unrecognized persons, the name 'unknown' will be returned.
    """
    if not os.path.isfile(X_img_path) or os.path.splitext(X_img_path)[1][1:] not in ALLOWED_EXTENSIONS:
        raise Exception("Invalid image path: {}".format(X_img_path))

    if knn_clf is None and model_path is None:
        raise Exception("Must supply knn classifier either thourgh knn_clf or model_path")

    # Load a trained KNN model (if one was passed in)
    if knn_clf is None:
        with open(model_path, 'rb') as f:
            knn_clf = pickle.load(f)

    # Load image file and find face locations
    X_img = face_recognition.load_image_file(X_img_path)
    X_face_locations = face_recognition.face_locations(X_img)

    # If no faces are found in the image, return an empty result.
    if len(X_face_locations) == 0:
        return []

    # Find encodings for faces in the test iamge
    faces_encodings = face_recognition.face_encodings(X_img, known_face_locations=X_face_locations)

    # Use the KNN model to find the best matches for the test face
    closest_distances = knn_clf.kneighbors(faces_encodings, n_neighbors=1)
    are_matches = [closest_distances[0][i][0] <= distance_threshold for i in range(len(X_face_locations))]

    # Predict classes and remove classifications that aren't within the threshold
    return [(pred, loc) if rec else ("unknown", loc) for pred, loc, rec in zip(knn_clf.predict(faces_encodings), X_face_locations, are_matches)]


# Initiate the parser
parser = argparse.ArgumentParser()
parser.add_argument("--oid", "-oid", help="oid to use if uknown")
parser.add_argument("--image", "-img", help="image path to process")
parser.add_argument("--train", "-t", help="just train")

# Read arguments from the command line
args = parser.parse_args()

if args.image:
    # Find all people in the image using a trained classifier model
    # Note: You can pass in either a classifier file name or a classifier model instance
    predictions = predict(args.image, model_path="/opt/sam/scripts/who.io/trained_knn_model.clf")

    file_name = ntpath.basename(args.image);

    oid = args.oid

    # Print results on the console
    for name, (top, right, bottom, left) in predictions:
        letters = string.digits
        randomness = ''.join(random.choice(letters) for i in range(10));
        directory = os.path.join("/opt/sam/scripts/who.io/dataset/", oid)


        if name == "unknown":

            name = oid

            os.mkdir(directory)
            copyfile(args.image, "/opt/sam/scripts/who.io/dataset/" + oid + "/" + file_name)
            
            # Retrain
            classifier = train("/opt/sam/scripts/who.io/dataset/", model_save_path="/opt/sam/scripts/who.io/trained_knn_model.clf", n_neighbors=2)
        else:
           directory = os.path.join("/opt/sam/scripts/who.io/dataset/" + name + "/")
           if os.path.isdir(directory) == False:
               os.mkdir(directory)
           copyfile(args.image, directory + file_name)


        reply = {
            "id": name,
            "directory": directory,
            "top": top,
            "right": right,
            "bottom": bottom,
            "left": left
        }

        y = json.dumps(reply)

        print(y)

if args.train:
    classifier = train("/opt/sam/scripts/who.io/dataset/", model_save_path="/opt/sam/scripts/who.io/trained_knn_model.clf", n_neighbors=2)
    