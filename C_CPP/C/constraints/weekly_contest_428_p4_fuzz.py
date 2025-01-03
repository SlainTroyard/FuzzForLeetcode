import os
import subprocess
import random
import string
import time 

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
min_length = 3  # Minimum length of the string
max_length = 20000  # Maximum length of the string

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_428_p4/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_428_p4.c"  # C source file name
executable_name = "solution"  # Executable name

# Generate a single test case
def generate_test_input():
    random.seed(time.time())
    n = random.randint(min_length, max_length)
    s = ''.join(random.choices(string.ascii_lowercase, k=n))  # Generate random string of lowercase letters
    return s

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    return f"{test_input}\n"

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
    formatted_input = format_test_input(test_input)
    try:
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
