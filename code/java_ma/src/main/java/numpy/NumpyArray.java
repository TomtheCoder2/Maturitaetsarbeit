package numpy;

import numpy.wrappers.ArrayWrappers;

import java.io.*;
import java.lang.reflect.Array;
import java.nio.charset.StandardCharsets;

public class NumpyArray {
    private static final byte[] MAGIC_STRING_HEADER = {(byte) 0x93, 'N', 'U', 'M', 'P', 'Y'};

    private IArrayWrapper data = null;

    public NumpyArray(Object data) {
        this.data = ArrayWrappers.createWrapper(data);
    }

    public static Object load(String path) {
        try {
            BufferedInputStream stream = new BufferedInputStream(new FileInputStream(path));
            Object result = load(stream);
            stream.close();

            return result;
        } catch (Exception e) {
            if (e instanceof RuntimeException) {
                throw ((RuntimeException) e);
            } else {
                throw new RuntimeException(e);
            }
        }
    }

    public static Object load(InputStream stream) {
        try {
            byte[] content = new byte[stream.available()];
            stream.read(content);

            return parseNumpyArray(content);
        } catch (Exception ex) {
            throw new RuntimeException(ex);
        }
    }

    private static Object parseNumpyArray(byte[] content) {
        BinaryBuffer buffer = new BinaryBuffer(content);

        checkMagicStringHeader(buffer);
        checkVersion(buffer);

        int headerLength = buffer.readShort();
        String header = new String(buffer.readByteArray(headerLength), StandardCharsets.US_ASCII).trim();
        String description = getDescriptionFromHeader(header);
        int[] shape = getShapeFromHeader(header);

        IArrayWrapper wrapper = ArrayWrappers.getWrapperForDType(description);
        Object result = wrapper.createWithShape(shape);
        loadArrayData(buffer, result, shape, 0, wrapper);

        return result;
    }

    private static void loadArrayData(BinaryBuffer buffer, Object result, int[] shape, int shapeIndex, IArrayWrapper wrapper) {
        if (shapeIndex + 1 == shape.length) {
            for (int i = 0; i < shape[shapeIndex]; i++) {
                Array.set(result, i, wrapper.readFromBuffer(buffer));
            }
        } else {
            for (int i = 0; i < shape[shapeIndex]; i++) {
                Object array = Array.get(result, i);
                loadArrayData(buffer, array, shape, shapeIndex + 1, wrapper);
            }
        }
    }

    private static String getDescriptionFromHeader(String header) {
        String name = "descr";
        String look = String.format("'%s': ", name);

        if (!header.contains(name)) {
            throw new RuntimeException("Invalid header. Missing '" + name + "' value.");
        }
        header = header.substring(header.indexOf(look) + look.length());
        header = header.substring(0, header.indexOf(","));

        header = header.replace("'", "");

        return header;
    }

    private static int[] getShapeFromHeader(String header) {
        String name = "shape";
        String look = String.format("'%s': (", name);

        if (!header.contains(name)) {
            throw new RuntimeException("Invalid header. Missing '" + name + "' value.");
        }
        header = header.substring(header.indexOf(look) + look.length());
        header = header.substring(0, header.indexOf("),"));

        if (!header.contains(" ")) {
            // This is a single dimension data. Header value should be "N,". Should remove the last comma
            header = header.substring(0, header.length() - 1);
        }

        String[] dims = header.split(", ");
        int[] result = new int[dims.length];

        for (int i = 0; i < result.length; i++) {
            result[i] = Integer.parseInt(dims[i]);
        }

        return result;
    }

    private static void checkVersion(BinaryBuffer buffer) {
        byte major = buffer.readByte();
        byte minor = buffer.readByte();

        if (major != 1 || minor != 0) {
            throw new RuntimeException("This numpy stream is not compatible with version 1.0.");
        }
    }

