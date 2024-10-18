package nn.datasets;


import nn.NeuralNetwork;
import nn.TrainingSet;
import org.jfree.data.xy.XYSeries;

import java.util.*;

import static nn.NeuralNetwork.*;
import static nn.Plot.livePlot;
import static nn.datasets.TurnIter.wheel_diameter;


public class Turn {
    public static boolean stop = false;

    public static void main(String[] args) {
        System.out.println("Turn");
        fit(1000);
    }

    public static void fit(int epochs) {
//        ProgressBar pb = new ProgressBar("Neural Network", epochs);
//        pb.setExtraMessage("Training...");
        var NN = new NeuralNetwork(new ArrayList<>(Arrays.asList(8, 40, 20, 2)), 0.000001, 123456);
//        errorsPerColor = new double[epochs][Y[0].length == 1 ? 2 : Y[0].length];
        List<XYSeries> errorsLeftRight = new ArrayList<>();
        errorsLeftRight.add(new XYSeries("Error"));
        errorsLeftRight.add(new XYSeries("Error Derivative"));
        errorsPerColorGUI = new double[0][0];
        System.out.println("init plot gui");
        accuracySeries = new XYSeries("Accuracy");
        accuracy_AF = livePlot(errorsLeftRight, "Accuracy");
        int lastEpoch = 0;
        // save training data
        List<double[]> input_data = new ArrayList<>();
        List<double[]> target_data = new ArrayList<>();
        for (int i = 0; i < epochs; i++) {
            // one epoch
            // means we set a new angle, then we step through all calculations
            // by first running the normal turn function, that should give us the target
            // then we run the neural network and compare the output with the target
            // then we use the target to set the variables for the iteration

            // variables: time, time_delta, speed_right, speed_left, dist_right, dist_left
            // we always increase the time by 2ms +- 0.5ms
            long time = 0; // we start at zero, this is measured in nanoseconds
            int delta_t = 0; // this is the time difference between the last two measurements
            int speed_right = 0; // this is the speed of the right motor
            int speed_left = 0; // this is the speed of the left motor
            float dist_right = 0; // this is the distance the right motor has traveled
            float dist_left = 0; // this is the distance the left motor has traveled

            // target variables
            // these are later used to calculate the error
            // and used for the next iteration
            int target_speed_right = 0;
            int target_speed_left = 0;

            // utility vars
            stop = false;
            int random_angle = (int) (Math.random() * (360 - (320 - 40)) + 40f);
//            System.out.println("random angle: " + random_angle);
            random_angle = 90;
            var turnIter = new TurnIter(random_angle);

            int c = 0;
            while (!stop) {
                var target = turnIter.turn(c, speed_left, speed_right, dist_left, dist_right);
                input_data.add(new double[]{time, delta_t, speed_left, speed_right, dist_left, dist_right, random_angle, c});
                target_data.add(new double[]{target.first, target.second});
//                if (target.first != turnIter.v_max) {
//                    var output = NN.train(new double[]{time, delta_t, speed_left, speed_right, dist_left, dist_right, random_angle, c}, new double[]{target.first / 1400.0, target.second / 1400f}, true);
//                    output.each(v -> v * 1400);
////                System.out.printf("target: %d, %d\n", target_speed_left, target_speed_right);
////                System.out.println("output: " + output);
//                    // calculate error
//                    var errorLeft = Math.abs(target.first - output.data[0][0]);
//                    var errorRight = Math.abs(target.second - output.data[1][0]);
////                System.out.println("error: " + errorLeft + ", " + errorRight);
////                if (errorLeft > 1 || errorRight > 1) {
//                    errorsThisEpoch += (errorRight + errorLeft) / 2f;
////                }
////                    errorsLeftRight.get(0).add(i, errorsThisEpoch);
////                }
                target_speed_left = target.first;
                target_speed_right = target.second;
//                var output = fit(speed_left, speed_right, dist_left, dist_right, target_speed_left, target_speed_right, i);
                speed_right = target_speed_right;
                speed_left = target_speed_left;
                // update distance and time
                // randomize delta_t
                delta_t = (int) (Math.random() * 1e6 + 1.5e6);
                dist_right += (float) delta_t / 1e9 * (float) speed_right * wheel_diameter * Math.PI / 360f;
                dist_left += (float) delta_t / 1e9 * (float) speed_left * wheel_diameter * Math.PI / 360f;
//                // we add a bit of noise to the distances'
//                dist_right += (float) (Math.random() * 0.1 - 0.05);
//                dist_left += (float) (Math.random() * 0.1 - 0.05);
                time += delta_t;
                c += 1;
//                System.out.printf("time: %fs, delta_t: %fms, speed_right: %d, speed_left: %d, dist_right: %f, dist_left: %f\n",
//                        time / 1e9, delta_t / 1e6, speed_right, speed_left, dist_right, dist_left);
            }
        }
        System.out.println("finished generating training data");
        for (int i = 0; i < epochs; i++) {// now actually train the network
            // shuffle the data
            System.out.println("epoch: " + i);
            System.out.println("samples: " + input_data.toArray().length + ", " + target_data.toArray().length);
            List<Integer> samples = new ArrayList<>();
            for (int j = 0; j < input_data.toArray().length; j++) {
                samples.add(j);
            }
            Collections.shuffle(samples);
            int errorsThisEpoch = 0;
            for (int sampleIndex : samples) {
                System.out.println("sample: " + sampleIndex);
                var input = input_data.get(sampleIndex);
                var target = target_data.get(sampleIndex);
                var output = NN.train(new double[]{input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]}, new double[]{target[0] / 1400.0, target[1] / 1400f}, true);
                output.each(v -> v * 1400);
                var errorLeft = Math.abs(target[0] - output.data[0][0]);
                var errorRight = Math.abs(target[1] - output.data[1][0]);
                errorsThisEpoch += (errorRight + errorLeft) / 2f;
                int skip = 100;
                if (sampleIndex % skip == 0) {
                    errorsLeftRight.get(0).add(sampleIndex, errorsThisEpoch);
//                    if (i != 0) {// calculate derivative
//                        var dev = (errorsThisEpoch - (double) errorsLeftRight.get(0).getY(i / skip - 1));
//                        errorsLeftRight.get(1).add(i, dev);
//                    }
                }
            }
        }
//        pb.close();
    }

    public static class Pair<T, k> {
        public T first;
        public k second;

        public Pair(T first, k second) {
            this.first = first;
            this.second = second;
        }

        @Override
        public String toString() {
            return "Pair{" +
                    "first=" + first +
                    ", second=" + second +
                    '}';
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (!(o instanceof Pair<?, ?> pair)) return false;
            return Objects.equals(first, pair.first) && Objects.equals(second, pair.second);
        }

        @Override
        public int hashCode() {
            return Objects.hash(first, second);
        }
    }

}
