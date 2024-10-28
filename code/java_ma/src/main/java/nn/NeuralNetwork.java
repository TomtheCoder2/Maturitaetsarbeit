package nn;//import com.google.gson.Gson;

import com.google.gson.Gson;
import org.jfree.chart.title.TextTitle;
import org.jfree.data.xy.XYSeries;
import org.jfree.ui.ApplicationFrame;

import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;
import java.io.FileWriter;
import java.io.IOException;
import java.util.*;
import java.util.function.Function;

import static java.lang.Math.abs;
import static java.lang.Math.max;
import static nn.Plot.chart;
import static nn.Plot.livePlot;
import static nn.Utility.printNN;
import static nn.Utility.progressPercentage;

/**
 * Class to generate and save Neural Networks with one hidden layer
 */
public class NeuralNetwork {
    public static boolean showGUIColor = true;
    public static double[][] errorsPerColorGUI; // errors while training
    public static XYSeries accuracySeries;
    public static ApplicationFrame AF;
    public static List<XYSeries> seriesList;
    public static ApplicationFrame accuracy_AF;
    public static ArrayList<NeuralNetwork> todoList = new ArrayList<>();
    // activation function (default: sigmoid)
    public Function<Double, Double> activationFunction = x -> 1 / (1 + Math.exp(-x));
    public Function<Double, Double> dActivationFunction = x -> x * (1 - x);
    static KeyListener keyListener = new KeyListener() {
        @Override
        public void keyTyped(KeyEvent e) {
        }

        @Override
        public void keyPressed(KeyEvent e) {
            System.out.println("Key pressed code=" + e.getKeyCode() + ", char=" + e.getKeyChar());
        }

        @Override
        public void keyReleased(KeyEvent e) {
        }
    };
    static int colorCount = 7;

    static {
        errorsPerColorGUI = new double[0][0];
        System.out.println("init plot gui");
        accuracySeries = new XYSeries("Accuracy");
        accuracy_AF = livePlot(List.of(accuracySeries), "Accuracy");
        seriesList = new ArrayList<>();
        seriesList.add(new XYSeries("Training errors"));
        for (int i = 0; i < colorCount; i++) {
            seriesList.add(new XYSeries("Color " + i));
        }
        seriesList.add(new XYSeries("Saves"));
        AF = livePlot(seriesList, "Errors");
    }

    public ArrayList<Matrix> errors = new ArrayList<>(); // errors while training
    public ArrayList<Matrix> accuracies = new ArrayList<>(); // accuracies while training
    public int curr_iter = 0;
    public ArrayList<Matrix> biases = new ArrayList<>(); // bias output layer
    public ArrayList<Matrix> weights = new ArrayList<>(); // weights output layer
    public ArrayList<Integer> layer_sizes;
    public double l_rate = 0.1001; // learning rate
    public boolean showGUI = false;
    public int localIter, localEpochs;
    public double[][] errorsPerColorLocal;

    /**
     * Create a new Neural Network
     *
     * @param layer_sizes the amount of nodes for each n-th layer
     */
    public NeuralNetwork(ArrayList<Integer> layer_sizes) {
        for (int i = 1; i < layer_sizes.size(); i++) {
            weights.add(new Matrix(layer_sizes.get(i), layer_sizes.get(i - 1)));
            biases.add(new Matrix(layer_sizes.get(i), 1));
        }
    }

    /**
     * Create a new Neural Network
     *
     * @param layer_sizes the amount of nodes for each n-th layer
     * @param l_rate      learning rate
     */
    public NeuralNetwork(ArrayList<Integer> layer_sizes, double l_rate, int seed) {
        this.layer_sizes = layer_sizes;
        for (int i = 1; i < layer_sizes.size(); i++) {
            weights.add(new Matrix(layer_sizes.get(i), layer_sizes.get(i - 1), seed));
            biases.add(new Matrix(layer_sizes.get(i), 1, seed));
        }
        this.l_rate = l_rate;
    }

