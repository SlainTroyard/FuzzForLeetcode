import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 10  # Number of test cases to generate
max_n = 10**5  # Maximum value for n
min_n = 1  # Minimum value for n
min_k = 0  # Minimum value for k
max_len = 2 * 10**5  # Maximum length of invocations
min_len = 0  # Minimum length of invocations
min_val = 0  # Minimum value for ai, bi

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_418_p2/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_418_p2.c"  # C source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    # 1 <= n <= 10^5, 0 <= k <= n - 1, 0 <= invocations.length <= 2 * 10^5, invocations[i] == [ai, bi], 0 <= ai, bi <= n - 1, ai != bi, invocations[i] != invocations[j]
    n = random.randint(min_n, max_n)
    k = random.randint(min_k, n - 1)
    invocations = []
    len_invocations = random.randint(min_len, max_len)
    for _ in range(len_invocations):
        ai = random.randint(min_val, n - 1)
        bi = random.randint(min_val, n - 1)
        while ai == bi:
            bi = random.randint(min_val, n - 1)
        # Ensure that invocations[i] != invocations[j]
        while [ai, bi] in invocations:
            ai = random.randint(min_val, n - 1)
            bi = random.randint(min_val, n - 1)
            while ai == bi:
                bi = random.randint(min_val, n - 1)
        invocations.append([ai, bi])
    return n, k, invocations

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    n, k, invocations = test_input
    formatted_input = f"{n} {k}\n"
    formatted_input += f"{len(invocations)}\n"
    for ai, bi in invocations:
        formatted_input += f"{ai} {bi}\n"
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
