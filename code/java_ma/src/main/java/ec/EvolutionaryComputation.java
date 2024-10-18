package ec;

import nn.TrainingSet;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Random;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;

import static ec.Contestant.fromLayerSizes;

public class EvolutionaryComputation {
    public static final boolean use_advanced_function = true;
    //parent choosing
    //every contestant has an error value, which is taken to the power of the error_scaler
    //the fitness value is for every contestant is the error value of the contestant with the highest error value divided by the own error value
    //every contest gets a parent chance in every iteration which is the fitness value of the contestant divided by the sum of all fitness values
    //cost function:
    public final static double cost_scaler = 1.2;
    public final static double error_scaler = 2.5;
    public final static double accuracy_scaler = 1.2;
    public final static double calculations_scaler = 0.2;
    public final static double epochs_scaler = 0.2;
    //rate of mutation
    public final static double parameter_change_rate_exp = 5;
    public final static double parameter_change_rate_linear = 1;
    public final static double exp_parameter_change_rate_exp = 5;
    public final static double exp_parameter_change_rate_linear = 10;
    public final static double asexual_reproduction = 0.75;
    public final static double sexual_reproduction = 0.22;

    //asexual reproduction
    //(-1 .. 1 rand) ** change_rate * (max - min) + min
    public final static double asexual_keep_rate = 0.95;
    //sexual reproduction
    //how much to keep from both parents - the rest is mixed together
    public final static double sexual_keep_rate = 0.3;
    private final Random generator;
    public Collection<Future<?>> todo;
    public int currentGeneration = 0;
    TrainingSet trainingSet;
    TrainingSet testSet;
    int seed;
    //rest is generated randomly
    int generations;
    ArrayList<Contestant> population;
    int population_size;
    int input_size;
    int output_size;
    int max_epochs;
    int max_epochs_start;
    int min_epochs;
    int max_calculations;
    int max_calculations_start;
    int max_layer_count;
    int max_nodes;
    int max_nodes_start;
    double max_learning_rate;

    public EvolutionaryComputation(TrainingSet trainingSet, int seed, int generations, int population_size, int input_size, int output_size, int max_epochs, int max_epochs_start, int max_calculations, int max_calculations_start, int max_layer_count, int max_nodes, int max_nodes_start, double max_learning_rate, int min_epochs, TrainingSet testSet) {
        this.trainingSet = trainingSet;
        this.testSet = testSet;
        this.seed = seed;
        this.generations = generations;
        this.population_size = population_size;
        this.population = new ArrayList<>();
        this.generator = new Random(seed);
        this.input_size = input_size;
        this.output_size = output_size;
        this.max_epochs = max_epochs;
        this.max_epochs_start = max_epochs_start;
        this.min_epochs = min_epochs;
        this.max_calculations = max_calculations;
        this.max_calculations_start = max_calculations_start;
        this.max_layer_count = max_layer_count;
        this.max_nodes = max_nodes;
        this.max_nodes_start = max_nodes_start;
        this.max_learning_rate = max_learning_rate;
    }

    public static Contestant getBest(ArrayList<Contestant> population) {
        Contestant best_contestant = population.get(0);
        for (Contestant contestant : population) {
            if (contestant.cost < best_contestant.cost) best_contestant = contestant;
        }
        return best_contestant;
    }

    public static Contestant getWorst(ArrayList<Contestant> population) {
        Contestant worst_contestant = population.get(0);
        for (Contestant contestant : population) {
            if (contestant.cost > worst_contestant.cost) worst_contestant = contestant;
        }
        return worst_contestant;
    }

