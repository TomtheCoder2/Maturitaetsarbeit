import cv2
import numpy as np
import glob

# Camera matrix and distortion coefficients from calibration
camera_matrix = np.array([[458.11615111, 0., 367.97913048],
                          [0., 458.19122834, 272.04155943],
                          [0., 0., 1.]])
dist_coeffs = np.array([[-0.32281387, 0.17342487, -0.0012062, 0.004915, -0.07841724]])

# Load images
images = glob.glob('./calibration_pictures/*.png')

for filename in images:
    image = cv2.imread(filename)
    h, w = image.shape[:2]

    # Get the optimal new camera matrix with a scaling factor close to 1 (to reduce cropping)
    new_camera_matrix, roi = cv2.getOptimalNewCameraMatrix(camera_matrix, dist_coeffs, (w, h), 1, (w, h))

    # Undistort the image
    undistorted_image = cv2.undistort(image, camera_matrix, dist_coeffs, None, new_camera_matrix)

    # Preserve aspect ratio by calculating the correct crop
    x, y, roi_w, roi_h = roi
    aspect_ratio = w / h
    new_h = roi_h
    new_w = int(new_h * aspect_ratio)

    # Ensure new dimensions are within image bounds
    new_w = min(new_w, roi_w)
    new_h = int(new_w / aspect_ratio)

    # Crop the undistorted image to preserve the aspect ratio
    # undistorted_image = undistorted_image[y:y+new_h, x:x+new_w]

    # Save or display the undistorted image
    cv2.imshow('Undistorted Image', undistorted_image)
    cv2.waitKey(0)

    # Optionally, save the corrected images
    output_filename = filename.replace('calibration_pictures', 'undistorted_pictures')
    cv2.imwrite(output_filename, undistorted_image)

cv2.destroyAllWindows()
