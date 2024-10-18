package numpy.wrappers;

import numpy.BinaryBuffer;
import numpy.IArrayWrapper;
import numpy.Utils;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.lang.reflect.Array;

public class JavaDoubleArrayWrapper implements IArrayWrapper {
    public static final String D_TYPE = "<f8";
    private double[] data = null;

    public JavaDoubleArrayWrapper() {

    }

    public JavaDoubleArrayWrapper(double[] data) {
        this.data = data;
    }

    @Override
    public int size() {
        return this.data.length;
    }

    @Override
    public boolean isArrayOfArray() {
        return false;
    }

    @Override
    public Object get(int index) {
        return this.data[index];
    }

    @Override
    public boolean supportType(Object data) {
        return data instanceof double[];
    }

    @Override
    public IArrayWrapper createInstance(Object data) {
        return new JavaDoubleArrayWrapper((double[]) data);
    }

    @Override
    public void serializeInto(ByteArrayOutputStream output) throws IOException {
        for (int i = 0; i < data.length; i++) {
            output.write(Utils.serializeToBuffer(Double.doubleToLongBits(data[i]), 8));
        }
    }

    @Override
    public String getDescriptionString() {
        return D_TYPE; // for dtype=np.double
    }

    @Override
    public Object createWithShape(int[] shape) {
        return Array.newInstance(double.class, shape);
    }

    @Override
    public Object readFromBuffer(BinaryBuffer buffer) {
        return buffer.readDouble();
    }
}
