import os
import subprocess
import random
import time

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_424_p4/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_424_p4.c"  # C source file name
executable_name = "solution"  # Executable name


# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())  # Ensure different random results each time the script runs
    n = random.randint(2, 10**5)  # Random length of the array between 2 and 10^5
    nums = []
    
    for _ in range(n):
        if random.random() < 0.2:  # 20% chance to insert -1
            nums.append(-1)
        else:
            # Random integer between 1 and 10^9
            nums.append(random.randint(1, 10**9))
    
    return nums

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    # First, the size of the array (n)
    n = len(test_input)
    # Then, the elements of the array (nums[i1] nums[i2] ... nums[in])
    return f"{n}\n" + " ".join(map(str, test_input))

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