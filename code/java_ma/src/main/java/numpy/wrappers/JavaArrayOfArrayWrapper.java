package numpy.wrappers;

import numpy.BinaryBuffer;
import numpy.IArrayWrapper;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.lang.reflect.Array;
import java.util.ArrayList;

public class JavaArrayOfArrayWrapper implements IArrayWrapper {
    private ArrayList<IArrayWrapper> data = null;

    public JavaArrayOfArrayWrapper() {

    }

    public JavaArrayOfArrayWrapper(Object data) {
        this.data = load(data);
    }

    private ArrayList<IArrayWrapper> load(Object data) {
        ArrayList<IArrayWrapper> result = new ArrayList<>();
        int length = Array.getLength(data);

        for (int i = 0; i < length; i++) {
            result.add(ArrayWrappers.createWrapper(Array.get(data, i)));
        }

        return result;
    }

    @Override
    public int size() {
        return this.data.size();
    }

    @Override
    public boolean isArrayOfArray() {
        return true;
    }

    @Override
    public Object get(int index) {
        return this.data.get(index);
    }

    @Override
    public boolean supportType(Object data) {
        return data.getClass().isArray();
    }

    @Override
    public IArrayWrapper createInstance(Object data) {
        return new JavaArrayOfArrayWrapper(data);
    }

    @Override
    public void serializeInto(ByteArrayOutputStream output) throws IOException {
        for (IArrayWrapper wrapper : data) {
            wrapper.serializeInto(output);
        }
    }

    @Override
    public String getDescriptionString() {
        throw new RuntimeException("This should not happen!");
    }

    @Override
    public Object createWithShape(int[] shape) {
        throw new RuntimeException("This should not happen!");
    }

    @Override
    public Object readFromBuffer(BinaryBuffer buffer) {
        throw new RuntimeException("This should not happen");
    }

}
