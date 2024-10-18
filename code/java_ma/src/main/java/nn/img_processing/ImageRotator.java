package nn.img_processing;

import javax.imageio.ImageIO;
import java.awt.Graphics2D;
import java.awt.RenderingHints;
import java.awt.geom.AffineTransform;
import java.awt.image.BufferedImage;
import java.io.File;

public class ImageRotator {

    public static BufferedImage rotateImage(BufferedImage image, double degrees) {
        int width = image.getWidth();
        int height = image.getHeight();
        double radians = Math.toRadians(degrees);

        // Calculate the new image dimensions after rotation
        double sin = Math.abs(Math.sin(radians));
        double cos = Math.abs(Math.cos(radians));
        int newWidth = (int) Math.floor(width * cos + height * sin);
        int newHeight = (int) Math.floor(height * cos + width * sin);

        // Create a new transparent BufferedImage to hold the rotated image
        BufferedImage rotatedImage = new BufferedImage(newWidth, newHeight, BufferedImage.TYPE_INT_ARGB);

        // Create a Graphics2D object to perform the rotation with transparency
        Graphics2D g2d = rotatedImage.createGraphics();

        // Enable high-quality rendering (optional)
        g2d.setRenderingHint(RenderingHints.KEY_INTERPOLATION, RenderingHints.VALUE_INTERPOLATION_BILINEAR);
        g2d.setRenderingHint(RenderingHints.KEY_RENDERING, RenderingHints.VALUE_RENDER_QUALITY);
        g2d.setRenderingHint(RenderingHints.KEY_ANTIALIASING, RenderingHints.VALUE_ANTIALIAS_ON);

        // Set up the rotation transform, rotating around the new center of the image
        AffineTransform transform = new AffineTransform();
        transform.translate((newWidth - width) / 2.0, (newHeight - height) / 2.0); // Center the image
        transform.rotate(radians, width / 2.0, height / 2.0); // Rotate around the center of the original image

        // Draw the original image onto the rotated image
        g2d.setTransform(transform);
        g2d.drawImage(image, 0, 0, null);
        g2d.dispose();

        return rotatedImage;
    }

    public static void main(String[] args) {
        try {
            // Load the image
            BufferedImage image = ImageIO.read(new File("./fiducials/fiducial_1.png"));

            // Rotate the image by 45 degrees
            BufferedImage rotatedImage = rotateImage(image, 45);

            // Save the rotated image
            ImageIO.write(rotatedImage, "png", new File("./rotated_image.png"));
        } catch (Exception e) {
            System.out.println("Error: " + e);
        }
    }
}
