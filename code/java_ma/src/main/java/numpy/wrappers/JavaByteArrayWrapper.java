package numpy.wrappers;

import numpy.BinaryBuffer;
import numpy.IArrayWrapper;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.lang.reflect.Array;

public class JavaByteArrayWrapper implements IArrayWrapper {
    public static final String D_TYPE = "|i1";
    private byte[] data = null;

    public JavaByteArrayWrapper() {

    }

    public JavaByteArrayWrapper(byte[] data) {
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
        return data instanceof byte[];
    }

    @Override
    public IArrayWrapper createInstance(Object data) {
        return new JavaByteArrayWrapper((byte[]) data);
    }

    @Override
    public void serializeInto(ByteArrayOutputStream output) throws IOException {
        output.write(data);
    }

    @Override
    public String getDescriptionString() {
        return D_TYPE; // for dtype=np.int8
    }

    @Override
    public Object createWithShape(int[] shape) {
        return Array.newInstance(byte.class, shape);
    }

    @Override
    public Object readFromBuffer(BinaryBuffer buffer) {
        return buffer.readByte();
    }

}
