package ec.datasets;

import ec.EvolutionaryComputation;
import nn.Pair;
import nn.TrainingSet;
import nn.datasets.AllColors;

import java.io.IOException;

import static nn.datasets.AllColors.*;

public class AllColorsTrainer {
    public static int seed = 123456;
    public static int generations = 100;
    public static int population_size = 100;
    public static int max_epochs = 10000;
    public static int max_epochs_start = 500;
    public static int max_calculations = 10000;
    public static int max_calculations_start = 10000;
    public static int max_layer_count = 5;
    public static int max_nodes = 1000;
    public static int max_nodes_start = 100;
    public static double max_learning_rate = 0.0001;
    public static int min_epochs = 20;
    public static EvolutionaryComputation ec;
    public static void main(String[] args) throws IOException {
        Pair<TrainingSet, TrainingSet> sets = AllColors.getData_allColors(AllColors.testCount, AllColors.fileName);
        ec = new EvolutionaryComputation(sets.first, seed, generations, population_size, AllColors.input_size, AllColors.output_size, max_epochs, max_epochs_start, max_calculations, max_calculations_start, max_layer_count, max_nodes, max_nodes_start, max_learning_rate, min_epochs, sets.second);
        ec.run();
    }
}
