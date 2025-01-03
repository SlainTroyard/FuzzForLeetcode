import os
import subprocess
import random
import time

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
min_n = 1  # Minimum value for n
max_n = 10**5  # Maximum value for n
min_m = 1  # Minimum value for m
max_m = 10**5  # Maximum value for m

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_430_p4/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_430_p4.c"  # C source file name
executable_name = "solution"  # Executable name

# Generate a single test case
def generate_test_input():
    random.seed(time.time())
    n = random.randint(min_n, max_n)
    m = random.randint(min_m, max_m)
    k = random.randint(0, n - 1)
    return n, m, k

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    n, m, k = test_input
    return f"{n} {m} {k}\n"

# Compile the C program
def compile_c():
    try:
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name)]
        subprocess.run(compile_command, check=True)
        print("Compilation successful.")
    except subprocess.CalledProcessError as e:
        print(f"Error during compilation: {e}")
        raise

# Simulate output for a test case by running the C program
def simulate_output(test_input):
    # Format input for the C program
    formatted_input = format_test_input(test_input)
    try:
        # Run the C program with the generated input
        run_command = [os.path.join(c_folder, executable_name)]
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, check=True)
        return process.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error during execution: {e}")
        return "Error"

# Clean up the compiled executable
def cleanup():
    executable_path = os.path.join(c_folder, executable_name)
    if os.path.exists(executable_path):
        os.remove(executable_path)
        print("Cleaned up compiled executable.")

# Main program
if __name__ == "__main__":
    try:
        compile_c()
        with open(output_file, "w") as f:
            for case_id in range(1, test_cases + 1):
                test_input = generate_test_input()
                formatted_input = format_test_input(test_input)
                simulated_output = simulate_output(test_input)

                f.write(f"input:\n{formatted_input}")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
