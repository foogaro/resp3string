public class TestRedis {
    static {
        // Load the Rust shared library (.so/.dll/.dylib depending on your OS)
        System.loadLibrary("resp3string");
    }

    // Native methods for interacting with the Rust library
    public native long createRedisConnection(String address);
    public native long createSetCommand(String key, String value);
    public native String executeSetCommand(long setCommandPtr, long connPtr);
    public native void freeRedisConnection(long connPtr);
    public native void freeSetCommand(long setCommandPtr);

    // Example usage
    public static void main(String[] args) {
        TestRedis rustRedis = new TestRedis();

        // Step 1: Create RedisConnection
        long conn = rustRedis.createRedisConnection("127.0.0.1:6379");

        // Step 2: Create a SetCommand
        long setCommand = rustRedis.createSetCommand("mykey", "myvalue");

        // Step 3: Execute the SetCommand
        String response = rustRedis.executeSetCommand(setCommand, conn);
        System.out.println("Response from Redis: " + response);

        // Step 4: Free the memory (clean up)
        rustRedis.freeSetCommand(setCommand);
        rustRedis.freeRedisConnection(conn);
    }
}