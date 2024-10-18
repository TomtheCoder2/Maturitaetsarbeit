package nn.gpu;


import com.aparapi.Kernel;
import nn.Pair;

public class NeuralNetworkOpenCL3 {
    public double[] weights;
    public double[] biases;
    public int[] layer_sizes;
    public int contestant_count;
    public int[] epochs;
    public double[] learning_rates;
    public int test_count;
    public int[] layerCounts;
    public double[] input;
    public Pair<Integer, Integer> inputSize;
    public double[] target;
    public Pair<Integer, Integer> targetSize;
    public double[] output; // temporary output
    int gid; // current training iteration
    int layer_count; // current training iteration
    int layer_size; // current training iteration
    int weight_index; // current training iteration
    int bias_index; // current training iteration
    int epoch; // current training iteration
    double learning_rate; // current training iteration
    private int[] weightIndexes;
    private int[] biasesIndexes;

    public NeuralNetworkOpenCL3(double[] weights, double[] biases, int[] layer_sizes, int contestant_count, int[] epochs, double[] learning_rates, int test_count, int[] layerCounts, double[] input, Pair<Integer, Integer> inputSize, double[] target, Pair<Integer, Integer> targetSize, double[] output) {
        this.weights = weights;
        this.biases = biases;
        this.layer_sizes = layer_sizes;
        this.contestant_count = contestant_count;
        this.epochs = epochs;
        this.learning_rates = learning_rates;
        this.test_count = test_count;
        this.layerCounts = layerCounts;
        this.input = input;
        this.inputSize = inputSize;
        this.target = target;
        this.targetSize = targetSize;
        this.output = output;
    }

    public void fit() {
        for (int i = 0; i < epoch; i++) {
            train();
        }
    }

    private void train() {
        // predict output
        int output_size = 0;
        for (int i = 0; i < layer_count; i++) {
            output_size += layer_sizes[i];
        }
        output = new double[output_size];
        if (inputSize.second >= 0) System.arraycopy(input, 0, output, 0, inputSize.second);
        for (int i = 1; i < layer_sizes.length; i++) {
            // multiply weights and input
            // compute where weights[i - 1] is
            int wi = 0;
            for (int j = 0; j < i - 1; j++) {
                wi += layer_sizes[j];
            }
//            mult(weight_index + wi, weight_index + wi + layer_sizes[i - 1], output, output_size, layer_sizes[i]);
        }

//        // predict the output
//        // convert the input to a nn.Matrix
//        ArrayList<Matrix> layers = new ArrayList<>();
//        layers.add(Matrix.fromArray(X));
//
//        for (int i = 1; i < layer_sizes.size(); i++) {
//            layers.add(Matrix.multiply(weights.get(i - 1), layers.get(i - 1)));
//            layers.get(i).add(biases.get(i - 1));
//            layers.get(i).sigmoid();
////            System.out.println("layer: " + i);
////            System.out.println(layers.get(i));
//        }
////        System.out.println("output: " + layers.get(layers.size() - 1));
//
//        // error detection and correction
//        Matrix target = Matrix.fromArray(Y);
//
//        // calculate the error between the output and the correct output (target)
//        Matrix error = Matrix.subtract(target, layers.get(layers.size() - 1));
//
//        Matrix transposed;
////        System.out.println("error: " + error);
//
//        // use the derivative sigmoid function to correct the error
//        correctError(layers.size() - 1, layers, error);
//        for (int i = layers.size() - 2; i > 0; i--) {
//            transposed = Matrix.transpose(weights.get(i));
//            error = Matrix.multiply(transposed, error);
//            correctError(i, layers, error);
//        }
//
//        return layers.get(layers.size() - 1);
    }

    public void run() {
        Kernel kernel = new Kernel() {
            @Override
            public void run() {
                gid = getGlobalId();
                layer_count = layerCounts[gid];
                layer_size = layer_sizes[layer_count];
                weight_index = weightIndexes[gid];
                bias_index = biasesIndexes[gid];
                epoch = epochs[gid];
                learning_rate = learning_rates[gid];
                fit();
            }
        };
    }
}
