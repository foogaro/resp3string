from resp3string import PyRedisConnection, PySetCommand, PyGetCommand

# Create a connection to Redis
conn = PyRedisConnection("127.0.0.1:6379")

# Send a command to Redis (e.g., PING)
response = conn.send_command("PING\r\n")

# Print the response from Redis
print(f"Response from Redis: {response}")

# Create a SetCommand to set a key-value pair in Redis
set_command = PySetCommand("mykey", "myvalue")

# Execute the SetCommand using the connection
response = set_command.execute(conn)

# Print the response from Redis
print(f"Response from Redis: {response}")

# Create a GetCommand to get a of a key fromRedis
get_command = PyGetCommand("mykey")

# Execute the GetCommand using the connection
response = get_command.execute(conn)

# Print the response from Redis
print(f"Response from Redis: {response}")
