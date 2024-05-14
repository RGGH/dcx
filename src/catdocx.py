import subprocess

# Define the arguments
args = [
    "cargo", "run", "--",
    "-t", "moo.txt",
    "-l", "this is my headline nice eh?"
]

# Call the Rust binary using subprocess.run()
result = subprocess.run(args, capture_output=True, text=True)

# Print the output
print("stdout:")
print(result.stdout)

print("stderr:")
print(result.stderr)

# Check if the command was successful
if result.returncode == 0:
    print("Success!")
else:
    print("Error:", result.returncode)

