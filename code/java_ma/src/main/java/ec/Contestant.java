package ec;

import nn.NeuralNetwork;
import nn.Pair;
import nn.TrainingSet;

import java.util.ArrayList;
import java.util.Arrays;

import static ec.EvolutionaryComputation.*;

public class Contestant {
    public final int layer_count;
    public final int[] layer_sizes;
    public final ArrayList<Integer> layers;
    public final int[] lookup_table;
    private final TrainingSet trainingSet;
    private final TrainingSet testSet;
    public NeuralNetwork nn;
    public int calculations;
    public int epochs;
    public Double learning_rate;
    public Double accuracy;
    public double average_error;
    public double max_error;
    public double cost;
    public double scaled_cost;
    public double fitness;
    int seed;
    private boolean isTrained = false;

    public Contestant(TrainingSet testSet, int epochs, int seed, int[] layer_sizes, Double learning_rate, TrainingSet trainingSet, int input_size, int output_size) {
        Pair<int[], ArrayList<Integer>> all_representations = fromLayerSizes(layer_sizes);

        this.testSet = testSet;
        this.epochs = epochs;
        this.layer_sizes = layer_sizes;
        this.learning_rate = learning_rate;
        this.lookup_table = all_representations.first;
        this.layer_count = all_representations.first.length;
        this.layers = addIO(all_representations.second, input_size, output_size);
        this.nn = new NeuralNetwork(this.layers, learning_rate, seed);
        this.trainingSet = trainingSet;
        this.calculations = calculationsCalculator(layers);
        this.seed = seed;
        System.out.println(Arrays.toString(this.layer_sizes));
        System.out.println(this.layers);
        System.out.println("Constructed!");
    }

    public static ArrayList<Integer> addIO(ArrayList<Integer> layers, int input_size, int output_size) {
        layers.add(0, input_size);
        layers.add(output_size);
        return layers;
    }

    public static int calculationsCalculator(ArrayList<Integer> layer_sizes) {
        int calculations = layer_sizes.get(0);
        for (int i = 1; i < layer_sizes.size(); i++) {
            calculations += layer_sizes.get(i) * layer_sizes.get(i - 1);
            calculations += layer_sizes.get(i);
        }
        return calculations;
    }

    public static Pair<int[], ArrayList<Integer>> fromLayerSizes(int[] layer_sizes) {
        ArrayList<Integer> layers = new ArrayList<Integer>();
        ArrayList<Integer> lookup = new ArrayList<Integer>();
        for (int i = 0; i < layer_sizes.length; i++) {
            if (layer_sizes[i] != 0) {
                if (layer_sizes[i] < 7) {
                    layer_sizes[i] = 7;
                }
                layers.add(layer_sizes[i]);
                lookup.add(i);
            }
        }
        return new Pair<>(lookup.stream().mapToInt(i -> i).toArray(), layers);
    }

    public void fit(boolean gui, int iter) {
        //fit the neural network
        nn.fit(trainingSet.tasks, trainingSet.targets, epochs, gui, iter, true, 10);
        //test the neural network
        Pair<Double, double[]> output = nn.fit(testSet.tasks, testSet.targets, 1, false, iter, false, 1);
        if (output.first == null || output.second == null) {
            System.err.println("Error: output is null");
            output.first = 0.0;
            output.second = new double[0];
        }
//        Pair<Double, double[]> output = nn.fit_smaller(trainingSet.tasks, trainingSet.targets, epochs, iter);
//        System.out.printf("{%s,%s}\n", output.first, Arrays.toString(output.second));
        nn.deleteWB();
        accuracy = output.first;

        average_error = 0.0;

        for (int i = 0; i < output.second.length; i++) {
            output.second[i] = Math.abs(output.second[i]);
            average_error += output.second[i];
            max_error = Math.max(max_error, output.second[i]);
        }
        average_error /= output.second.length;
        isTrained = true;
        cost = (Math.pow((average_error + max_error), error_scaler) * Math.pow(accuracy, accuracy_scaler) * Math.pow(calculations, calculations_scaler) * Math.pow(epochs, epochs_scaler));
        cost = average_error + max_error; //fuck this shit I'm out
    }

    public double calculateCost() {
        if (!isTrained) {
            fit(false, 0);
        }
        return cost;
    }

    public void printProperties() {
        System.out.println("Properties:");
        System.out.println("layer_sizes: " + Arrays.toString(layer_sizes));
        System.out.println("layer_sizes: " + layers);
        System.out.println("calculations: " + calculations);
        System.out.println("learning rate: " + learning_rate);
        System.out.println("epochs: " + epochs);
        System.out.println("Performance:");
        System.out.println("accuracy: " + accuracy);
        System.out.println("average error: " + average_error);
        System.out.println("max error: " + max_error);
        System.out.println("cost: " + cost);
        System.out.println("scaled cost: " + scaled_cost);
        System.out.println("fitness: " + fitness);
    }


}
