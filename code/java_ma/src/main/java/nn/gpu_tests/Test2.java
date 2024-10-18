package nn.gpu_tests;

import static nn.gpu.GPUUtils.compute_2Arrays;

public class Test2 {
    /**
     * translation to java: <br><code>
     * void sampleKernel(int index, double[] a,double[] b, double[] c)
     * {
     * c[index] = a[index] + b[index]; // index: get_global_id(0)
     * }</code>
     */
    private static String programSource =
            "__kernel void " +
                    "sampleKernel(__global const double *a," +
                    "             __global const double *b," +
                    "             __global double *c)" +
                    "{" +
                    "    int gid = get_global_id(0);" +
                    "    c[gid] = a[gid] * b[gid];" +
                    "}";

    public static void main(String[] args) {
        // Create input- and output data
        int n = 1000000;
        double[] srcArrayA = new double[n];
        double[] srcArrayB = new double[n];
        for (int i = 0; i < n; i++) {
            srcArrayA[i] = Math.random();
            srcArrayB[i] = Math.random();
        }

        double[] dstArray = new double[n];
        System.out.println("Start..");
        long start = System.currentTimeMillis();
        for (int i = 0; i < 10; i ++) {
            dstArray = compute_2Arrays(srcArrayA, srcArrayB, programSource);
//            System.out.println(Arrays.toString(dstArray));
        }
        long finish = System.currentTimeMillis();
        long timeElapsed = finish - start;
        System.out.printf("GPU: Time elapsed: %d\n", timeElapsed);


        computeJava(srcArrayA, srcArrayB);

//        // Verify the result
//        boolean passed = true;
//        final double epsilon = 1e-7f;
//        for (int i = 0; i < n; i++) {
//            double x = dstArray[i];
//            double y = srcArrayA[i] * srcArrayB[i];
//            if (x != y) {
//                passed = false;
//                break;
//            }
////            boolean epsilonEqual = Math.abs(x + y) <= epsilon * Math.abs(x);
////            System.out.printf("x: %f, y: %f\n", x, y);
////            if (!epsilonEqual) {
////                passed = false;
////                break;
////            }
//        }
//        System.out.println("Test " + (passed ? "PASSED" : "FAILED"));
//        if (n <= 10) {
//            System.out.println("Result: " + Arrays.toString(dstArray));
//        }
    }

    public static void computeJava(double[] srcArrayA, double[] srcArrayB) {
        var n = srcArrayA.length;
        double[] dstArray = new double[n];
        long start = System.nanoTime();
        for (int i = 0; i < 10; i ++) {
            for (int j = 0; j < n; j++) {
                dstArray[j] = srcArrayA[j] * srcArrayB[j];
            }
        }
        long finish = System.nanoTime();
        long timeElapsed = finish - start;
        System.out.printf("Java: Time elapsed: %dns\n", timeElapsed / 10);
//        System.out.println(Arrays.toString(dstArray));
    }


}
