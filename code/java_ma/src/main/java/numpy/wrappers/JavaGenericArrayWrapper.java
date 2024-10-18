package numpy.wrappers;

import numpy.BinaryBuffer;
import numpy.IArrayWrapper;
import numpy.NumpyArray;
import numpy.Utils;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.lang.reflect.Array;

public class JavaGenericArrayWrapper<Type extends Number> implements IArrayWrapper {
    private Type[] data = null;

    public JavaGenericArrayWrapper() {

    }

    public JavaGenericArrayWrapper(Type[] data) {

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
        // Stupid Java!!!!
        try {
            Type[] temp = (Type[]) data;
            return true;
        } catch (Exception ex) {
            return false;
        }
    }

    @Override
    public IArrayWrapper createInstance(Object data) {
        return new JavaGenericArrayWrapper<Type>((Type[]) data);
    }

    @Override
    public void serializeInto(ByteArrayOutputStream output) throws IOException {
        if (isFloatingPointArray()) {
            for (int i = 0; i < data.length; i++) {
                Number number = data[i];
                output.write(getFloatingPointBuffer(number));
            }
        } else {
            int length = getNotFloatingPointElementLength();
            for (int i = 0; i < data.length; i++) {
                long number = (long) data[i];
                output.write(Utils.serializeToBuffer(number, length));
            }
        }
    }

    @Override
    public String getDescriptionString() {
        String result = "";

        if (isFloatingPointArray()) {
            Number number = data[0];

            if (number instanceof Double) {
                result = JavaDoubleArrayWrapper.D_TYPE;
            } else if (number instanceof Float) {
                result = JavaFloatArrayWrapper.D_TYPE;
            }
        } else {
            int length = getNotFloatingPointElementLength();

            switch (length) {
                case 8:
                    result = JavaLongArrayWrapper.D_TYPE;
                    break;
                case 4:
                    result = JavaIntArrayWrapper.D_TYPE;
                    break;
                case 2:
                    result = JavaShortArrayWrapper.D_TYPE;
                    break;
                case 1:
                    result = JavaByteArrayWrapper.D_TYPE;
                    break;
            }
        }

        return result;
    }

    private boolean isFloatingPointArray() {
        return (data[0] instanceof Double) || (data[0] instanceof Float);
    }

    private int getNotFloatingPointElementLength() {
        Number n = data[0];
        int result = 0;

        if (n instanceof Long) {
            result = 8;
        } else if(n instanceof Integer) {
            result = 4;
        } else if (n instanceof Short) {
            result = 2;
        } else if (n instanceof Byte) {
            result = 1;
        }

        return result;
    }

    private byte[] getFloatingPointBuffer(Number number) {
        if (number instanceof Double) {
            double value = (double) number;
            return Utils.serializeToBuffer(Double.doubleToLongBits(value), 8);
        } else if (number instanceof Float) {
            float value = (float) number;
            return Utils.serializeToBuffer(Float.floatToIntBits(value), 4);
        }

        throw new RuntimeException("Not implemented");
    }

    @Override
    public Object createWithShape(int[] shape) {
        throw new RuntimeException("This should not happen");
    }

    @Override
    public Object readFromBuffer(BinaryBuffer buffer) {
        throw new RuntimeException("This should not happen");
    }
}