    private static void checkMagicStringHeader(BinaryBuffer buffer) {
        byte[] magicHeader = buffer.readByteArray(MAGIC_STRING_HEADER.length);
        boolean same = true;

        for (int i = 0; i < magicHeader.length && same; i++) {
            same = magicHeader[i] == MAGIC_STRING_HEADER[i];
        }

        if (!same) {
            throw new RuntimeException("This stream is not a valid numpy array.");
        }
    }

    public void save(String path) {
        try {
            FileOutputStream fs = new FileOutputStream(path);
            fs.write(toByteArray());

            fs.flush();
            fs.close();
        } catch (Exception e) {
            throw new RuntimeException(e);
        }
    }

    public void save(OutputStream stream) throws IOException {
        byte[] buffer = toByteArray();
        stream.write(buffer);
    }

    private byte[] toByteArray() {
        // For more detail information on the file format refer to:
        // https://numpy.org/neps/nep-0001-npy-format.html

        ByteArrayOutputStream output = new ByteArrayOutputStream();

        try {
            addFileHeaderToBuffer(output);
            addArrayContentToBuffer(output);
        } catch (Exception ex) {
            throw new RuntimeException(ex);
        }

        return output.toByteArray();
    }

    private void addArrayContentToBuffer(ByteArrayOutputStream output) throws IOException {
        data.serializeInto(output);
    }

    private void addFileHeaderToBuffer(ByteArrayOutputStream output) throws IOException {
        int lengthSoFar = 0;

        lengthSoFar = addMagicString(output);
        lengthSoFar += addFileVersion(output);

        lengthSoFar += 2; // The header length occupies 2 bytes. The final header length including the fields
        //                   added before should be divisible by 16. This is required by the specification.

        StringBuilder header = new StringBuilder();
        short headerLength = createHeaderString(header, lengthSoFar);

        // Write header length
        output.write(Utils.serializeToBuffer(headerLength, 2));

        // Write header content
        output.write(header.toString().getBytes(StandardCharsets.US_ASCII));

        // Just to check, if this exception occurs, there is a serious bug in the header serialization
        if ((output.toByteArray().length % 16) != 0) {
            throw new RuntimeException("Error in serializing numpy header.");
        }
    }

    private short createHeaderString(StringBuilder header, int lengthSoFar) {
        String headerContent = "{'descr': '%s', 'fortran_order': False, 'shape': (%s), }";
        headerContent = String.format(headerContent, getDataDescriptionType(), getDataShape());

        header.append(headerContent);
        header.append("                "); // Not sure if this is really needed. The sample files had additional space

        /*
            From the specification:
            -----------------------

            The next HEADER_LEN bytes form the header data describing the array’s format. It is an ASCII string
            which contains a Python literal expression of a dictionary. It is terminated by a newline (‘n’) and
            padded with spaces (‘x20’) to make the total length of the magic string + 4 + HEADER_LEN be evenly
            divisible by 16 for alignment purposes.
         */
        while (((lengthSoFar + header.length() + 1) % 32) != 0) {
            header.append(" ");
        }

        header.append('\n'); // Also required by the specification
        return (short) header.length();
    }

    private String getDataDescriptionType() {
        IArrayWrapper data = this.data;

        while (data.isArrayOfArray()) {
            data = (IArrayWrapper) data.get(0);
        }

        return data.getDescriptionString();
    }

    private String getDataShape() {
        StringBuilder result = new StringBuilder();

        IArrayWrapper data = this.data;
        boolean multi = false;

        while (data.isArrayOfArray()) {
            result.append(data.size());
            result.append(", ");
            multi = true;

            data = (IArrayWrapper) data.get(0);
        }

        result.append(data.size());
        if (!multi) {
            result.append(",");
        }

        return result.toString().trim();
    }

    private int addFileVersion(ByteArrayOutputStream output) throws IOException {
        // Use file format version 1.0
        byte[] version = {0x01, 0x00};
        output.write(version);

        return version.length;
    }

    private int addMagicString(ByteArrayOutputStream output) throws IOException {
        output.write(MAGIC_STRING_HEADER);

        return MAGIC_STRING_HEADER.length;
    }
}
