package nn.gpu;

import org.jocl.*;

import static nn.gpu.GPUUtils.readFile;
import static org.jocl.CL.*;
import static org.jocl.CL.clCreateCommandQueueWithProperties;

public class DSigmoid {
    private static cl_context context;
    private static final cl_device_id device;
    private static cl_context_properties contextProperties;
    private static cl_command_queue commandQueue;
    private static cl_kernel kernel;
    private static cl_program program;

    private static final String dSigmoidCode = readFile("src/main/resources/kernels/dSigmoid.cl");

    public static void init() {}


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
                1, new String[]{dSigmoidCode}, null, null);

        // Build the program
        clBuildProgram(program, 0, null, null, null, null);

        // Create the kernel
        kernel = clCreateKernel(program, "dSigmoid", null);
    }

    public static double[] dSigmoid(double[] inputArray) {
        var n = inputArray.length;
        double[] dstArray = new double[n];
        Pointer srcA = Pointer.to(inputArray);
        Pointer dst = Pointer.to(dstArray);

        // Allocate the memory objects for the input- and output data
        cl_mem srcMemA = clCreateBuffer(context,
                CL_MEM_READ_ONLY | CL_MEM_COPY_HOST_PTR,
                (long) Sizeof.cl_double * n, srcA, null);
        cl_mem dstMem = clCreateBuffer(context,
                CL_MEM_READ_WRITE,
                (long) Sizeof.cl_double * n, null, null);

        // Set the arguments for the kernel
        int a = 0;
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(srcMemA));
        clSetKernelArg(kernel, a++, Sizeof.cl_mem, Pointer.to(dstMem));

        // Set the work-item dimensions
        long[] global_work_size = new long[]{n};


        var start = System.nanoTime();
        // Execute the kernel
        clEnqueueNDRangeKernel(commandQueue, kernel, 1, null,
                global_work_size, null, 0, null, null);
        System.out.printf("GPU computation: Time elapsed: %fms\n", (System.nanoTime() - start) / 1000000.0);


        // Read the output data
        clEnqueueReadBuffer(commandQueue, dstMem, CL_TRUE, 0,
                (long) n * Sizeof.cl_double, dst, 0, null, null);

        // Release kernel, program, and memory objects
        clReleaseMemObject(srcMemA);
        clReleaseMemObject(dstMem);
//        clReleaseKernel(kernel);
//        clReleaseProgram(program);
//        clReleaseCommandQueue(commandQueue);
//        clReleaseContext(context);

        return dstArray;
    }
}
