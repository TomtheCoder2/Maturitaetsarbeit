import sys

import cv2

# read first arg and then read the img with given path with cv2 and show it
img = cv2.imread(sys.argv[1])
cv2.imshow('Image', img)
cv2.waitKey(0)
cv2.destroyAllWindows()
