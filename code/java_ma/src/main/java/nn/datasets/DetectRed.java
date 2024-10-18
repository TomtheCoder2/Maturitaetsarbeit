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

public class DetectRed {
    public static void main(String[] args) {
        NeuralNetwork nn = new NeuralNetwork(new ArrayList<>(Arrays.asList(3, 20, 2)), 0.001, 123456);

        var input = new double[][]{
                //red
                {244, 184, 93}, {244, 177, 88}, {248, 242, 143}, {246, 216, 118}, {245, 163, 78}, {243, 111, 46}, {241, 180, 93}, {244, 249, 155}, {243, 196, 100}, {242, 107, 45},
                {185, 81, 33}, {185, 92, 38}, {195, 79, 31}, {200, 74, 26}, {197, 214, 136}, {241, 172, 82}, {239, 74, 26}, {255, 104, 39}, {249, 73, 26}, {229, 43, 28},
                {244, 184, 93}, {244, 177, 88}, {248, 242, 143}, {246, 216, 118}, {245, 163, 78}, {243, 111, 46}, {241, 180, 93}, {244, 249, 155}, {243, 196, 100}, {242, 107, 45},
                // other
                {229, 115, 33}, {228, 116, 33}, {229, 114, 34}, {218, 108, 34}, {224, 105, 35}, {221, 100, 34}, {218, 98, 34}, {223, 102, 35}, {175, 85, 26}, {223, 118, 34},
                {237, 217, 157}, {239, 223, 160}, {240, 223, 161}, {228, 187, 136}, {227, 192, 139}, {206, 154, 114}, {233, 220, 159}, {245, 255, 169}, {239, 228, 163}, {228, 154, 111},
                {210, 42, 27}, {208, 47, 30}, {234, 121, 80}, {213, 110, 68}, {152, 64, 35}, {150, 72, 40}, {168, 79, 42}, {173, 102, 52}, {229, 107, 78}, {179, 66, 41}
        };

        var target = new double[60][];
        // first 30 are {1, 0} and the next 30 are {0, 1}
        for (int i = 0; i < 30; i++) {
            target[i] = new double[]{1, 0};
        }
        for (int i = 30; i < 60; i++) {
            target[i] = new double[]{0, 1};
        }

        System.out.println("length of input: " + input.length);
        System.out.println("length of target: " + target.length);

        TrainingSet ts = new TrainingSet(input, target);
        System.out.println(ts);

//        livePlot(new ArrayList<>(Arrays.asList(new XYSeries("Error"), new XYSeries("Error Derivative"))), "Accuracy");


        nn.fit(ts.tasks, ts.targets, 10000, true, 1, true, 11);
//        readCSVData("C:\\Users\\janwi\\Ev3_summer_2022\\java\\src\\main\\resources\\detect_line_data.csv");
        printNN(nn);
        // print the weights
        System.out.println("Weights:");
        for (int i = 0; i < nn.weights.size(); i++) {
            System.out.println("Layer " + i);
            System.out.println(nn.weights.get(i));
        }
        System.out.println("finished printing");
        System.out.println(nn.predict(new double[]{244.0, 184.0, 93.0}));
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
