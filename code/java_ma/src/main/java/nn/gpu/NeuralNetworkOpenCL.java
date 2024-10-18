package nn.gpu;

import nn.Matrix;
import nn.NeuralNetwork;
import org.jocl.*;

import java.util.ArrayList;
import java.util.List;

import static nn.gpu.GPUUtils.readFile;
import static org.jocl.CL.*;
import static org.jocl.CL.clReleaseMemObject;

public class NeuralNetworkOpenCL {

    private static final cl_device_id device;
    private static final String gradientCode = readFile("src/main/resources/kernels/neuralNetwork.cl");
    private static cl_context context;
    private static cl_context_properties contextProperties;
    private static cl_command_queue commandQueue;
    private static cl_kernel kernel;
    private static cl_program program;

    static {
        // The platform, device type and device number
        // that will be used
        final int platformIndex = 1; // IMPORTANT: 1 much faster than 0 idk why
        final long deviceType = CL_DEVICE_TYPE_ALL;
        final int deviceIndex = 0;

        // Enable exceptions and subsequently omit error checks in this sample
        CL.setExceptionsEnabled(true);

        // Obtain the number of platforms
        int numPlatformsArray[] = new int[1];
        clGetPlatformIDs(0, null, numPlatformsArray);
        int numPlatforms = numPlatformsArray[0];

        // Obtain a platform ID
        cl_platform_id platforms[] = new cl_platform_id[numPlatforms];
        clGetPlatformIDs(platforms.length, platforms, null);
        cl_platform_id platform = platforms[platformIndex];

        // Initialize the context properties
        contextProperties = new cl_context_properties();
        contextProperties.addProperty(CL_CONTEXT_PLATFORM, platform);

        // Obtain the number of devices for the platform
        int numDevicesArray[] = new int[1];
        clGetDeviceIDs(platform, deviceType, 0, null, numDevicesArray);
        int numDevices = numDevicesArray[0];

        // Obtain a device ID
        cl_device_id devices[] = new cl_device_id[numDevices];
        clGetDeviceIDs(platform, deviceType, numDevices, devices, null);
        device = devices[deviceIndex];

        // Create a context for the selected device
        context = clCreateContext(
                contextProperties, 1, new cl_device_id[]{device},
                null, null, null);

        // Create a command-queue for the selected device
        cl_queue_properties properties = new cl_queue_properties();

        commandQueue = clCreateCommandQueueWithProperties(
                context, device, properties, null);

        // Create the program from the source code
        program = clCreateProgramWithSource(context,
                1, new String[]{gradientCode}, null, null);

        // Build the program
        clBuildProgram(program, 0, null, null, null, null);

        // Create the kernel
        kernel = clCreateKernel(program, "fit", null);
    }


    public static double[] get_1dArray(Matrix input) {
        double[] new_error = new double[input.data.length];
        System.arraycopy(input.data, 0, new_error, 0, input.data.length);
        return new_error;
    }

