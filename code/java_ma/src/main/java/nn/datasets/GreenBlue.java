package nn.datasets;

import nn.NeuralNetwork;
import nn.TrainingSet;
import org.jfree.data.xy.XYSeries;

import java.io.BufferedReader;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

import static nn.NeuralNetwork.AF;
import static nn.NeuralNetwork.accuracy_AF;
import static nn.Plot.livePlot;
import static nn.Utility.printNN;

public class GreenBlue {
    public static void main(String[] args) {
        NeuralNetwork nn = new NeuralNetwork(new ArrayList<>(Arrays.asList(3, 5, 2)), 0.01, 123456);

        var input = new double[][]{
                // green
                {3, 15, 9},{6, 30, 16},{6, 31, 15},{6, 27, 13},{8, 39, 18},{6, 36, 12},{9, 40, 18},{8, 40, 19},{8, 40, 17},{7, 39, 17},{6, 42, 20},{8, 42, 20},{13, 58, 19},{7, 41, 6},{4, 29, 5},{6, 39, 7},{17, 74, 25},{5, 22, 11},{4, 15, 8},{4, 10, 7},{2, 7, 4},{2, 9, 4},{5, 17, 10},{4, 19, 10},{6, 32, 6},{5, 36, 6},{6, 32, 5},{6, 28, 14},{5, 24, 12},{4, 19, 9},{2, 12, 7},{5, 12, 8},{4, 17, 9},{6, 25, 6},{3, 25, 4},{1, 20, 3},{1, 19, 2},{0, 17, 2},{8, 37, 11},{4, 29, 12},{1, 22, 7},{1, 21, 6},{4, 39, 15},{19, 84, 33},{6, 42, 6},{3, 27, 4},{1, 21, 3},{3, 21, 3},{6, 23, 12},{4, 22, 11},{2, 22, 9},{2, 21, 9},{3, 23, 9},{4, 27, 12},{7, 34, 18},{8, 47, 8},{7, 48, 8},{7, 49, 8},{8, 43, 11},{5, 35, 14},{7, 36, 17},{5, 32, 5},{6, 31, 6},{9, 40, 10},{10, 50, 20},{7, 26, 13},{5, 23, 10},{4, 25, 8},{5, 24, 11},{4, 23, 11},{2, 23, 10},{6, 31, 13},{8, 38, 13},{9, 49, 7},{11, 57, 10},{5, 26, 13},{4, 17, 9},{4, 23, 4},{4, 24, 4},{6, 30, 7},{6, 30, 13},{3, 23, 10},{1, 16, 6},{5, 22, 12},{8, 34, 8},{4, 33, 5},{8, 55, 8},{9, 55, 9},{10, 53, 11},{10, 51, 14},{9, 39, 19},{6, 34, 16},{3, 25, 11},{9, 41, 19},{10, 47, 15},{8, 42, 7},{7, 41, 6},{5, 33, 6},{5, 32, 5},{6, 31, 16},
                // blue
                {9, 26, 62},{8, 25, 60},{7, 25, 60},{7, 22, 46},{9, 29, 52},{17, 49, 97},{19, 55, 123},{9, 29, 78},{6, 16, 44},{3, 5, 19},{1, 3, 14},{0, 3, 14},{2, 6, 19},{5, 14, 25},{4, 13, 24},{3, 13, 23},{2, 12, 20},{3, 14, 23},{8, 24, 48},{9, 27, 66},{6, 25, 81},{20, 58, 122},{11, 38, 62},{3, 16, 32},{2, 8, 33},{3, 14, 39},{5, 13, 37},{3, 9, 31},{3, 9, 32},{5, 14, 38},{5, 18, 30},{3, 11, 19},{3, 12, 20},{2, 11, 19},{2, 14, 23},{4, 14, 25},{2, 11, 24},{3, 11, 30},{2, 9, 30},{2, 7, 27},{3, 6, 20},{3, 7, 17},{4, 10, 16},{1, 9, 14},{1, 10, 18},{3, 15, 27},{9, 30, 50},{9, 29, 60},{9, 25, 67},{4, 16, 52},{3, 12, 46},{2, 10, 38},{4, 10, 30},{4, 12, 25},{4, 14, 23},{3, 15, 25},{5, 18, 31},{6, 20, 32},{8, 25, 44},{9, 25, 61},{7, 23, 59},{5, 16, 51},{2, 11, 39},{2, 8, 34},{3, 7, 29},{2, 5, 24},{2, 5, 21},{3, 7, 24},{3, 13, 28},{2, 11, 34},{4, 18, 40},{5, 25, 43},{7, 25, 45},{7, 21, 40},{4, 11, 29},{4, 12, 32},{8, 24, 55},{6, 23, 39},{6, 22, 34},{5, 19, 33},{2, 12, 27},{3, 10, 27},{2, 6, 23},{2, 6, 24},{1, 5, 21},{1, 5, 22},{2, 8, 33},{7, 22, 45},{4, 11, 26},{2, 10, 17},{2, 11, 17},{6, 22, 38},{7, 21, 55},{3, 11, 40},{3, 8, 26},{4, 13, 25},{3, 15, 24},{5, 21, 37},{9, 31, 53},{9, 27, 70},};

        var target = new double[200][];
        // first 100 are {1, 0} and the next 100 are {0, 1}
        for (int i = 0; i < 100; i++) {
            target[i] = new double[]{1, 0};
        }
        for (int i = 100; i < 200; i++) {
            target[i] = new double[]{0, 1};
        }

        System.out.println("length of input: " + input.length);
        System.out.println("length of target: " + target.length);

        TrainingSet ts = new TrainingSet(input, target);
        System.out.println(ts);

//        livePlot(new ArrayList<>(Arrays.asList(new XYSeries("Error"), new XYSeries("Error Derivative"))), "Accuracy");


        nn.fit(ts.tasks, ts.targets, 100, true, 1, true, 11);
//        readCSVData("C:\\Users\\janwi\\Ev3_summer_2022\\java\\src\\main\\resources\\detect_line_data.csv");
        printNN(nn);
        // print the weights
        System.out.println("Weights:");
        for (int i = 0; i < nn.weights.size(); i++) {
            System.out.println("Layer " + i);
            System.out.println(nn.weights.get(i));
        }
        System.out.println("finished printing");
        System.out.println(nn.predict(new double[]{9, 26, 62}));
    }

