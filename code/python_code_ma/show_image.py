import sys
import cv2
import tkinter as tk
from PIL import Image, ImageTk

root = tk.Tk()
root.title("Image Annotation Tool")

# Read first arg and then read the img with given path with cv2 and show it
path = sys.argv[1]
input_x = 0
if len(sys.argv) > 2:
    input_x = int(sys.argv[2])
    print(input_x)

input_y = 0
if len(sys.argv) > 3:
    input_y = int(sys.argv[3])
    print(input_y)

fac = 1
current_image = Image.open(path)
file_name = path.split("/")[-1]
file_name = file_name.split("\\")[-1]
print(file_name)
point_x, point_y = 0, 0
if len(file_name.split('.')) > 1:
    if len(file_name.split('_')) > 1:
        try:
            x_str, y_str = file_name.split('.')[0].split('_')[0:2]
            point_x, point_y = int(x_str) * fac, int(y_str) * fac
        except ValueError:
            print(f"Skipping invalid file name: {file_name}")

canvas = None
point_set = False

width = current_image.width
height = current_image.height
print(width, height)
display_image = current_image.resize((width * fac, height * fac))
display_photo = ImageTk.PhotoImage(display_image)

# Update or create the canvas to show the new image
if canvas:
    canvas.delete("all")
else:
    canvas = tk.Canvas(root, width=display_image.width, height=display_image.height)
    canvas.pack()

# Display image on canvas
canvas.image = display_photo
canvas.create_image(0, 0, anchor=tk.NW, image=display_photo)

# Create a text field to show the coordinates
coord_label = tk.Label(root, text="Coordinates: (0, 0)")
coord_label.pack()

def on_mouse_click(event):
    global point_x, point_y, point_set
    point_x, point_y = event.x, event.y
    point_set = True
    draw_point()
    coord_label.config(text=f"Coordinates: ({point_x}, {point_y})")

def draw_point():
    # Clear previous point and draw a new one
    canvas.delete("point")
    radius = 5
    canvas.create_oval(point_x - radius, point_y - radius, point_x + radius, point_y + radius, fill='red', tags="point")

canvas.bind("<Button-1>", on_mouse_click)

# Clear previous point and draw a new one
canvas.delete("point")
radius = 5
canvas.create_oval(point_x - radius, point_y - radius, point_x + radius, point_y + radius, fill='red', tags="point")
canvas.create_oval(input_x - radius, input_y - radius, input_x + radius, input_y + radius, fill='red', tags="point")

root.mainloop()