import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 15  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_413_p4/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_413_p4.c"  # C source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    # 1 <= n == nums.length <= 2000, 0 <= nums[i] <= 2^31 - 1, 1 <= q == queries.length <= 10^5, queries[i].length == 2, queries[i] = [li, ri], 0 <= li <= ri <= n - 1
    n = random.randint(1, 2000)
    nums = [random.randint(0, 2**31 - 1) for i in range(n)]
    q = random.randint(1, 10**5)
    queries = []
    for i in range(q):
        li = random.randint(0, n - 1)
        ri = random.randint(li, n - 1)
        queries.append([li, ri])
    return [n, nums, q, queries]

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    n, nums, q, queries = test_input
    formatted_input = f"{n}\n"
    formatted_input += " ".join(map(str, nums)) + "\n"
    formatted_input += f"{q}\n"
    for query in queries:
        formatted_input += " ".join(map(str, query)) + "\n"
    return formatted_input

# Compile the C program
def compile_c():
    try:
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name), "-lm"] # sometimes need to add -lm for math library
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