    public static void check(List<Double> values) {
        for (Double value : values) {
            if (Double.isInfinite(value)) {
                throw new RuntimeException("Infinite value");
            }
            if (Double.isNaN(value)) {
                throw new ArithmeticException("NaN size: " + values.size());
            }
        }
    }

    public void deleteWB() {
        weights = new ArrayList<>();
        biases = new ArrayList<>();
    }

    /**
     * Predict a result based on current weights and biases
     */
    public List<Double> predict(double[] X) {
        ArrayList<Matrix> layers = new ArrayList<>();
        layers.add(Matrix.fromArray(X));

        for (int i = 1; i < layer_sizes.size(); i++) {
            layers.add(Matrix.multiply(weights.get(i - 1), layers.get(i - 1)));
            layers.get(i).add(biases.get(i - 1));
            layers.get(i).activate(activationFunction);
        }
        return layers.get(layers.size() - 1).toArray();
    }

    public double predictTime(double[] X) {
        long start = System.nanoTime();
        predict(X);
        long finish = System.nanoTime();
        long timeElapsed = finish - start;
        System.out.println("Elapsed time (ms): " + (float) timeElapsed / 1000000);
        System.out.println(timeElapsed);
        int totalNodes = layer_sizes.get(0);
        for (int i = 1; i < layer_sizes.size(); i++) {
            totalNodes += layer_sizes.get(i) * layer_sizes.get(i - 1);
            totalNodes += layer_sizes.get(i);
        }
        System.out.println(totalNodes);
        System.out.println((float) timeElapsed / 1000000 / totalNodes);
        return timeElapsed;
    }

    /**
     * One training iteration
     */
    public Matrix train(double[] X, double[] Y, boolean correct_error) {
        // predict the output
        // convert the input to a nn.Matrix
        ArrayList<Matrix> outputs = new ArrayList<>();
        outputs.add(Matrix.fromArray(X));

        for (int i = 1; i < layer_sizes.size(); i++) {
//            outputs.get(i - 1).print();
            weights.get(i - 1).checkNaN();
            outputs.get(i - 1).checkNaN();
            outputs.add(Matrix.multiply(weights.get(i - 1), outputs.get(i - 1)));
//            outputs.get(i).print();
//            System.out.println("i: " + i);
            outputs.get(i).checkNaN();
            outputs.get(i).add(biases.get(i - 1));
            outputs.get(i).activate(activationFunction);
//            System.out.println("layer: " + i);
//            System.out.println(layers.get(i));
        }
//        System.out.println("output: " + layers.get(layers.size() - 1));

        // error detection and correction
        Matrix target = Matrix.fromArray(Y);

        // calculate the error between the output and the correct output (target)
        Matrix error = Matrix.subtract(target, outputs.get(outputs.size() - 1));

        Matrix transposed;
//        System.out.println("error: " + error);

        // use the derivative sigmoid function to correct the error
        if (correct_error) {
            correctError(outputs.size() - 1, outputs, error);
            for (int i = outputs.size() - 2; i > 0; i--) {
                transposed = Matrix.transpose(weights.get(i));
                error = Matrix.multiply(transposed, error);
                correctError(i, outputs, error);
            }
        }

        return outputs.get(outputs.size() - 1);
    }

    private void correctError(int i, ArrayList<Matrix> outputs, Matrix error) {
        Matrix h_gradient = outputs.get(i).dActivate(dActivationFunction);
        h_gradient.multiply(error);
        h_gradient.multiply(l_rate);

        Matrix wih_delta = Matrix.multiply(h_gradient, Matrix.transpose(outputs.get(i - 1)));
//        System.out.println(wih_delta);
//        System.out.println("wih_delta: " + wih_delta);

        weights.get(i - 1).add(wih_delta);
        biases.get(i - 1).add(h_gradient);
//        System.out.println("weights: " + weights.get(i - 1));
    }

