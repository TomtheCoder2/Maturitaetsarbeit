package nn;

public class Pair<T, K> {
    public T first;
    public K second;

    public Pair(T t, K k) {
        first = t;
        second = k;
    }

    public Pair() {
        first = null;
        second = null;
    }

    public Pair<K, T> switchValues() {
        if (first.getClass() == second.getClass()) {
            T tmp = first;
            first = (T) second;
            second = (K) tmp;
        }
        return new Pair<K, T>(second, first);
    }

    public void print() {
        System.out.printf("{%s, %s}", first, second);
    }

    public String toString() {
        return String.format("{%s, %s}", first.toString(), second.toString());
    }
}