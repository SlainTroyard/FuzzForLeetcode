import os
import subprocess
import random
import time

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate
max_n = 200000  # Maximum value for n (array size)
max_k = 100  # Maximum value for k

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_423_p2/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_423_p2.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# Function to generate a single test input
def generate_test_input():
    # Set seed for random number generator
    random.seed(time.time())
    
    # Randomly choose the size of the array
    n = random.randint(2, max_n)

    if n<=4:
        k=1
    else:
        # Randomly select a valid k (should be at most n//2)
        select = random.randint(0, 4)
        if select == 0:
            max_k = 100
        elif select == 1:
            max_k = 1000
        elif select == 2:
            max_k = 10000
        else:
            max_k = 99999
        k = random.randint(2, min(n // 2, max_k))
    
    # Start generating the array with two strictly increasing subarrays of length k
    nums = [0] * n

    # Start index for first subarray
    start_idx_1 = random.randint(0, n - 2 * k)
    
    add_A = random.randint(0, 100)

    # Generate first increasing subarray nums[start_idx_1 .. start_idx_1 + k - 1]
    for i in range(start_idx_1, start_idx_1 + k):
        nums[i] = i + add_A  # Increasing sequence [5, 6, 7 ...]

    # Start index for second subarray, which must be immediately after the first one
    start_idx_2 = start_idx_1 + k

    add_B = random.randint(0, 100)
    
    # Generate second increasing subarray nums[start_idx_2 .. start_idx_2 + k - 1]
    for i in range(start_idx_2, start_idx_2 + k):
        nums[i] = i + add_B  # Increasing sequence [1, 2, 3 ...]

    # Fill the rest of the array randomly, but ensure that no additional increasing sequences of length >= k exist
    max_val = 1000 - add_A - add_B
    for i in range(n):
        if nums[i] == 0:  # If the number is still zero, fill it with a random value
            nums[i] = random.randint(-1000, max_val)

    # Return the test input as a tuple (array size, nums array)
    return (n, nums)

# Function to format the test input as a string for terminal input simulation
def format_test_input(test_input):
    n, nums = test_input
    return f"{n}\n" + " ".join(map(str, nums)) + "\n"

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
                # TODO: if the formatted_input ends with a newline, remove the newline
                if formatted_input.endswith("\n"):
                    formatted_input = formatted_input[:-1]
                f.write(f"input:\n{formatted_input}\n")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
