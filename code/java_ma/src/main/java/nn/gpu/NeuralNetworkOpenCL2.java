package nn.gpu;

import com.aparapi.Range;
import com.aparapi.device.Device;
import com.aparapi.device.OpenCLDevice;
import nn.Matrix;
import nn.NeuralNetwork;

import java.util.List;

import com.aparapi.Kernel;


public class NeuralNetworkOpenCL2 {
    public static void fit(List<NeuralNetwork> nns, Matrix input, Matrix target, List<Integer> epochs, List<Double> learning_rates) {
//        // convert input to array
//        double[][] input_array = new double[input.data.length][input.data[0].length];
//        for (int i = 0; i < input.data.length; i++) {
//            System.arraycopy(input.data[i], 0, input_array[i], 0, input.data[0].length);
//        }
//        double[][] target_array = new double[target.data.length][target.data[0].length];
//        for (int i = 0; i < target.data.length; i++) {
//            System.arraycopy(target.data[i], 0, target_array[i], 0, target.data[0].length);
//        }
//        Kernel kernel = new Kernel() {
//            @Override
//            public void run() {
//                int gid = getGlobalId();
//                nns.get(gid).l_rate = learning_rates.get(gid);
//                nns.get(gid).fit_smaller(input_array, target_array, epochs.get(gid), gid);
//            }
//        };
//        Device device = Device.best();
//        for (Device d: OpenCLDevice.listDevices(Device.TYPE.GPU)){
//            System.out.println(d.getShortDescription());
//            if (d.getShortDescription().contains("NVIDIA")) {
//                device = d;
//                break;
//            }
//            System.out.println(d);
//        }
//        Range RANGE =  device.createRange(nns.size());
//        kernel.execute(RANGE);
//        kernel.dispose();
    }
}
