package numpy.wrappers;

import numpy.BinaryBuffer;
import numpy.IArrayWrapper;
import numpy.Utils;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.lang.reflect.Array;

public class JavaShortArrayWrapper implements IArrayWrapper {
    public static final String D_TYPE = "<i2";
    private short[] data = null;

    public JavaShortArrayWrapper() {

    }

    public JavaShortArrayWrapper(short[] data) {
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
        return data instanceof short[];
    }

    @Override
    public IArrayWrapper createInstance(Object data) {
        return new JavaShortArrayWrapper((short[]) data);
    }

    @Override
    public void serializeInto(ByteArrayOutputStream output) throws IOException {
        for (int i = 0; i < data.length; i++) {
            output.write(Utils.serializeToBuffer(data[i], 2));
        }
    }

    @Override
    public String getDescriptionString() {
        return D_TYPE; // for dtype=np.int16
    }

    @Override
    public Object createWithShape(int[] shape) {
        return Array.newInstance(short.class, shape);
    }

    @Override
    public Object readFromBuffer(BinaryBuffer buffer) {
        return buffer.readShort();
    }

}