    /**
     * Train the Neural Network
     *
     * @param X list of samples (training set)
     * @param Y solutions of the training set X
     */


    public Pair<Double, double[]> fit(double[][] X, double[][] Y, int epochs, boolean gui, int iter, boolean correct_error, int error_averaging_count) {
        errorsPerColorGUI = new double[0][0];
        showGUI = gui;
        localIter = iter;
        if (epochs < 10) {
            epochs = 10;
        }
        localEpochs = epochs;
        while (!todoList.contains(this)) {// sync issues??
            try {
                todoList.add(this);
            } catch (Exception e) {
                System.out.println("todo list: Error: " + e.getMessage());
            }
        }
        errorsPerColorLocal = new double[epochs][Y[0].length];
        var test_count = X.length;
        System.out.println("test_count: " + test_count);
        double cur_accuracy = 0;
        if (showGUI) {
            resetGui(iter, epochs, Y);
        }
        int i;
        for (i = 0; i < (l_rate == 0.0 ? 10 : epochs); i++) {
            curr_iter = i;
            if (showGUI) {
//                progressPercentage("Neural Network #" + iter, i, epochs);
            }
            List<Integer> samples = new ArrayList<>();
            for (int j = 0; j < X.length; j++) {
                samples.add(j);
            }

            Collections.shuffle(samples);
            int errorsThisEpoch = 0;
            for (int sampleIndex : samples) {
//                System.out.println("input: " + Arrays.toString(X[sampleIndex]));
//                System.out.println("target: " + Arrays.toString(Y[sampleIndex]));
                Matrix output = train(X[sampleIndex], Y[sampleIndex], correct_error);
//                System.out.println("output: " + output.toArray());
                if (output.toArray().stream()
                        .filter(Objects::nonNull)  // Avoid null values
                        .mapToDouble(Double::doubleValue)
                        .anyMatch(Double::isNaN)) {
                    System.out.println("output: " + output.toArray());
                    throw new ArithmeticException("NaN");
                }
                // check result
                int targetIndex = 0;
//                System.out.println("target: " + Arrays.toString(Y[sampleIndex]));
                for (int k = 0; k < Y[sampleIndex].length; k++) {
                    if (Y[sampleIndex][k] == 1) {
                        targetIndex = k;
                    }
                }
                List<Double> errorList = output.toArray();
                if (errorList.indexOf(Collections.max(errorList)) != targetIndex) {
//                        System.out.println("SampleCase #" + sampleIndex + " Error: " + Arrays.toString(X[sampleIndex]) + " target: " + targetIndex + " output: " + errorList.indexOf(Collections.max(errorList)) + " Prediction: " + output.toArray());
                    errorsThisEpoch++;
                    // check if errorsPerColorLocal[i][targetIndex] = NaN
                    errorsPerColorLocal[i][targetIndex]++;
//                    System.out.println("errorsPerColorLocal[i][targetIndex]: " + errorsPerColorLocal[i][targetIndex] + " i: " + i + " targetIndex: " + targetIndex);
                    if (showGUI) {
                        errorsPerColorGUI = errorsPerColorLocal;
                    }
                }
                // calculate accuracy
                Matrix accuracy = Matrix.subtract(Matrix.fromArray(Y[sampleIndex]), output);
                accuracies.add(accuracy);
            }

            float sum = 0;
            for (int j = 0; j < test_count; j++) {
                sum += abs(accuracies.get(i + j).data[0][0]);
            }
            cur_accuracy = sum / test_count;
//            System.out.println("in this epoch: error: " + errorsThisEpoch);
            if (i % 200 == 0) {
                System.out.println("in this epoch: error: " + errorsThisEpoch);
            }
            if (showGUI) {
                for (int j = 0; j < errorsPerColorGUI[i].length; j++) {
                    errorsPerColorGUI[i][j] = errorsPerColorGUI[i][j] / test_count * 100;
                }
                this.errors.add(Matrix.fromArray(new double[]{(float) errorsThisEpoch / Y[0].length / test_count * 100}));
                // accuracy
                accuracySeries.add(i, sum / test_count);
                // avg error
                sum = 0;
                for (int j = 0; j < errorsPerColorGUI[i].length; j++) {
                    if (j < seriesList.size() - 1) {
                        seriesList.get(j + 1).addOrUpdate(i, errorsPerColorGUI[i][j]);
                    }
                    sum += (float) errorsPerColorGUI[i][j];
                }
//                System.out.println("sum: " + sum);
//                System.out.println("errorsPerColorGUI[i]: " + Arrays.toString(errorsPerColorGUI[i]) + " i: " + i);
//                System.out.println("errorsPerColorLocal[i]: " + Arrays.toString(errorsPerColorLocal[i]) + " i: " + i);
                seriesList.get(0).add(i, sum / (float) Y[0].length);
            }
            for (int j = 0; j < errorsPerColorLocal[i].length; j++) {
                errorsPerColorLocal[i][j] = errorsPerColorLocal[i][j] / test_count * 100;
            }
        }
//        pb.close();
        double[] sum = new double[errorsPerColorLocal[i - 1].length];
        int counter = 0;
        for (int j = i - 1; j > i - error_averaging_count - 1; j--) {
            counter++;
            for (int c = 0; c < errorsPerColorLocal[i - 1].length; c++) {
                sum[c] += errorsPerColorLocal[j][c];
            }
        }
        for (int j = 0; j < sum.length; j++) {
            sum[j] = sum[j] / counter;
        }

        todoList.remove(this);
        if (showGUI) {
            if (!todoList.isEmpty()) {
                NeuralNetwork next = todoList.get(0);
                next.showGUI = true;
                errorsPerColorGUI = next.errorsPerColorLocal;
                next.resetGui(next.localIter, next.localEpochs, Y);
            }
        }
        progressPercentage("Neural Networks", 100 - todoList.size(), 100);
        System.out.println("finish fit() #" + iter + " total: " + todoList.size());
        showGUI = false;
        return new Pair<>(cur_accuracy, sum);
    }

