package numpy;

/**
 * Created by yektaie on 8/5/16.
 */
public class BinaryBuffer {
    private static final int INTEGER_SIZE = 4;
    private static final int SHORT_SIZE = 2;
    private static final int BYTE_SIZE = 1;

    /**
     * Byte array storing the BinaryBuffer data.
     */
    protected byte[] buffer = null;

    /**
     * Point to the index where next read will happen.
     */
    public int readIndex = 0;

    /**
     * Initialize a BinaryBuffer from a byte array. Such a buffer will be read
     * only!
     *
     * @param buffer
     *            Input buffer to make BinaryBuffer.
     */
    public BinaryBuffer(byte[] buffer) {
        this.buffer = buffer;
    }

    /**
     * Read a signed int from the buffer.
     *
     * @return Signed int read from the buffer.
     * @throws Exception
     *             on error.
     */
    public int readInt() {
        int result = 0;
        int size = INTEGER_SIZE;

        if (readIndex + size <= buffer.length) {
            for (int i = 0; i < size; i++) {
                result = result * 256;
                result = ((int) (buffer[readIndex + size - 1 - i] & 0x000000FF)) + result;
            }
            readIndex += size;
        } else {
            throw new RuntimeException();
        }

        return result;
    }

    /**
     * Read a signed long from the buffer.
     *
     * @return Signed long read from the buffer.
     * @throws Exception
     *             on error.
     */
    public long readLong() {
        long result = 0;
        int size = 8;

        if (readIndex + size <= buffer.length) {
            for (int i = 0; i < size; i++) {
                result = result << 8;
                result = ((int) (buffer[readIndex + size - 1 - i] & 0x000000FF)) + result;
            }
            readIndex += size;
        } else {
            throw new RuntimeException();
        }

        return result;
    }

    /**
     * Read an unsigned int from the buffer.
     *
     * @return Unsigned byte read from the buffer.
     * @throws Exception
     *             on error.
     */
    public byte readByte() {
        byte result = 0;

        if (readIndex + 1 <= buffer.length) {
            result = buffer[readIndex];
            readIndex += 1;
        } else {
            throw new RuntimeException();
        }

        return result;
    }

    /**
     * Read a short int from the buffer.
     *
     * @return Short int read from the buffer.
     * @throws Exception
     *             on error.
     */
    public short readShort() {
        int result = 0;
        short size = SHORT_SIZE;

        if (readIndex + size <= buffer.length) {
            for (int i = 0; i < size; i++) {
                result = (result << 8);
                result = ((buffer[readIndex + size - 1 - i] & 0x000000ff) | result);
            }
            readIndex += size;
        } else {
            throw new RuntimeException();
        }

        return (short) result;
    }

    public byte[] readByteArray(int length) {
        byte[] result = new byte[length];

        for (int i = 0; i < result.length; i++) {
            result[i] = buffer[readIndex];
            readIndex++;
        }

        return result;
    }

    /**
     * Read a float from the buffer.
     *
     * @return float read from the buffer.
     * @throws Exception
     *             on error.
     */
    public float readFloat() {
        float result = 0;
        int temp = readInt();
        result = Float.intBitsToFloat(temp);

        return result;
    }

    /**
     * Read a double from the buffer.
     *
     * @return float read from the buffer.
     * @throws Exception
     *             on error.
     */
    public double readDouble() {
        double result = 0;
        long temp = readLong();
        result = Double.longBitsToDouble(temp);

        return result;
    }
}