    public void run() {
        ArrayList<Contestant> new_population;
        for (int i = 0; i < population_size; i++) {
            population.add(generateRandomStart());
        }
        for (int i = 0; i < generations; i++) {
            currentGeneration = i;
            long start = System.currentTimeMillis();
            System.out.println("Generation " + i);
            ExecutorService executorService = Executors.newFixedThreadPool(population_size);
            boolean gui = true;
            todo = new ArrayList<>();
            for (int j = 0; j < population_size; j++) {
                int finalJ = j;
                boolean finalGui = gui;
                todo.add(executorService.submit(() -> population.get(finalJ).fit(finalGui, finalJ)));
                gui = false;
            }
            for (Future<?> f : todo) {
                try {
                    f.get();
                } catch (Exception e) {
                    e.printStackTrace();
                }
            }
            /*
            for (int iter = 0; iter < population_size; iter++) {
                population.get(iter).fit(true, iter);
            }*/
//            fit(nns, from_2d_array(trainingSet.tasks), from_2d_array(trainingSet.targets), epochs, learning_rate);
//            Kernel kernel = new Kernel() {
//                @Override
//                public void run() {
//                    int gid = getGlobalId();
//                    population.get(gid).fit(gid % population_size == 0, gid);
//                }
//            };
//            Device device = Device.best();
//            for (Device d : OpenCLDevice.listDevices(Device.TYPE.GPU)) {
//                System.out.println(d.getShortDescription());
//                if (d.getShortDescription().contains("NVIDIA")) {
//                    device = d;
//                    break;
//                }
//                System.out.println(d);
//            }
//            Range RANGE = device.createRange(population_size);
//            kernel.execute(RANGE);
//            kernel.dispose();
            System.out.println("Best contestant: ");
            getBest(population).printProperties();
            new_population = nextGeneration(population);
            population = new_population;
            long finish = System.currentTimeMillis();
            long timeElapsed = finish - start;
            System.out.println("Gen #" + i + ": " + timeElapsed + "ms");
        }
    }

    public ArrayList<Contestant> nextGeneration(ArrayList<Contestant> population) {
        ArrayList<Contestant> next_population = new ArrayList<>();
        for (Contestant contestant : population) {
            contestant.scaled_cost = Math.pow(contestant.cost, cost_scaler);
        }
        Contestant worst_contestant = getWorst(population);
        double fitness_sum = 0;
        for (Contestant contestant : population) {
            contestant.fitness = worst_contestant.scaled_cost / contestant.scaled_cost;
            fitness_sum += contestant.fitness;
        }
        for (int i = 0; i < population_size; i++) {
            System.out.println("Contestant " + i);
            population.get(i).printProperties();
        }

        System.out.println("Fitness sum: " + fitness_sum);
        for (int i = 0; i < population_size; i++) {
            int point = contestantInt(population, fitness_sum);
            if (generator.nextDouble() < asexual_reproduction) {
                System.out.println("Asexual reproduction: ");
                System.out.println("Old contestant: ");
                System.out.println("Contestant #" + point);
                population.get(point).printProperties();
                next_population.add(mutate(population.get(point)));
                System.out.println("New contestant: ");
                next_population.get(next_population.size() - 1).printProperties();
            } else if (generator.nextDouble() < sexual_reproduction + asexual_reproduction) {
                System.out.println("Sexual reproduction: ");
                System.out.println("Old contestants: ");
                System.out.println("Contestant #" + point);
                int point2 = contestantInt(population, fitness_sum);
                population.get(point).printProperties();
                System.out.println("Contestant #" + point2);
                population.get(point2).printProperties();
                next_population.add(mutate(sexualReproduction(population.get(point), population.get(point2))));
                System.out.println("New contestant: ");
                next_population.get(next_population.size() - 1).printProperties();
            } else {
                next_population.add(generateRandomStart());
            }
        }
        return next_population;
    }

    private int contestantInt(ArrayList<Contestant> population, double fitness_sum) {
        double current_sum = population.get(0).fitness;
        int point = 0;
        double goal = generator.nextDouble(fitness_sum);
        while (current_sum < goal) {
            point++;
            current_sum += population.get(point).fitness;
        }
        return point;
    }