    public static TrainingSet readCSVData(String fileName) {
        ArrayList<ArrayList<Double>> X = new ArrayList<>();
        ArrayList<ArrayList<Double>> Y = new ArrayList<>();
        try {
            List<List<String>> records = new ArrayList<>();
            try (BufferedReader br = new BufferedReader(new FileReader(fileName))) {
                String line;
                while ((line = br.readLine()) != null) {
                    String[] values = line.split("},");
                    records.add(Arrays.asList(values));
                    System.out.println(Arrays.asList(values));
                    System.out.println(values[0].replace("{", "").replace("}", ""));
                    if (line.startsWith("input, output")) {
                        continue;
                    }
                    X.add(new ArrayList<>(Arrays.stream(values[0].replace("{", "").replace("}", "").split(", ")).toList().stream().map(Double::parseDouble).collect(Collectors.toList())));
                    Y.add(new ArrayList<>(Arrays.stream(values[1].replace("{", "").replace("}", "").split(", ")).toList().stream().map(Double::parseDouble).collect(Collectors.toList())));
                }
            }
//            System.out.println(records);
        } catch (Exception e) {
            e.printStackTrace();
        }
        // convert X to an double[][]
        double[][] XArray = new double[X.size()][X.get(0).size()];
        for (int i = 0; i < X.size(); i++) {
            for (int j = 0; j < X.get(i).size(); j++) {
                XArray[i][j] = X.get(i).get(j);
            }
        }
        // convert Y to an double[][]
        double[][] YArray = new double[Y.size()][Y.get(0).size()];
        for (int i = 0; i < Y.size(); i++) {
            for (int j = 0; j < Y.get(i).size(); j++) {
                YArray[i][j] = Y.get(i).get(j);
            }
        }

        System.out.println("X: " + X);
        System.out.println("Y: " + Y);

        return new TrainingSet(XArray, YArray);
    }
}
