import os
import tkinter as tk
from tkinter import filedialog
from PIL import Image, ImageTk
import json

# File to store the directories
config_file = 'config.json'

def save_config(directory, output_directory):
    with open(config_file, 'w') as f:
        json.dump({'directory': directory, 'output_directory': output_directory}, f)

def load_config():
    if os.path.exists(config_file):
        with open(config_file, 'r') as f:
            return json.load(f)
    return {'directory': '', 'output_directory': ''}

# Load the last used directories
config = load_config()
directory = config['directory']
output_directory = config['output_directory']

# Select directory with images
if not directory:
    directory = filedialog.askdirectory(title="Select Directory with Images")
else:
    directory = filedialog.askdirectory(initialdir=directory, title="Select Directory with Images")
print(f"Selected directory: {directory}, found {len(os.listdir(directory))} files")

# Select output directory
if not output_directory:
    output_directory = filedialog.askdirectory(title="Select Output Directory")
else:
    output_directory = filedialog.askdirectory(initialdir=output_directory, title="Select Output Directory")

# Save the selected directories
save_config(directory, output_directory)

def check_if_output_directory_contains_file(filename):
    for file in os.listdir(output_directory):
        if file.endswith(filename.lower()):
            return True
    return False

# Get all images without an underscore in their name
image_files = [f for f in os.listdir(directory) if
               f.lower().endswith(('.png', '.jpg', '.jpeg')) and not check_if_output_directory_contains_file(f)]

print(f"Found {len(image_files)} images")

# Shuffle image list for random order
# random.shuffle(image_files)

# Create the main tkinter window
root = tk.Tk()
root.title("Image Annotation Tool")

# Variables to store current point position
current_image = None
canvas = None
point_x, point_y = 0, 0
point_set = False
current_filename = ""
current_filename_number = ""
# Resize for viewing
fac = 3
last_pic = None

def update_image():
    global canvas, current_image, point_x, point_y, point_set, current_filename, current_filename_number, last_pic

    # If no images left, close the app
    if not image_files:
        root.quit()
        return

    # Load the next image
    current_filename = image_files.pop(0)
    # check if the output folder already contains a file that ends with the current_filename
    images_left = len(image_files)
    # print(f"Images left: {images_left}")
    # show in gui
    root.title(f"Image Annotation Tool - {current_filename} ({images_left} images left)")

    filepath = os.path.join(directory, current_filename)
    current_image = Image.open(filepath)
    if last_pic == current_image:
        save_coordinates()
        print("skipped")
        return
    last_pic = current_image

    # Extract coordinates from the filename
    try:
        x, y, current_filename_number = current_filename.split('_', 2)
        point_x, point_y = int(x), int(y)
    except ValueError:
        # Default to center if coordinates are not in the filename
        # point_x, point_y = current_image.width // 2, current_image.height // 2
        current_filename_number = current_filename

    display_image = current_image.resize((132 * fac, 190 * fac))
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

    point_set = False
    draw_point()

    # Bind mouse click to move the point
    canvas.bind("<Button-1>", on_mouse_click)

def on_mouse_click(event):
    global point_x, point_y, point_set
    point_x, point_y = event.x, event.y
    point_set = True
    draw_point()

def draw_point():
    # Clear previous point and draw a new one
    canvas.delete("point")
    radius = 5
    canvas.create_oval(point_x - radius, point_y - radius, point_x + radius, point_y + radius, fill='red', tags="point")

def save_coordinates():
    # Save the image with updated coordinates
    if current_image:
        new_filename = f"{int(point_x / fac)}_{int(point_y / fac)}_{current_filename_number}"
        output_path = os.path.join(output_directory, new_filename)
        current_image.save(output_path)
        print(f"Saved {new_filename}")
    update_image()

def set_coordinates_none():
    # Set coordinates to -1, -1 and save
    global point_x, point_y
    point_x, point_y = -1, -1
    save_coordinates()

def small_up():
    global point_y, fac
    point_y -= fac
    draw_point()

def small_down():
    global point_y, fac
    point_y += fac
    draw_point()

def small_left():
    global point_x, fac
    point_x -= fac
    draw_point()

def small_right():
    global point_x, fac
    point_x += fac
    draw_point()

# Create buttons
save_button = tk.Button(root, text="Save Point (f)", command=save_coordinates)
save_button.pack(side=tk.LEFT)

none_button = tk.Button(root, text="Set to (-1, -1) (d)", command=set_coordinates_none)
none_button.pack(side=tk.LEFT)

# Keyboard shortcuts
root.bind('f', lambda event: save_coordinates())
root.bind('d', lambda event: set_coordinates_none())
# short cuts for small adjustments using arrow keys
root.bind('<Up>', lambda event: small_up())
root.bind('<Down>', lambda event: small_down())
root.bind('<Left>', lambda event: small_left())
root.bind('<Right>', lambda event: small_right())

# Start with the first image
update_image()

root.mainloop()