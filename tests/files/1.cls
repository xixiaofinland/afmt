public class TestObject2 {

    private static final Integer DEFAULT_SIZE = 10;

    Integer size;

    private TestObject2() {
        this(DEFAULT_SIZE);
    }

    // Constructor with one argument
    private TestObject2(Integer ObjectSize) {
        size = ObjectSize;
    }
}
