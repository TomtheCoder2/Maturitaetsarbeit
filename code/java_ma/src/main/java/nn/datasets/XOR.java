package nn.datasets;

import nn.NeuralNetwork;

import java.util.ArrayList;
import java.util.Arrays;

public class XOR {
    public static void main(String[] args) {
        NeuralNetwork nn = new NeuralNetwork(new ArrayList<>(Arrays.asList(2, 3, 1)), 1, 123456);
        nn.fit(new double[][]{{0, 0},
                {0, 1},
                {1, 0},
                {1, 1}}, new double[][]{{0},
                {1},
                {1},
                {0}}, 1000, true, 1, true, 11);
    }
}
