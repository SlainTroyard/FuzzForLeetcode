import os
import subprocess
import random
import time

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
min_length = 1
max_length = 100
min_value = -1000
max_value = 1000

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_425_p1/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_425_p1.c"  # C source file name
executable_name = "solution"  # Executable name

# Generate a single test case
def generate_test_input():
    random.seed(time.time())
    # Randomly generate array length
    nums_length = random.randint(min_length, max_length)
    # Generate random array elements
    nums = [random.randint(min_value, max_value) for _ in range(nums_length)]
    # Randomly generate l and r within the range [1, nums.length]
    l = random.randint(1, nums_length)
    r = random.randint(l, nums_length)
    return nums, l, r

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    nums, l, r = test_input
    formatted_input = f"{len(nums)}\n"
    formatted_input += " ".join(map(str, nums)) + "\n"
    formatted_input += f"{l} {r}\n"
    return formatted_input

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
                # Write input and output to the output file
                f.write(f"input:\n{formatted_input}")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
