package numpy;

import java.io.ByteArrayOutputStream;
import java.io.IOException;

public interface IArrayWrapper {
    int size();
    boolean isArrayOfArray();
    Object get(int index);
    boolean supportType(Object data);
    IArrayWrapper createInstance(Object data);

    void serializeInto(ByteArrayOutputStream output) throws IOException;
    String getDescriptionString();

    Object createWithShape(int[] shape);

    Object readFromBuffer(BinaryBuffer buffer);
}
