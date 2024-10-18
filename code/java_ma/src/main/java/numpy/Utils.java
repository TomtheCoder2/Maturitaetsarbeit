package numpy;

public class Utils {
    public static byte[] serializeToBuffer(long value, int size) {
        byte[] result = new byte[size];

        for (int i = 0; i < size; i++) {
            byte toAdd = (byte) (value & 0x00000000000000FF);
            result[i] = toAdd;
            value = (value >> 8);
        }

        return result;
    }
}
