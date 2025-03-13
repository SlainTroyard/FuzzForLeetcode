import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate
max_mountain_height = 10**5
min_mountain_height = 1
max_worker_times_length = 10**4
min_worker_times_length = 1
max_worker_time = 10**6
min_worker_time = 1

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_416_p2/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_416_p2.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    # 1 <= mountainHeight <= 10^5, 1 <= workerTimes.length <= 10^4, 1 <= workerTimes[i] <= 10^6
    mountain_height = random.randint(min_mountain_height, max_mountain_height)
    worker_times_length = random.randint(min_worker_times_length, max_worker_times_length)
    worker_times = [random.randint(min_worker_time, max_worker_time) for _ in range(worker_times_length)]
    return mountain_height, worker_times

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    mountain_height, worker_times = test_input
    formatted_input = f"{mountain_height}\n{len(worker_times)}\n"
    for worker_time in worker_times:
        formatted_input += f"{worker_time} \n"
    return formatted_input[:-1]

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