    /**
     * <code>
     * Matrix *weights[contestantCount][layerCount (index i)];<br>
     * Matrix *biases][contestantCount][layerCount (index i);<br>
     * training data is the same for all contestants<br>
     * int layerSizes[contestantCount][layerCount];<br>
     * int layerCounts[contestantCount];<br>
     * __kernel void fit(
     * <p>
     * __global const double *input,
     * __global const double *target,
     * const int test_count,<br>
     *  __global double *weights,
     *  __global double *biases,<br>
     *  __global const int *layerSizes,
     *  __global const int *layerCounts,
     *  const int contestantCount,
     *  const int *epochs,
     *  __global const double *learning_rates
     *  )<br>
     * </code>
     */
    public static Matrix fit(List<NeuralNetwork> nns, Matrix input, Matrix target, List<Integer> epochs, List<Double> learning_rates) {
        Pointer p_input = Pointer.to(get_1dArray(input));
        Pointer p_target = Pointer.to(get_1dArray(target));
        Pointer p_testCount = Pointer.to(new int[]{get_1dArray(input).length / 4});
        // append all weights and biases to one array
        List<Double> weights = new ArrayList<>();
        List<Double> biases = new ArrayList<>();
        List<Integer> layerSizes = new ArrayList<>();
        List<Integer> layerCounts = new ArrayList<>();
        for (var nn : nns) {
            for (int i = 0; i < nn.layer_sizes.size(); i++) {
                for (var k : get_1dArray(nn.weights.get(i))) {
                    weights.add(k);
                }
                for (var k : get_1dArray(nn.biases.get(i))) {
                    biases.add(k);
                }
                layerSizes.add(nn.layer_sizes.get(i));
            }
            layerCounts.add(nn.layer_sizes.size());
        }
        // convert to arrays
        double[] weightsArray = weights.stream().mapToDouble(d -> d).toArray();
        double[] biasesArray = biases.stream().mapToDouble(d -> d).toArray();
        int[] layerSizesArray = layerSizes.stream().mapToInt(d -> d).toArray();
        int[] layerCountsArray = layerCounts.stream().mapToInt(d -> d).toArray();


        Pointer p_weights = Pointer.to(weightsArray);
        Pointer p_biases = Pointer.to(biasesArray);
        Pointer p_layerSizes = Pointer.to(layerSizesArray);
        Pointer p_layerCounts = Pointer.to(layerCountsArray);
        Pointer p_contestantCount = Pointer.to(new int[]{nns.size()});
        Pointer p_epochs = Pointer.to(epochs.stream().mapToInt(d -> d).toArray());
        Pointer p_learning_rates = Pointer.to(learning_rates.stream().mapToDouble(d -> d).toArray());

        // Allocate the memory objects for the input- and output data
        cl_mem input_mem = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                (long) Sizeof.cl_double * get_1dArray(input).length, p_input, null);
        cl_mem target_mem = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                (long) Sizeof.cl_double * get_1dArray(target).length, p_target, null);
        cl_mem testCount_mem = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                Sizeof.cl_int, p_testCount, null);
        cl_mem weights_mem = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                (long) Sizeof.cl_double * weightsArray.length, p_weights, null);
        cl_mem biases_mem = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                (long) Sizeof.cl_double * biasesArray.length, p_biases, null);
        cl_mem layerSizes_mem = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                (long) Sizeof.cl_int * layerSizesArray.length, p_layerSizes, null);
        cl_mem layerCounts_mem = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                (long) Sizeof.cl_int * layerCountsArray.length, p_layerCounts, null);
        cl_mem contestantCount_mem = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                (long) Sizeof.cl_int, p_contestantCount, null);
        cl_mem epochs_mem = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                (long) Sizeof.cl_int * epochs.size(), p_epochs, null);
        cl_mem learning_rates_mem = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                (long) Sizeof.cl_double * learning_rates.size(), p_learning_rates, null);

        // Set the arguments for the kernel
        int a = 0;
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(input_mem));
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(target_mem));
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(testCount_mem));
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(weights_mem));
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(biases_mem));
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(layerSizes_mem));
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(layerCounts_mem));
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(contestantCount_mem));
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(epochs_mem));
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(learning_rates_mem));


        // Set the work-item dimensions
        long[] global_work_size = new long[]{nns.size()};

//        var start = System.nanoTime();
        clEnqueueNDRangeKernel(commandQueue, kernel, 1, null,
                global_work_size, null, 0, null, null);
//        System.out.printf("GPU computation: Time elapsed: %fms\n", (System.nanoTime() - start) / 1000000.0);


        // Read the output data
        clEnqueueReadBuffer(commandQueue, weights_mem, CL_TRUE, 0,
                (long) Sizeof.cl_double * weightsArray.length, p_weights, 0, null, null);
        clEnqueueReadBuffer(commandQueue, biases_mem, CL_TRUE, 0,
                (long) Sizeof.cl_double * biasesArray.length, p_biases, 0, null, null);

        // Release kernel, program, and memory objects
        clReleaseMemObject(input_mem);
        clReleaseMemObject(target_mem);
        clReleaseMemObject(testCount_mem);
        clReleaseMemObject(weights_mem);
        clReleaseMemObject(biases_mem);
        clReleaseMemObject(layerSizes_mem);
        clReleaseMemObject(layerCounts_mem);
        clReleaseMemObject(contestantCount_mem);
        clReleaseMemObject(epochs_mem);
        clReleaseMemObject(learning_rates_mem);

        return null;
    }
}
