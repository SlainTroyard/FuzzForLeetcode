import os
import subprocess
import random
import time

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_428_p1/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_428_p1.c"  # C source file name
executable_name = "solution"  # Executable name

# Generate a single test case
def generate_test_input():
    random.seed(time.time())
    n = random.randint(1, 1000)  # Number of events
    events = []
    last_time = 0
    for _ in range(n):
        index = random.randint(1, 100000)
        last_time += random.randint(1, 100)  # Ensure increasing times
        events.append((index, last_time))
    return n, events


# Format test_input as a string for terminal input simulation
def format_test_input(n, events):
    result = [f"{n}"]
    result.extend(f"{index} {time}" for index, time in events)
    return "\n".join(result)


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
    try:
        run_command = [os.path.join(c_folder, executable_name)]
        process = subprocess.run(run_command, input=test_input, text=True, capture_output=True, check=True)
        return process.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"Error during execution: {e}")
        return "Error"


# Main program for testing
if __name__ == "__main__":
    try:
        compile_c()
        with open(output_file, "w") as f:
            for case_id in range(1, test_cases + 1):
                n, events = generate_test_input()
                test_input = format_test_input(n, events)
                expected_output = simulate_output(test_input)
                f.write(f"Input:\n{test_input}\n")
                f.write(f"Output:\n{expected_output}\n")
                f.write("-------------------------\n")
    finally:
        executable_path = os.path.join(c_folder, executable_name)
        if os.path.exists(executable_path):
            os.remove(executable_path)