    public Pair<Double, double[]> fit_unqual(double[][] X, double[][] Y, int epochs, boolean gui, int iter, boolean correct_error, int error_averaging_count) {
        errorsPerColorGUI = new double[0][0];
        showGUI = gui;
        localIter = iter;
        if (epochs < 10) {
            epochs = 10;
        }
        localEpochs = epochs;
        while (!todoList.contains(this)) {// sync issues??
            try {
                todoList.add(this);
            } catch (Exception e) {
                System.out.println("Error: " + e.getMessage());
            }
        }
        errorsPerColorLocal = new double[epochs][Y[0].length];
        var test_count = X.length / Y[0].length;
        double cur_accuracy = 0;
        if (showGUI) {
            resetGui(iter, epochs, Y);
        }
        int i;
        // create a list of all samples that have a [1, 0] target
        List<Integer> samples_positive = new ArrayList<>();
        for (int j = 0; j < X.length; j++) {
            if (Y[j][0] == 1) {
                samples_positive.add(j);
            }
        }
        // same for [0, 1]
        List<Integer> samples_negative = new ArrayList<>();
        for (int j = 0; j < X.length; j++) {
            if (Y[j][1] == 1) {
                samples_negative.add(j);
            }
        }
        for (i = 0; i < (l_rate == 0.0 ? 10 : epochs); i++) {
            curr_iter = i;
            if (showGUI) {
//                progressPercentage("Neural Network #" + iter, i, epochs);
            }
            List<Integer> samples = new ArrayList<>();
            for (int j = 0; j < X.length; j++) {
                samples.add(j);
            }

            Collections.shuffle(samples_positive);
            Collections.shuffle(samples_negative);
            int errorsThisEpoch = 0;
            for (int j = 0; j < max(samples_negative.size(), samples_positive.size()); j++) {
                // positive
                int sampleIndex = samples_positive.get(j % samples_positive.size());
                Matrix output = train(X[sampleIndex], Y[sampleIndex], correct_error);
                //System.out.println(output.toArray());
                // check result
                int targetIndex = 0;
//                System.out.println("target: " + Arrays.toString(Y[sampleIndex]));
                for (int k = 0; k < Y[sampleIndex].length; k++) {
                    if (Y[sampleIndex][k] == 1) {
                        targetIndex = k;
                    }
                }
                List<Double> errorList = output.toArray();
                if (errorList.indexOf(Collections.max(errorList)) != targetIndex) {
//                        System.out.println("SampleCase #" + sampleIndex + " Error: " + Arrays.toString(X[sampleIndex]) + " target: " + targetIndex + " output: " + errorList.indexOf(Collections.max(errorList)) + " Prediction: " + output.toArray());
                    errorsThisEpoch++;
                    errorsPerColorLocal[i][targetIndex]++;
                    if (showGUI) {
                        errorsPerColorGUI = errorsPerColorLocal;
                    }
                }
                // calculate accuracy
                Matrix accuracy = Matrix.subtract(Matrix.fromArray(Y[sampleIndex]), output);
                accuracies.add(accuracy);

                // negative
                sampleIndex = samples_negative.get(j % samples_negative.size());
                output = train(X[sampleIndex], Y[sampleIndex], correct_error);
                //System.out.println(output.toArray());
                // check result
                targetIndex = 0;
//                System.out.println("target: " + Arrays.toString(Y[sampleIndex]));
                for (int k = 0; k < Y[sampleIndex].length; k++) {
                    if (Y[sampleIndex][k] == 1) {
                        targetIndex = k;
                    }
                }
                errorList = output.toArray();
                if (errorList.indexOf(Collections.max(errorList)) != targetIndex) {
//                        System.out.println("SampleCase #" + sampleIndex + " Error: " + Arrays.toString(X[sampleIndex]) + " target: " + targetIndex + " output: " + errorList.indexOf(Collections.max(errorList)) + " Prediction: " + output.toArray());
                    errorsThisEpoch++;
                    errorsPerColorLocal[i][targetIndex]++;
                    if (showGUI) {
                        errorsPerColorGUI = errorsPerColorLocal;
                    }
                }
                // calculate accuracy
                accuracy = Matrix.subtract(Matrix.fromArray(Y[sampleIndex]), output);
                accuracies.add(accuracy);
            }

            float sum = 0;
            for (int j = 0; j < test_count; j++) {
                sum += abs(accuracies.get(i + j).data[0][0]);
            }
            cur_accuracy = sum / test_count;
            if (showGUI) {
                try {
                    for (int j = 0; j < errorsPerColorGUI[i].length; j++) {
                        errorsPerColorGUI[i][j] = errorsPerColorGUI[i][j] / test_count * 100;
                    }
                    this.errors.add(Matrix.fromArray(new double[]{(float) errorsThisEpoch / Y[0].length / test_count * 100}));
                    // accuracy
//                accuracySeries.add(i, sum / test_count);
                    // avg error
                    sum = 0;
                    for (int j = 0; j < errorsPerColorGUI[i].length; j++) {
                        seriesList.get(j + 1).addOrUpdate(i, errorsPerColorGUI[i][j]);
                        sum += errorsPerColorGUI[i][j];
                    }
                    seriesList.get(0).add(i, sum / (float) Y[0].length);
                } catch (Exception e) {
                    System.out.println("Error: " + e.getMessage());
                }
            }
            for (int j = 0; j < errorsPerColorLocal[i].length; j++) {
                errorsPerColorLocal[i][j] = errorsPerColorLocal[i][j] / test_count * 100;
            }
        }
//        pb.close();
        double[] sum = new double[errorsPerColorLocal[i - 1].length];
        int counter = 0;
        for (int j = i - 1; j > i - error_averaging_count - 1; j--) {
            counter++;
            for (int c = 0; c < errorsPerColorLocal[i - 1].length; c++) {
                sum[c] += errorsPerColorLocal[j][c];
            }
        }
        for (int j = 0; j < sum.length; j++) {
            sum[j] = sum[j] / counter;
        }

        todoList.remove(this);
        if (showGUI) {
            if (!todoList.isEmpty()) {
                NeuralNetwork next = todoList.get(0);
                next.showGUI = true;
                errorsPerColorGUI = next.errorsPerColorLocal;
                next.resetGui(next.localIter, next.localEpochs, Y);
            }
        }
        progressPercentage("Neural Networks", 100 - todoList.size(), 100);
//        System.out.println("finish fit() #" + iter + " total: " + todoList.size());
        showGUI = false;
        return new Pair<>(cur_accuracy, sum);
    }

