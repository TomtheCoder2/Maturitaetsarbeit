package nn;

import java.io.FileWriter;
import java.io.IOException;
import java.util.Arrays;

public class Utility {
    public static Utility.color color;

    public static void printNN(NeuralNetwork nn) {
        printNN(nn, null);
    }

    public static void printNN(NeuralNetwork nn, String suffix) {
        // TODO: fix this
        StringBuilder sb = new StringBuilder();
        sb.append("weights_list = vec![");
        for (Matrix m : nn.weights) {
            sb.append(Arrays.deepToString(m.data).replace("[", "vec![").replace("],", "]").replace("]", "],"));
        }
        sb.append("];\n");
        sb.append("biases_list = vec![");
        for (Matrix m : nn.biases) {
            sb.append(Arrays.deepToString(m.data).replace("[", "vec![").replace("],", "]").replace("]", "],"));
        }
        sb.append("];\n");
        sb.append("for i in 0 .. biases_list.len() {\n" +
                "        weights.push(Matrix::from_2d_array(weights_list[i].clone()));\n" +
                "        biases.push(Matrix::from_2d_array(biases_list[i].clone()));\n" +
                "    } \n" +
                "NN_YOUR_NETWORK.lock().unwrap().weights = weights;\n" +
                "    NN_YOUR_NETWORK.lock().unwrap().biases = biases;\n" +
                "    weights = vec![];\n" +
                "    biases = vec![];\n");

/*
        System.out.println(".lock().unwrap().bias_output = nn.Matrix::from_2d_array(" + Arrays.deepToString(nn.layers.data).replace("[", "vec![") + ");");
        System.out.println(".lock().unwrap().weights_input_hidden = nn.Matrix::from_2d_array(" + Arrays.deepToString(nn.weights_ih.data).replace("[", "vec![") + ");");
*/

        try {
            FileWriter myWriter = new FileWriter("output" + (suffix != null ? suffix : "") + ".rust");
            myWriter.write(sb.toString());
            myWriter.close();
            System.out.println("Successfully wrote to the file.");
        } catch (IOException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
    }

    public static void progressPercentage(String desc, int remain, int total) {
//        System.out.printf("%s: %d/%d (%.2f%%)%n", desc, remain, total, (double) remain / total * 100);
        if (remain > total) {
            throw new IllegalArgumentException();
        }
        int maxBareSize = 40; // 10 units for 100%
        int remainPercent = (int) ((double) remain / total * 100.0);
        int remainBareSize = (int) ((double) remainPercent / 100.0 * maxBareSize);
//        System.out.println("remainPercent = " + remainPercent);
        char defaultChar = ' ';
        char remainChar = '#';
        StringBuilder bare = new StringBuilder(desc + ": [");
        for (int i = 0; i < maxBareSize; i++) {
            if (i < remainBareSize) {
                bare.append(remainChar);
            } else {
                bare.append(defaultChar);
            }
        }
        System.out.print("\r" + bare + "] " + remainPercent + "%");
        if (remain == total) {
            System.out.print("\n");
        }
    }

    public enum color {
        BLACK, WHITE, RED, GREEN, BLUE, YELLOW, NOTHING, UNKNOWN
    }
}
