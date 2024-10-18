package nn.img_processing;

import nn.TrainingSet;

import javax.imageio.ImageIO;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

import static nn.img_processing.ImageDistortion.fakeDistortion;

public class ReadImg {
    public static void main(String[] args) throws IOException {
        // read image in src/resources/base_img.png
        // and print its dimensions
        File file = new File("./base_img5.png");
        BufferedImage image = ImageIO.read(file);
        int width = image.getWidth();
        int height = image.getHeight();
        System.out.println("Width: " + width + " Height: " + height);

        // split the image into 4 parts: top-left, top-right, bottom-left, bottom-right
        // horizontally we split it into 6 parts and take the first and the last
        // vertically in 4 parts
        int verticalSplit = 4;
        int horizontalSplit = 6;
        BufferedImage topLeft = image.getSubimage(0, 0, width / horizontalSplit, height / verticalSplit);
        BufferedImage topRight = image.getSubimage(width - width / horizontalSplit, 0, width / horizontalSplit, height / verticalSplit);
        BufferedImage bottomLeft = image.getSubimage(0, height - height / verticalSplit, width / horizontalSplit, height / verticalSplit);
        BufferedImage bottomRight = image.getSubimage(width - width / horizontalSplit, height - height / verticalSplit, width / horizontalSplit, height / verticalSplit);

        // save the images
        ImageIO.write(topLeft, "png", new File("./splits/topLeft.png"));
        ImageIO.write(topRight, "png", new File("./splits/topRight.png"));
        ImageIO.write(bottomLeft, "png", new File("./splits/bottomLeft.png"));
        ImageIO.write(bottomRight, "png", new File("./splits/bottomRight.png"));

        TrainingSet testSet = GenTrainingSet(List.of(new BufferedImage[]{topLeft, topRight, bottomLeft, bottomRight}), 20, "test_");
        testSet.writeToFile("test");

        int n = 14;
        TrainingSet trainingSet = GenTrainingSet(List.of(new BufferedImage[]{topLeft, topRight, bottomLeft, bottomRight}), n * n, "training_");
        trainingSet.writeToFile("training");


        int input_layer_size = testSet.tasks[0].length;
        int output_layer_size = testSet.targets[0].length;
        System.out.println("Input layer size: " + input_layer_size + " Output layer size: " + output_layer_size);

        System.out.println("Amount of tasks: " + testSet.tasks.length + " Amount of targets: " + testSet.targets.length);

//        NeuralNetwork nn = new NeuralNetwork(new ArrayList<>(Arrays.asList(input_layer_size, input_layer_size * 2, input_layer_size / 2, input_layer_size / 10, output_layer_size * 2, output_layer_size)), 0.0001, 123456);
//        // change to standard relu activation function
//        nn.activationFunction = x -> {
//            if (x.isNaN()) {
//                throw new RuntimeException("NaN");
//            } else {
//                return Math.max(0, x);
//            }
//        };
//        nn.dActivationFunction = x -> (double) (x > 0 ? 1 : 0);
//        nn.fit(trainingSet.tasks, trainingSet.targets, 100, true, 1, true, 11);

    }

    public static class Pair<T, U> {
        public T first;
        public U second;

        public Pair(T first, U second) {
            this.first = first;
            this.second = second;
        }
    }