    public void resetGui(int iter, int epochs, double[][] Y) {
        chart.clearSubtitles();
//        changeEpochs(epochs);
        chart.addSubtitle(new TextTitle("layer Sizes: " + this.layer_sizes.toString() + " epochs: " + this.localEpochs + " iterations: " + this.localIter + " learning rate: " + this.l_rate + " current Gen: " + "undefined"));
        for (XYSeries s : seriesList) {
            s.clear();
        }
        errorsPerColorGUI = errorsPerColorLocal;
        for (int i = 0; i < errorsPerColorGUI.length; i++) {
            try {
                for (int j = 0; j < errorsPerColorGUI[i].length; j++) {
                    seriesList.get(j + 1).addOrUpdate(i, errorsPerColorGUI[i][j]);
                }
            } catch (Exception e) {
                break;
            }
        }
//        errorsPerColorGUI = new double[epochs][Y[0].length];
//        errorsPerColorGUI = errorsPerColorLocal.clone();
        var curr_nn = this;

        accuracySeries.clear();
        accuracySeries.add(0, 0);

        AF.removeKeyListener(keyListener);
        keyListener = new KeyListener() {
            @Override
            public void keyTyped(KeyEvent e) {
            }

            @Override
            public void keyPressed(KeyEvent e) {
                System.out.println("Key pressed code=" + e.getKeyCode() + ", char=" + e.getKeyChar());
                if ((e.getKeyCode() >= 48 && e.getKeyCode() <= 57) || e.getKeyCode() == 13) {
                    printNN(curr_nn, (String.valueOf(e.getKeyChar()).matches("\\d+") ? String.valueOf(e.getKeyChar()) : null));
                    seriesList.get(errorsPerColorGUI[0].length).addOrUpdate(curr_nn.curr_iter - 0.001, 0);
                    seriesList.get(errorsPerColorGUI[0].length).addOrUpdate(curr_nn.curr_iter, 100);
                    seriesList.get(errorsPerColorGUI[0].length).addOrUpdate(curr_nn.curr_iter + 0.001, 0);
                }
            }

            @Override
            public void keyReleased(KeyEvent e) {
            }
        };
        AF.addKeyListener(keyListener);
        AF.setTitle("Neural Network " + iter);
    }

    public void parseToJson(String path) throws IOException {
        Gson gson = new Gson();
        FileWriter writer = new FileWriter(path);
        gson.toJson(this, writer);
        writer.flush(); // flush data to file   <---
        writer.close(); // close write          <---
        System.out.println(gson.toJson(this));
    }

//    public static nn.NeuralNetwork getNNFromJson(String path) throws FileNotFoundException {
//        Gson gson = new Gson();
//        return gson.fromJson(new FileReader(path), nn.NeuralNetwork.class);
//    }
}
