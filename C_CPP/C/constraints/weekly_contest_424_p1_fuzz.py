import os
import subprocess
import random
import time

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_424_p1/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_424_p1.c"  # C source file name
executable_name = "solution"  # Executable name


# Generate a single test case
def generate_test_input():
    random.seed(time.time())
    n = random.randint(1, 100)  
    
    max_value_prob = random.randint(1, 6)  
    if max_value_prob == 1:
        max_value = 10
    elif max_value_prob == 2:
        max_value = 3
    elif max_value_prob == 3:
        max_value = 1
    elif max_value_prob == 4:
        max_value = 2
    elif max_value_prob == 5:
        max_value = 5
    else:
        max_value = 100
    
    nums = [random.randint(1, max_value) for _ in range(n)]
    ratio_choice = random.choice([1/4, 1/8, 3/4, 7/8, 0]) 
    zero_count = max(1, int(n * ratio_choice))  
    zero_indices = random.sample(range(n), zero_count)  
    for idx in zero_indices:
        nums[idx] = 0
    
    return (n, nums)

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    n, nums = test_input
    # Format as required for the C++ program input (first line with n, second line with space-separated nums)
    formatted_input = f"{n}\n" + " ".join(map(str, nums)) + "\n"
    return formatted_input

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