    public static Pair<BufferedImage, Pair<Integer, Integer>> placeFiducial(int number, BufferedImage image, int degrees, double n, double m, int x_offset, int y_offset, int skip) {
        // copy new image
        BufferedImage newImage = new BufferedImage(image.getWidth() / skip, image.getHeight() / skip, BufferedImage.TYPE_INT_RGB);
        for (int i = 0; i + (skip == 2 ? 1 : 0) < image.getWidth(); i += skip) {
            for (int j = 0; j + (skip == 2 ? 1 : 0) < image.getHeight(); j += skip) {
                newImage.setRGB(i / skip, j / skip, image.getRGB(i, j));
            }
        }
        // we want to read the fidcuial image in ./fiducials/fiducial_<number>.png
        // then we place that image in the middle of the input image, so that it is 1/n of the width and 1/m height of the input image
        // we return the new image
        int midX = 0;
        int midY = 0;
        try {
            BufferedImage oFiducial = ImageIO.read(new File("./fiducials/fiducial_" + number + ".png"));
            BufferedImage fiducial = ImageRotator.rotateImage(oFiducial, degrees);
            int width = newImage.getWidth();
            int height = newImage.getHeight();
//            System.out.println("Fiducial width: " + fiducial.getWidth() + " Fiducial height: " + fiducial.getHeight());
            int fiducialWidth = (int) ((double) width / n);
            int fiducialHeight = (int) ((double) height / m);
            int x = (width - fiducialWidth) / 2 + x_offset;
            int y = (height - fiducialHeight) / 2 + y_offset;
            for (int i = 0; i < fiducialWidth; i++) {
                for (int j = 0; j < fiducialHeight; j++) {
                    try {
                        int rgb = fiducial.getRGB(i * fiducial.getWidth() / fiducialWidth, j * fiducial.getHeight() / fiducialHeight);
                        int alpha = fiducial.getAlphaRaster().getSample(i * fiducial.getWidth() / fiducialWidth, j * fiducial.getHeight() / fiducialHeight, 0);
                        if (alpha != 0) {
                            int oRgb = image.getRGB(x + i, y + j);
                            double fid = 0.8;
                            double org = 1 - fid;
                            // add 20 precent of the old color to the new color
                            int red = (oRgb >> 16) & 0xff;
                            int green = (oRgb >> 8) & 0xff;
                            int blue = oRgb & 0xff;
                            red = (int) (org * red + fid * ((rgb >> 16) & 0xff));
                            green = (int) (org * green + fid * ((rgb >> 8) & 0xff));
                            blue = (int) (org * blue + fid * (rgb & 0xff));
                            rgb = (red << 16) | (green << 8) | blue;
                            newImage.setRGB(x + i, y + j, rgb);
                        }
                    } catch (Exception e) {
                    }
                }
            }
            // calculate where the midpoint of the fiducial is
            // for the midpoint x: i * fiducial.getWidth() / fiducialWidth = fiducial.getWidth() / 2
            // so i = fiducialWidth / 2
            // for the midpoint y: j * fiducial.getHeight() / fiducialHeight = fiducial.getHeight() / 2
            // so j = fiducial.getHeight() / 2 * fiducialHeight
            midX = x + fiducialWidth / 2;
            midY = y + fiducialHeight / 2;
        } catch (Exception e) {
            System.out.println("placeFiducial: Error: " + e);
        }
        return new Pair<>(newImage, new Pair<>(midX, midY));
    }

