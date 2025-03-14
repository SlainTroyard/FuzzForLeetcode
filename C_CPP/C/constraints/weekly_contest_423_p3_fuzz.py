import os
import subprocess
import random
import time

# Configuration for the test case generation
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_423_p3/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_423_p3.c"  # C source file name
executable_name = "solution"  # Executable name

# Function to generate a single test case
def generate_test_input():
    random.seed(time.time())
    n = random.randint(1, 10**5)  # Length of the array, between 1 and 10^5
    nums = [random.randint(0, 100000) for _ in range(n)]  # Generate an initial random array
    
    # Check if there's already a good subsequence (i.e., two consecutive elements with difference 1)
    good_subsequence_found = False
    for i in range(1, n):
        if abs(nums[i] - nums[i - 1]) == 1:
            good_subsequence_found = True
            break
    
    # If no good subsequence is found, randomly modify two consecutive values to form a good subsequence
    if not good_subsequence_found:
        idx = random.randint(0, n - 2)  # Random index for the modification
        nums[idx] = random.randint(0, 99999)  # Random number
        nums[idx + 1] = nums[idx] + 1  # Make the next number differ by 1 (good subsequence)
    
    return nums

# Function to format test input for the terminal input simulation
def format_test_input(test_input):
    return f"{len(test_input)}\n" + " ".join(map(str, test_input)) + "\n"

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
                # If the formatted_input ends with a newline, remove the newline
                if formatted_input.endswith("\n"):
                    formatted_input = formatted_input[:-1]
                f.write(f"input:\n{formatted_input}\n")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
