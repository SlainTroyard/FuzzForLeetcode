import os
import subprocess
import random

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
min_array_size = 1  # Minimum size of the array
max_array_size = 10**5  # Maximum size of the array
min_value = 1  # Minimum value in the array
max_value = 10**9  # Maximum value in the array
min_diff = 0  # Minimum difference value
max_diff = 10**9  # Maximum difference value

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_429_p2/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_429_p2.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# Generate a single test case
def generate_test_input():
    n = random.randint(min_array_size, max_array_size)
    nums = [random.randint(min_value, max_value) for _ in range(n)]
    diff = random.randint(min_diff, max_diff)
    return nums, diff

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    nums, diff = test_input
    formatted_nums = " ".join(map(str, nums))
    return f"{len(nums)} {diff}\n{formatted_nums}\n"

# Compile the C++ program
def compile_cpp():
    try:
        compile_command = ["g++", os.path.join(cpp_folder, cpp_file), "-o", os.path.join(cpp_folder, executable_name)]
        subprocess.run(compile_command, check=True)
        print("Compilation successful.")
    except subprocess.CalledProcessError as e:
        print(f"Error during compilation: {e}")
        raise

# Simulate output for a test case by running the C++ program
def simulate_output(test_input):
    # Format input for the C++ program
    formatted_input = format_test_input(test_input)
    try:
        # Run the C++ program with the generated input
        run_command = [os.path.join(cpp_folder, executable_name)]
        process = subprocess.run(run_command, input=formatted_input, text=True, capture_output=True, check=True)
        return process.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error during execution: {e}")
        return "Error"

# Clean up the compiled executable
def cleanup():
    executable_path = os.path.join(cpp_folder, executable_name)
    if os.path.exists(executable_path):
        os.remove(executable_path)
        print("Cleaned up compiled executable.")

# Main program
if __name__ == "__main__":
    try:
        compile_cpp()
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
