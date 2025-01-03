import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_423_p1/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_423_p1.c"  # C source file name
executable_name = "solution"  # Executable name

def generate_test_input():
    random.seed(time.time())
    # Random array length between 2 and 100
    n = random.randint(2, 100)
    
    # Ensure k satisfies the constraint 1 < 2 * k <= nums.length
    if n <= 4:
        k = 1
    else:
        k = random.randint(1, n // 2)  # k must be at least 2 and at most n // 2
    
    nums = []
    for i in range(n):
        nums.append(random.randint(-1000, 999))  # Populate with random numbers between -1000 and 1000
    
    rands = random.randint(1, 10)  # Random number between 1 and 10
    # Ensure at least half of the test cases are true
    if rands < 5:
        # Make sure the test case has two increasing subarrays
        start_idx_1 = random.randint(0, n - 2 * k)  # Find the first index for the first subarray
        start_idx_2 = start_idx_1 + k  # Ensure that the second subarray is exactly k elements after the first
        
        # Create strictly increasing subarrays
        for i in range(start_idx_1, start_idx_1 + k - 1):
            nums[i] = i + 1  # First increasing subarray [5, 6, 7...]
        for i in range(start_idx_2, start_idx_2 + k - 1):
            nums[i] = i + 1  # Second increasing subarray [1, 2, 3...]
    
    # Return the formatted test input
    return (nums,k)

# Function to format test input as a string for terminal input simulation
def format_test_input(test_input):
    nums,k = test_input
    n = len(nums)
    test_input_str = f"{n}\n"  # First line is the number of elements
    test_input_str += " ".join(map(str, nums))  # Rest is the space-separated array
    test_input_str += f"\n{k}\n"  # Append k value to the input
    return test_input_str

# Compile the C program
def compile_c():
    try:
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name)] # sometimes need to add -lm for math library
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
                # TODO: if the formatted_input ends with a newline, remove the newline
                if formatted_input.endswith("\n"):
                    formatted_input = formatted_input[:-1]
                f.write(f"input:\n{formatted_input}\n")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
