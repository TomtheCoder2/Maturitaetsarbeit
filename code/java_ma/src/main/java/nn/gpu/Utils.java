//package nn.gpu;
//
//import nn.Matrix;
//import org.junit.jupiter.api.Test;
//
//import java.util.Arrays;
//
//import static nn.gpu.Gradient.get_1dArray;
//
//public class Utils {
//    public static double[] dSigmoid(double[] input) {
////        return compute_1Array(input, dSigmoidCode, "dSigmoid");
//        return DSigmoid.dSigmoid(input);
//    }
//
//    @Test
//    public void matrix_test() {
//        Matrix a = new Matrix(3, 3);
//        for (int i = 0; i < a.data.length * a.data[0].length; i++) {
//            a.data[i % a.data.length][i - i % a.data.length] = i;
//        }
//        System.out.println(a);
//    }
//
//    @Test
//    public void test_dsigmoid() {
//        DSigmoid.init();
//        var n = 70000000;
//        double[] a = new double[n];
//        for (int i = 0; i < n; i++) {
//            a[i] = Math.random();
//        }
////        a = new double[]{};
////        List<String> items = Arrays.asList(readFile("src/main/resources/test_data.txt").split("\\s*,\\s*"));
////        a = ArrayUtils.toPrimitive(items.stream().map(Double::parseDouble).toList().toArray(new Double[0]));
//        n = a.length;
//        System.out.printf("length: %d\n", n);
//        long start = System.nanoTime();
//        var gpu_out = dSigmoid(a);
//        long finish = System.nanoTime();
//        long timeElapsed = finish - start;
//        System.out.printf("GPU: Time elapsed: %fms\n", timeElapsed / 1000000.0);
//
//        for (int i = 0; i < 1; i++) {
//            start = System.nanoTime();
//            gpu_out = dSigmoid(a);
//            finish = System.nanoTime();
//            timeElapsed = finish - start;
//            System.out.printf("GPU: Time elapsed: %fms\n", timeElapsed / 1000000.0);
//        }
//
//        start = System.nanoTime();
//        var java_out = new double[n];
//        for (int i = 0; i < n; i++) {
//            java_out[i] = a[i] * (1 - a[i]);
//        }
//        finish = System.nanoTime();
//        timeElapsed = finish - start;
//        System.out.printf("Java: Time elapsed: %fms\n", timeElapsed / 1000000.0);
//        assert (Arrays.equals(gpu_out, java_out));
//    }
//
//    @Test
//    public void gradient_test() {
//        Matrix a = new Matrix(3, 3);
//        for (int i = 0; i < a.data.length * a.data[0].length; i++) {
//            a.data[i / a.data.length][i % a.data.length] = i;
//        }
//        Matrix b = new Matrix(3, 3);
//        for (int i = 0; i < b.data.length * b.data[0].length; i++) {
////            System.out.printf("x: %d, y: %d\n", i % a.data.length, i - i % a.data.length);
//            b.data[i / b.data.length][i % b.data.length] = i;
//        }
//        System.out.println(Arrays.deepToString(Matrix.multiply(a, b).data));
//        a.sigmoid();
//        System.out.println(Arrays.deepToString(a.data));
////        var n = 10;
////        var m = 20;
////        var learning_rate = 0.001;
////        Matrix layer = new Matrix(n, m);
////        Matrix error = new Matrix(n, m);
////        for (int i = 0; i < n; i++) {
////            for (int j = 0; j < m; j++) {
////                layer.data[i][j] = Math.random();
////                error.data[i][j] = Math.random();
////            }
////        }
////
////        Matrix output = gradient(layer, error, learning_rate);
////
//////        System.out.println("Input: " + Arrays.deepToString(layer.data));
////        var start = System.nanoTime();
////        Matrix h_gradient = layer.dsigmoid();
////        h_gradient.multiply(error);
////        h_gradient.multiply(learning_rate);
////        System.out.printf("Java: Time elapsed: %fms\n", (System.nanoTime() - start) / 1000000.0);
////        System.out.println(Arrays.deepToString(h_gradient.data));
////        System.out.println(Arrays.deepToString(output.data));
////        for (int i = 0; i < n; i++) {
////            for (int j = 0; j < m; j++) {
////                assert (output.data[i][j] == h_gradient.data[i][j]);
////            }
////        }
//////        assert (h_gradient.data == output.data);
//    }
//
//    @Test
//    void test_1dArray() {
//        var n = 10;
//        var m = 20;
//        Matrix layer = new Matrix(n, m);
//        for (int i = 0; i < n; i++) {
//            for (int j = 0; j < m; j++) {
//                layer.data[i][j] = Math.random();
//            }
//        }
//        System.out.println(Arrays.deepToString(layer.data));
//        System.out.println(Arrays.toString(get_1dArray(layer)));
//    }
//}