    public static TrainingSet GenTrainingSet(List<BufferedImage> allImages, int amountPerImage, String prefix) {
        List<List<Double>> inputs = new ArrayList<>();
        List<List<Double>> targets = new ArrayList<>();
        // -> 1,2,3,4
        Pair<Integer, Integer> fiducialLimits = new Pair<>(1, 4);
        int[] preRotations = new int[]{0, 90, 180, 270};
        // -> -5 to 5
        Pair<Integer, Integer> degreeLimits = new Pair<>(-5, 5);
        // -> 1.8 to 2.2
        Pair<Double, Double> nLimits = new Pair<>(1.4, 2.2);
        // -> 1.8 to 2.2
        Pair<Double, Double> mLimits = new Pair<>(1.4, 2.2);
        // -> -20 to 20
        Pair<Integer, Integer> x_offsetLimits = new Pair<>(-20, 20);
        // -> -20 to 20
        Pair<Integer, Integer> y_offsetLimits = new Pair<>(-20, 20);

        // check if the folder training_set exists, if not create it
        File trainingSetFolder = new File("./training_set");
        if (!trainingSetFolder.exists()) {
            trainingSetFolder.mkdir();
        }

        int skip = 1;
// create an image containing all images, so we can see the test inputs.
        // should be a square so each side is the square root of the amount of images
        int side = (int) Math.ceil(Math.sqrt(amountPerImage));
        System.out.println("side: " + side);
        BufferedImage wholeGeneralImage = new BufferedImage(allImages.get(0).getWidth() / skip * side * 2 + 1, allImages.get(0).getHeight() / skip * side * 2 + 1, BufferedImage.TYPE_INT_RGB);

        int amount = 0;
        List<List<Pair<BufferedImage, Pair<Integer, Integer>>>> fiducialsSorted = new ArrayList<>(fiducialLimits.second);
        for (BufferedImage img : allImages) {

            // remove the divide by 2 to get the full image
            BufferedImage generalImage = new BufferedImage(img.getWidth() / skip * side, img.getHeight() / skip * side, BufferedImage.TYPE_INT_RGB);

            for (int i = 0; i < amountPerImage; i++) {
                // generate random values for the parameters
                int fiducial = (int) (Math.random() * (fiducialLimits.second - fiducialLimits.first + 1) + fiducialLimits.first);
                int rotation = preRotations[(int) (Math.random() * preRotations.length)];
                int d = (int) (Math.random() * (degreeLimits.second - degreeLimits.first + 1) + degreeLimits.first) + rotation;
                double n = Math.random() * (nLimits.second - nLimits.first) + nLimits.first;
                double m = Math.random() * (mLimits.second - mLimits.first) + mLimits.first;
                int x_offset = (int) (Math.random() * (x_offsetLimits.second - x_offsetLimits.first + 1) + x_offsetLimits.first);
                int y_offset = (int) (Math.random() * (y_offsetLimits.second - y_offsetLimits.first + 1) + y_offsetLimits.first);
                Pair<BufferedImage, Pair<Integer, Integer>> placedFiducial = placeFiducial(fiducial, img, d, n, m, x_offset, y_offset, skip);
                int fidWidth = placedFiducial.first.getWidth();
                int fidHeight = placedFiducial.first.getHeight();
//                BufferedImage newImage = resizeImage(placedFiducial.first, fidWidth/2, fidHeight/2);
//                placedFiducial.first = fakeDistortion(728, 544, resizeImage(newImage, fidWidth, fidHeight), amount, 4, 6);
                if (fiducialsSorted.size() < fiducial + 1) {
                    for (int j = fiducialsSorted.size(); j < fiducial + 1; j++) {
                        fiducialsSorted.add(new ArrayList<>());
                    }
                }
                fiducialsSorted.get(fiducial).add(placedFiducial);
                int midX = placedFiducial.second.first;
                int midY = placedFiducial.second.second;
                // save the image in ./training_set/img_<i>.png
                try {
                    if (i == 0) {
                        ImageIO.write(placedFiducial.first, "png", new File("./training_set/img_" + i + ".png"));
                        System.out.println("Saved image: " + i + " Info: fid:" + fiducial + " " + d + " " + n + " " + m + " " + x_offset + " " + y_offset + " x: " + midX + " y: " + midY);
                        System.out.println("Image Dimensions: " + placedFiducial.first.getWidth() + " " + placedFiducial.first.getHeight());
                    }
                } catch (Exception e) {
                    System.out.println("Error: " + e);
                }

                BufferedImage copyImage = new BufferedImage(placedFiducial.first.getWidth(), placedFiducial.first.getHeight(), BufferedImage.TYPE_INT_RGB);
                for (int j = 0; j < placedFiducial.first.getWidth(); j++) {
                    for (int k = 0; k < placedFiducial.first.getHeight(); k++) {
                        copyImage.setRGB(j, k, placedFiducial.first.getRGB(j, k));
                    }
                }

                // add round circle around the middle at the midpoint (midX, midY)
                int radius = 2;
                for (int j = -radius; j <= radius; j++) {
                    for (int k = -radius; k <= radius; k++) {
                        if (j * j + k * k <= radius * radius) {
                            try {
                                copyImage.setRGB(midX + j, midY + k, 0xff0000);
                            } catch (Exception e) {
                            }
                        }
                    }
                }

                // add image to general image
                for (int j = 0; j < placedFiducial.first.getWidth(); j++) {
                    for (int k = 0; k < placedFiducial.first.getHeight(); k++) {
                        generalImage.setRGB(j + (i % side) * placedFiducial.first.getWidth(), k + (i / side) * placedFiducial.first.getHeight(), copyImage.getRGB(j, k));
                    }
                }

                // add the parameters to the input
                List<Double> input = new ArrayList<>();
                // iterate through image and add the rgb values to the input
                for (int j = 0; j < placedFiducial.first.getWidth(); j++) {
                    for (int k = 0; k < placedFiducial.first.getHeight(); k++) {
                        int rgb = placedFiducial.first.getRGB(j, k);
                        // convert bitwise to double, so the bit representation is the same
//                        input.add(intToDouble(rgb));
                        int gray = (int) (0.299 * ((rgb >> 16) & 0xff) + 0.587 * ((rgb >> 8) & 0xff) + 0.114 * (rgb & 0xff));
//                        input.add((double) ((rgb >> 16) & 0xff));
//                        input.add((double) ((rgb >> 8) & 0xff));
//                        input.add((double) (rgb & 0xff));
                        input.add((double) gray);
                    }
                }
                inputs.add(input);
//                // add the parameters to the target
//                List<Double> target = new ArrayList<>(4 + img.getHeight() + img.getWidth());
//                // fill with zeros
//                for (int j = 0; j < 4 + img.getHeight() + img.getWidth(); j++) {
//                    target.add(0d);
//                }
//                target.set(fiducial - 1, 1d);
//                target.set(4 + midX, 1d);
//                target.set(4 + img.getWidth() + midY, 1d);
//                targets.add(target);
                List<Double> target = new ArrayList<>(0);
//                target.add((double) fiducial);
                target.add((double) midX);
                target.add((double) midY);
                targets.add(target);
            }

            // add the general image to the whole general image
            for (int i = 0; i < generalImage.getWidth(); i++) {
                for (int j = 0; j < generalImage.getHeight(); j++) {
                    wholeGeneralImage.setRGB(i + (amount % 2) * generalImage.getWidth(), j + (amount / 2) * generalImage.getHeight(), generalImage.getRGB(i, j));
                }
            }
            amount++;

            // save the general image
            try {
                ImageIO.write(generalImage, "png", new File("./training_set/" + prefix + "general_image.png"));
            } catch (Exception e) {
                System.out.println("Error: " + e);
            }
        }
        // save the general image
        try {
            ImageIO.write(wholeGeneralImage, "png", new File("./training_set/" + prefix + "whole_general_image.png"));
        } catch (Exception e) {
            System.out.println("Error: " + e);
        }

        String base = "./fiducial_" + prefix + "sets/";
        File gen_folder = new File(base);
        if (!gen_folder.exists()) {
            gen_folder.mkdir();
        }
        int fiducial_index = 0;
        for (List<Pair<BufferedImage, Pair<Integer, Integer>>> fiducials : fiducialsSorted) {
            if (fiducials.isEmpty()) {
                fiducial_index++;
                continue;
            }
            // create folder
            File folder = new File(base + "fiducial_" + fiducial_index);
            if (!folder.exists()) {
                folder.mkdir();
            } else {
                // delete all files in the folder
                File[] files = folder.listFiles();
                if (files != null) {
                    for (File f : files) {
                        f.delete();
                    }
                }
            }
            for (int j = 0; j < fiducials.size(); j++) {
                try {
                    int x = fiducials.get(j).second.first;
                    int y = fiducials.get(j).second.second;
                    ImageIO.write(fiducials.get(j).first, "png", new File(base + "/fiducial_" + fiducial_index + "/" + j + "_" + x + "_" + y + ".png"));
                } catch (Exception e) {
                    System.out.println("Error: " + e);
                }
            }
            fiducial_index++;
        }

        System.out.println("finished generating training set");
        // convert targets and inputs to arrays
        double[][] inputsArray = new double[inputs.size()][];
        double[][] targetsArray = new double[targets.size()][];
        for (int i = 0; i < inputs.size(); i++) {
            inputsArray[i] = new double[inputs.get(i).size()];
            for (int j = 0; j < inputs.get(i).size(); j++) {
                inputsArray[i][j] = inputs.get(i).get(j);
            }
            targetsArray[i] = new double[targets.get(i).size()];
            for (int j = 0; j < targets.get(i).size(); j++) {
                targetsArray[i][j] = targets.get(i).get(j);
            }
        }

        return new TrainingSet(inputsArray, targetsArray);
    }

    public static double intToDouble(int value) {
        // Convert the int to long to preserve the 32 bits in a 64-bit long
        long longValue = value & 0xFFFFFFFFL; // Zero-extend to 64 bits

        // Reinterpret the long bits as a double
        return Double.longBitsToDouble(longValue);
    }

    public static BufferedImage resizeImage(BufferedImage inputImage, int width, int height) {

        // Create a new output image with the specified width and height
        BufferedImage outputImage = new BufferedImage(width, height, inputImage.getType());

        // Draw the input image onto the output image, scaling it to the new size
        Graphics2D g2d = outputImage.createGraphics();
        g2d.drawImage(inputImage, 0, 0, width, height, null);
        g2d.dispose();
        return outputImage;
    }
}