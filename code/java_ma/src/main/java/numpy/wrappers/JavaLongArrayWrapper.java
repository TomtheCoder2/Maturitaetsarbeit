package numpy.wrappers;

import numpy.BinaryBuffer;
import numpy.IArrayWrapper;
import numpy.Utils;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.lang.reflect.Array;

public class JavaLongArrayWrapper implements IArrayWrapper {
    public static final String D_TYPE = "<i8";
    private long[] data = null;

    public JavaLongArrayWrapper() {

    }

    public JavaLongArrayWrapper(long[] data) {
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
        return data instanceof long[];
    }

    @Override
    public IArrayWrapper createInstance(Object data) {
        return new JavaLongArrayWrapper((long[]) data);
    }

    @Override
    public void serializeInto(ByteArrayOutputStream output) throws IOException {
        for (int i = 0; i < data.length; i++) {
            output.write(Utils.serializeToBuffer(data[i], 8));
        }
    }

    @Override
    public String getDescriptionString() {
        return D_TYPE; // for dtype=np.int64
    }

    @Override
    public Object createWithShape(int[] shape) {
        return Array.newInstance(long.class, shape);
    }

    @Override
    public Object readFromBuffer(BinaryBuffer buffer) {
        return buffer.readLong();
    }

}
