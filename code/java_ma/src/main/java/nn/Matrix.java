package nn;


import java.util.ArrayList;
import java.util.List;
import java.util.Random;
import java.util.function.Function;

/**
 * Create, save and operate with Matrices
 */
public class Matrix {
    public double[][] data;
    public int rows, cols;

    /**
     * Crate a new nn.Matrix
     *
     * @param rows amount of rows
     * @param cols amount of columns
     * @param seed random seed
     */
    public Matrix(int rows, int cols, int seed) {
        data = new double[rows][cols];
        this.rows = rows;
        this.cols = cols;
        Random r = new Random(seed);
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (seed != 0) {
                    data[i][j] = r.nextDouble() * 2 - 1;
                } else {
                    data[i][j] = 0;
                }
//                data[i][j] = Math.random() * 2 - 1;
//                System.out.println(data[i][j]);
            }
        }
    }

    /**
     * Crate a new nn.Matrix
     *
     * @param rows amount of rows
     * @param cols amount of columns
     */
    public Matrix(int rows, int cols) {
        data = new double[rows][cols];
        this.rows = rows;
        this.cols = cols;
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                data[i][j] = 0;
//                System.out.print(data[i][j] + " ");
            }
        }
    }

    /**
     * subtract nn.Matrix b from nn.Matrix a
     */
    public static Matrix subtract(Matrix a, Matrix b) {
        Matrix temp = new Matrix(a.rows, a.cols);
        for (int i = 0; i < a.rows; i++) {
            for (int j = 0; j < a.cols; j++) {
                temp.data[i][j] = a.data[i][j] - b.data[i][j];
            }
        }
        return temp;
    }

    /**
     * Mirror the nn.Matrix
     */
    public static Matrix transpose(Matrix a) {
//        System.out.printf("cols: %d, row: %d\n", a.cols, a.rows);
        Matrix temp = new Matrix(a.cols, a.rows);
        for (int i = 0; i < a.rows; i++) {
            for (int j = 0; j < a.cols; j++) {
                temp.data[j][i] = a.data[i][j];
            }
        }
        return temp;
    }

    /*
     * Multiply two Matrices
     * */
    public static Matrix multiply(Matrix a, Matrix b) {
        // check if the columns of the first nn.Matrix are equal to the rows of the second nn.Matrix
        if (a.cols != b.rows) {
            throw new ArithmeticException("Columns of A must match rows of B");
        }
        Matrix temp = new Matrix(a.rows, b.cols);
        for (int i = 0; i < temp.rows; i++) {
            for (int j = 0; j < temp.cols; j++) {
                double sum = 0;
                for (int k = 0; k < a.cols; k++) {
                    sum += a.data[i][k] * b.data[k][j];
//                    if (Double.isNaN(a.data[i][k])) {
//                        throw new ArithmeticException("NaN a: i: " + i + ", k: " + k);
//                    }
//                    if (Double.isNaN(b.data[k][j])) {
//                        throw new ArithmeticException("NaN b: i: " + i + ", k: " + k);
//                    }
                }
                temp.data[i][j] = sum;
            }
        }
        return temp;
    }

    /**
     * Create nn.Matrix from a list with doubles
     */
    public static Matrix fromArray(double[] x) {
        Matrix temp = new Matrix(x.length, 1);
        for (int i = 0; i < x.length; i++) {
            temp.data[i][0] = x[i];
        }
        return temp;
    }

    public static Matrix fromArray(Double[] x) {
        Matrix temp = new Matrix(x.length, 1);
        for (int i = 0; i < x.length; i++) {
            temp.data[i][0] = x[i];
        }
        return temp;
    }

    public void add(double scaler) {
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                this.data[i][j] += scaler;
            }
        }
    }

    /**
     * Add a nn.Matrix to the current nn.Matrix
     */
    public void add(Matrix m) {
//        System.out.println("nn.Matrix A: " + rows + "x" + cols);
//        System.out.println("nn.Matrix B: " + m.rows + "x" + m.cols);
        if (cols != m.cols || rows != m.rows) {
            System.out.println("Shape Mismatch");
            System.out.println("nn.Matrix A: " + rows + "x" + cols);
            System.out.println("nn.Matrix B: " + m.rows + "x" + m.cols);
            return;
        }

        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                this.data[i][j] += m.data[i][j];
            }
        }
    }

    /**
     * Multiply itself by a matrix
     */
    public void multiply(Matrix a) {
        for (int i = 0; i < a.rows; i++) {
            for (int j = 0; j < a.cols; j++) {
                this.data[i][j] *= a.data[i][j];
            }
        }
    }

    /**
     * Multiply each number by a factor a
     */
    public void multiply(double a) {
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                this.data[i][j] *= a;
            }
        }
    }

    /**
     * Sigmoid function
     */
    public void sigmoid() {
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                this.data[i][j] = 1 / (1 + Math.exp(-this.data[i][j]));
            }
        }
    }

    public void activate(Function<Double, Double> f) {
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                this.data[i][j] = f.apply(this.data[i][j]);
            }
        }
    }

    public Matrix dActivate(Function<Double, Double> df) {
        Matrix temp = new Matrix(rows, cols);
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                temp.data[i][j] = df.apply(this.data[i][j]);
            }
        }
        return temp;
    }

    /**
     * Sigmoid function with derivative
     */
    public Matrix dsigmoid() {
        Matrix temp = new Matrix(rows, cols);
        if (cols == 10) {
            temp = fromArray(
//                    dSigmoid(
                    this.toArray().toArray(new Double[0])
//
            );
            //System.out.println(Arrays.toString(ArrayUtils.toPrimitive(this.toArray().toArray(new Double[0]))));
        } else {
            for (int i = 0; i < rows; i++) {
                for (int j = 0; j < cols; j++) {
                    temp.data[i][j] = this.data[i][j] * (1 - this.data[i][j]);
                }
//            temp.data[i] = dSigmoid(this.data[i]);
//            System.out.printf("check: %s, temp: %s, diff: %f\n", Arrays.toString(check.data[i]), Arrays.toString(temp.data[i]), check.data[i][0] - temp.data[i][0]);
//            temp.data[i] = check.data[i];
//            System.out.println(check.data[i] == temp.data[i]);
//            System.out.println(Arrays.toString(data[i]));
            }
        }
        return temp;
    }

    /**
     * Parse a nn.Matrix to an Array
     */
    public List<Double> toArray() {
        List<Double> temp = new ArrayList<Double>();

        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                temp.add(data[i][j]);
            }
        }
        return temp;
    }

    public void print() {
        System.out.println("nn.Matrix: " + rows + "x" + cols);
        for (int i = 0; i < rows; i++) {
            System.out.print("| ");
            for (int j = 0; j < cols; j++) {
                System.out.print(data[i][j] + ", ");
            }
            System.out.print("|");
        }
    }

    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append("Matrix: " + rows + "x" + cols + "\n");
        for (int i = 0; i < rows; i++) {
            sb.append("| ");
            for (int j = 0; j < cols; j++) {
                sb.append(data[i][j] + ", ");
            }
            sb.append("|");
            if (i != rows - 1) {
                sb.append("\n");
            }
        }
        return sb.toString();
    }

    public void eachRow(Function<Double, Double> f) {
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                data[i][j] = f.apply(data[i][j]);
            }
        }
    }

    public void each(Function<Double, Double> f) {
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                data[i][j] = f.apply(data[i][j]);
            }
        }
    }

    public void checkNaN() {
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (Double.isNaN(data[i][j])) {
                    throw new ArithmeticException("NaN: i: " + i + ", j: " + j);
                }
            }
        }
    }
}