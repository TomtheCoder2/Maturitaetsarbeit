package nn.datasets;

import nn.NeuralNetwork;
import nn.TrainingSet;

import java.io.BufferedReader;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

import static nn.Utility.printNN;

public class DetectLine {
    public static void main(String[] args) {
        NeuralNetwork nn = new NeuralNetwork(new ArrayList<>(Arrays.asList(40, 40, 2)), 0.0001, 123456);

        TrainingSet ts = readCSVData("C:\\Users\\janwi\\Ev3_summer_2022\\java\\src\\main\\resources\\detect_line_data.csv");

        nn.fit_unqual(ts.tasks, ts.targets, 1000, true, 1, true, 11);
//        readCSVData("C:\\Users\\janwi\\Ev3_summer_2022\\java\\src\\main\\resources\\detect_line_data.csv");
        printNN(nn);
        // print the weights
        System.out.println("Weights:");
        for (int i = 0; i < nn.weights.size(); i++) {
            System.out.println("Layer " + i);
            System.out.println(nn.weights.get(i));
        }
        System.out.println(nn.predict(new double[]{44, 52, 44, 52, 44, 52, 44, 52, 45, 49, 46, 38, 45, 38, 46, 20, 46, 20, 45, 8, 46, 8, 46, 5, 46, 5, 46, 5, 46, 5, 46, 5, 47, 5, 48, 6, 48, 6, 49, 12}));
        System.out.println(nn.predict(new double[]{36, 57, 36, 57, 36, 57, 35, 57, 35, 57, 34, 55, 34, 55, 34, 55, 34, 55, 34, 55, 34, 55, 34, 58, 33, 58, 33, 58, 33, 58, 32, 58, 33, 58, 33, 58, 34, 59, 34, 59}));
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
