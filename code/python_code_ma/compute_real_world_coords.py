import numpy as np
import cv2
import re


# Function to read the points from midpoints.txt
def read_midpoints(file_path):
    with open(file_path, 'r') as f:
        content = f.read()

    # Extract points using regex
    points = re.findall(r'[-+]?\d*\.\d+', content)

    # Convert points to float and assign them to corners
    top_left = (float(points[0]), float(points[1]))
    top_right = (float(points[2]), float(points[3]))
    bottom_left = (float(points[4]), float(points[5]))
    bottom_right = (float(points[6]), float(points[7]))
    print(np.array([top_left, top_right, bottom_left, bottom_right], dtype=np.float32))

    return np.array([top_left, top_right, bottom_left, bottom_right], dtype=np.float32)


# Path to the midpoints.txt file
file_path = './midpoints.txt'

# Read the pixel coordinates from the file
pixel_points = read_midpoints(file_path)

# Define the real-world coordinates of the points (in mm)
real_points = np.array([
    [78, 65],  # Top Left fiducial_2
    [1225 - 90, 65],  # Top Right fiducial_1
    [85, 723 - 87],  # Bottom Left fiducial_3
    [1225 - 72, 723 - 70]  # Bottom Right fiducial_4
], dtype=np.float32)

# Compute the homography matrix
homography_matrix, status = cv2.findHomography(pixel_points, real_points)

np.savetxt('homography_matrix.csv', homography_matrix, delimiter=',')


# Define the new point (361, 228) in pixel coordinates
new_pixel_point = np.array([[550, 286]], dtype=np.float32).reshape(-1, 1, 2)

# Apply the homography to the new point
real_world_point = cv2.perspectiveTransform(new_pixel_point, homography_matrix)

# Print the transformed real-world coordinates
print(f"Real-world coordinates of the point (361, 228): {real_world_point[0][0]}")