    public Contestant generateRandomStart() {
        int[] layer_sizes = new int[max_layer_count];
        for (int i = 0; i < layer_sizes.length; i++) {
            layer_sizes[i] = generator.nextInt(max_nodes_start);
        }
        while (Contestant.calculationsCalculator(Contestant.addIO(fromLayerSizes(layer_sizes).second, input_size, output_size)) > max_calculations_start) {
            layer_sizes[fromLayerSizes(layer_sizes).first[generator.nextInt(fromLayerSizes(layer_sizes).first.length - 1)]] = 0;
        }
        int[] lookup = fromLayerSizes(layer_sizes).first;
        double keep_rate = generator.nextDouble();

        for (int i : lookup) {
            if (generator.nextDouble() < keep_rate) {
                layer_sizes[i] = 0;
            }
        }

        return new Contestant(testSet, generator.nextInt(max_epochs_start - min_epochs) + min_epochs, seed, layer_sizes, generator.nextDouble() * max_learning_rate, trainingSet, input_size, output_size);
    }

    public Contestant sexualReproduction(Contestant a, Contestant b) {
        int[] layer_sizes = new int[max_layer_count];
        for (int i = 0; i < layer_sizes.length; i++) {
            if (generator.nextBoolean()) {
                layer_sizes[i] = a.layer_sizes[i];
            } else {
                layer_sizes[i] = b.layer_sizes[i];
            }
        }
        Contestant out = new Contestant(testSet, (int) merge(a.epochs, b.epochs), seed, layer_sizes, merge(a.learning_rate, b.learning_rate), trainingSet, input_size, output_size);
        out.fitness = merge(a.fitness, b.fitness);
        return out;
    }

    private double merge(double a, double b) {
        if (generator.nextDouble() < sexual_keep_rate) {
            if (generator.nextBoolean()) {
                return a;
            } else {
                return b;
            }
        } else {
            return (a + b) / 2;
        }
    }

    public Contestant mutate(Contestant contestant) {
        int[] layer_sizes = contestant.layer_sizes.clone();
        for (int i = 0; i < max_layer_count; i++) {
            if (layer_sizes[i] > 0) {
                if (generator.nextDouble() < asexual_keep_rate) {
                    //keep layer
                    layer_sizes[i] = (int) calculateChange(1, max_nodes, layer_sizes[i], contestant.fitness);
                } else {
                    layer_sizes[i] = 0;
                }
            } else {
                if (generator.nextDouble() * asexual_keep_rate < 0.25) {
                    layer_sizes[i] = generator.nextInt(max_nodes_start);
                }
            }
        }
        while (Contestant.calculationsCalculator(Contestant.addIO(fromLayerSizes(layer_sizes).second, input_size, output_size)) > max_calculations) {
            int next_int = fromLayerSizes(layer_sizes).first.length;
            layer_sizes[fromLayerSizes(layer_sizes).first[generator.nextInt(next_int)]] = 0;
        }

        return new Contestant(testSet, (int) calculateChange(min_epochs, max_epochs, contestant.epochs, contestant.fitness), seed, layer_sizes, calculateChange(0, max_learning_rate, contestant.learning_rate, contestant.fitness), trainingSet, input_size, output_size);
    }

    private double calculateChange(double min, double max, double current, double fitness) {
        //hac
        if (use_advanced_function) return calculateChangeExp(min, max, current, fitness);
        return Math.min(
                Math.max(
                        current + (
                                (
                                        (Math.pow(((generator.nextDouble() * 2) - 1), parameter_change_rate_exp)
                                                * (max - min) + min)
                                                * parameter_change_rate_linear
                                ) / fitness),
                        min),
                max);
    }

    //TODO: calculateChangeExp which is also oriented at the current value
    private double calculateChangeExp(double min, double max, double current, double fitness) {
        return Math.min(
                Math.max(
                        current + (
                                (
                                        (Math.pow(((generator.nextDouble() * 2) - 1), exp_parameter_change_rate_exp)
                                                * (current - min))
                                                * exp_parameter_change_rate_linear
//                                                * (current / (max - min))
                                ) / fitness),
                        min),
                max);
    }
}
