package nn.datasets;

import nn.NeuralNetwork;

import java.util.ArrayList;
import java.util.Arrays;

import static nn.NeuralNetwork.AF;
import static nn.NeuralNetwork.accuracy_AF;

public class NNTest {
    public static void main(String[] args) {
        NeuralNetwork nn = new NeuralNetwork(new ArrayList<>(Arrays.asList(4, 74, 89, 7)), 0.000056, 1);
//        weights_list = vec![vec![vec![0.0141129, 0.0359112, 0.0215627, 0.0859542, ],vec![0.0158632, 0.0395061, 0.0243927, 0.0808768, ],],vec![vec![1.09342, 0.965217, ],vec![1.08694, 0.958383, ],vec![1.08917, 0.95925, ],],vec![vec![-0.577363, -0.571786, -0.582449, ],vec![0.486081, 0.470242, 0.473425, ],vec![-0.616374, -0.609917, -0.608424, ],vec![-0.572523, -0.561484, -0.557633, ],vec![-0.610435, -0.607933, -0.612662, ],vec![-0.607315, -0.619278, -0.613357, ],vec![-0.419159, -0.411442, -0.420687, ],],],
//biases_list = vec![vec![vec![0.00381524, ],vec![-0.00857864, ],],vec![vec![1.10784, ],vec![1.10598, ],vec![1.11095, ],],vec![vec![-0.825543, ],vec![0.380865, ],vec![-0.907462, ],vec![-0.740244, ],vec![-0.851197, ],vec![-0.904746, ],vec![-0.586667, ],],]
//for i in 0 .. biases_list.len() {
//    weights.push(Matrix::from_2d_array(weights_list[i].clone()));
//    biases.push(Matrix::from_2d_array(biases_list[i].clone()));
//}
//nn.weights = weights;
//nn.biases = biases;
//weights = vec![];
//biases = vec![];
        AF.setVisible(false);
        AF.dispose();
        accuracy_AF.setVisible(false);
        accuracy_AF.dispose();

        // print network for c
        for (int i = 0; i < nn.weights.size(); i++) {
            System.out.print("double a" + i + "[] = {");
            for (int j = 0; j < nn.weights.get(i).rows; j++) {
                for (int k = 0; k < nn.weights.get(i).cols; k++) {
                    System.out.print(nn.weights.get(i).data[j * nn.weights.get(i).cols + k] + ", ");
                }
            }
            System.out.println("};");
            System.out.print("weights[" + i + "] = init_matrix_from_array(" + nn.weights.get(i).rows + ", " + nn.weights.get(i).cols + ", a" + i + ");\n");

            System.out.print("double b" + i + "[] = {");
            for (int j = 0; j < nn.biases.get(i).rows; j++) {
                for (int k = 0; k < nn.biases.get(i).cols; k++) {
                    System.out.print(nn.biases.get(i).data[j * nn.biases.get(i).cols + k] + ", ");
                }
            }
            System.out.println("};");
            System.out.print("biases[" + i + "] = init_matrix_from_array(" + nn.biases.get(i).rows + ", " + nn.biases.get(i).cols + ", b" + i + ");\n");
        }

//        nn.weights = new ArrayList<>();
//        nn.biases = new ArrayList<>();
//// layer 0
//        nn.weights.add(Matrix.from_2d_array(new double[][]{new double[]{-0.00643209, -0.00666802, -0.00876129, -0.00448669,}, new double[]{-0.00357934, -0.00264526, -0.00560686, -0.00911769,},}));
//        nn.biases.add(Matrix.from_2d_array(new double[][]{new double[]{-0.00643209,}, new double[]{-0.00666802,},}));
//// layer 1
//        nn.weights.add(Matrix.from_2d_array(new double[][]{new double[]{-0.00643209, -0.00666802,}, new double[]{-0.00876129, -0.00448669,}, new double[]{-0.00357934, -0.00264526,},}));
//        nn.biases.add(Matrix.from_2d_array(new double[][]{new double[]{-0.00643209,}, new double[]{-0.00666802,}, new double[]{-0.00876129,},}));
//// layer 2
//        nn.weights.add(Matrix.from_2d_array(new double[][]{new double[]{-0.00643209, -0.00666802, -0.00876129,}, new double[]{-0.00448669, -0.00357934, -0.00264526,}, new double[]{-0.00560686, -0.00911769, 0.00295308,}, new double[]{-0.00940665, 0.00310837, 0.000117138,}, new double[]{-0.0062383, -0.00883806, 0.0027507,}, new double[]{-0.00175881, -0.00438953, -0.00943896,}, new double[]{0.00265616, -0.00219567, 0.00152903,},}));
//        nn.biases.add(Matrix.from_2d_array(new double[][]{new double[]{-0.00643209,}, new double[]{-0.00666802,}, new double[]{-0.00876129,}, new double[]{-0.00448669,}, new double[]{-0.00357934,}, new double[]{-0.00264526,}, new double[]{-0.00560686,},}));
//        nn.biases.add(Matrix.from_2d_array(new double[][]{new double[]{0.00156741,}, new double[]{0.00948496,}, new double[]{0.00391123,}, new double[]{-0.00677729,}, new double[]{0.00828283,}, new double[]{0.00109598,}, new double[]{0.00567805,},}));
//        nn.train(new double[]{17, 35, 27, 81, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{16, 34, 27, 80, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{17, 34, 27, 81, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{17, 36, 28, 83, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{16, 35, 27, 81, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{17, 34, 27, 82, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{17, 35, 27, 82, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{17, 35, 27, 84, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{19, 39, 30, 84, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{16, 35, 39, 124, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{17, 35, 27, 81, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{16, 34, 26, 80, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{14, 32, 24, 78, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{19, 38, 28, 88, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{29, 49, 35, 115, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{16, 33, 26, 80, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{16, 33, 25, 80, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{91, 162, 73, 213, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{67, 126, 83, 237, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        nn.train(new double[]{17, 35, 26, 83, }, new double[]{1, 0, 0, 0, 0, 0, 0, });
//        System.out.println(nn.predict(new double[]{17, 35, 27, 81}));
    }
}
