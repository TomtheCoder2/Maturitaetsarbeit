package nn;

import numpy.NumpyArray;

import java.util.Arrays;

public class TrainingSet {
    public double[][] tasks;
    public double[][] targets;

    public TrainingSet(double[][] X, double[][] Y) {
        this.tasks = X;
        this.targets = Y;
    }

    public String toString() {
        return "TrainingSet{" +
                "tasks=" + Arrays.deepToString(tasks) +
                ", targets=" + Arrays.deepToString(targets) +
                '}';
    }

    // append another data set at the end
    public void append(TrainingSet other) {
        double[][] newTasks = new double[this.tasks.length + other.tasks.length][];
        double[][] newTargets = new double[this.targets.length + other.targets.length][];
        for (int i = 0; i < this.tasks.length; i++) {
            newTasks[i] = this.tasks[i];
            newTargets[i] = this.targets[i];
        }
        for (int i = 0; i < other.tasks.length; i++) {
            newTasks[this.tasks.length + i] = other.tasks[i];
            newTargets[this.targets.length + i] = other.targets[i];
        }
        this.tasks = newTasks;
        this.targets = newTargets;
    }

    public void writeToFile(String path) {
        // write in the format:
        // first line is X in the format: [[1, 2], [3, 4]]
        // second line is Y in the format: [[1], [0]]
        try {
            java.io.FileWriter myWriter = new java.io.FileWriter(path + ".txt");
            myWriter.write(Arrays.deepToString(this.tasks) + "\n");
            myWriter.write(Arrays.deepToString(this.targets) + "\n");
            myWriter.close();
        } catch (java.io.IOException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
        NumpyArray tasks = new NumpyArray(this.tasks);
        tasks.save(path + "_tasks.npy");

        NumpyArray targets = new NumpyArray(this.targets);
        targets.save(path + "_targets.npy");
    }
}
