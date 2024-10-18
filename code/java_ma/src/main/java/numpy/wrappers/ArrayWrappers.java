package numpy.wrappers;

import numpy.IArrayWrapper;

import java.util.ArrayList;

public class ArrayWrappers {
    private static ArrayList<IArrayWrapper> wrappers = null;

    static {
        wrappers = new ArrayList<>();

        wrappers.add(new JavaByteArrayWrapper());
        wrappers.add(new JavaShortArrayWrapper());
        wrappers.add(new JavaIntArrayWrapper());
        wrappers.add(new JavaLongArrayWrapper());
        wrappers.add(new JavaFloatArrayWrapper());
        wrappers.add(new JavaDoubleArrayWrapper());

        wrappers.add(new JavaGenericArrayWrapper<Byte>());
        wrappers.add(new JavaGenericArrayWrapper<Short>());
        wrappers.add(new JavaGenericArrayWrapper<Integer>());
        wrappers.add(new JavaGenericArrayWrapper<Long>());
        wrappers.add(new JavaGenericArrayWrapper<Float>());
        wrappers.add(new JavaGenericArrayWrapper<Double>());

        // This should be after the atomic arrays
        wrappers.add(new JavaArrayOfArrayWrapper());
    }

    public static IArrayWrapper createWrapper(Object array) {
        IArrayWrapper result = null;

        for (IArrayWrapper wrapper : wrappers) {
            if (wrapper.supportType(array)) {
                result = wrapper.createInstance(array);
                break;
            }
        }

        if (result == null) {
            throw new RuntimeException("Unsupported array type.");
        }

        return result;
    }

    public static IArrayWrapper getWrapperForDType(String dtype) {
        IArrayWrapper result = null;

        for (IArrayWrapper wrapper : wrappers) {
            if (!wrapper.isArrayOfArray() && wrapper.getDescriptionString().equals(dtype)) {
                result = wrapper;
                break;
            }
        }

        return result;
    }
}
