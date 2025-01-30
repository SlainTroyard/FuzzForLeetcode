import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate
max_n = 100000  # Maximum value of n
min_n = 1  # Minimum value of n

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_420_p4/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_420_p4.c"  # C source file name
executable_name = "solution"  # Executable name

def generate_test_input():
    random.seed(time.time())
    n = random.randint(1, 100000)
    parent = [-1]
    for i in range(1, n):
        p = random.randint(0, i - 1)
        parent.append(p)
    s = ''.join(random.choice(string.ascii_lowercase) for _ in range(n))
    # n = 6
    # parent = [-1, 0, 0, 1, 1, 2]
    # s = "aababa"
    return (n, parent, s)

def format_test_input(test_input):
    n, parent, s = test_input
    formatted_input = f"{n}\n"
    formatted_input += ' '.join(map(str, parent)) + "\n"
    formatted_input += s
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
