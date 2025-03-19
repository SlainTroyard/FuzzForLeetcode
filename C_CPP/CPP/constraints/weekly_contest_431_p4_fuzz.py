import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate
max_intervals = 5*10**4  # Maximum number of intervals
max_li = 10**9  # Maximum value of li
max_ri = 10**9  # Maximum value of ri
max_weight = 10**9  # Maximum value of weight

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_431_p4/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_431_p4.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    # 1 <= intevals.length <= 5 * 10^4, intervals[i].length == 3, intervals[i] = [li, ri, weighti], 1 <= li <= ri <= 10^9, 1 <= weighti <= 10^9
    length = random.randint(1, max_intervals)
    intervals = []
    # 在1-10^9中随机取出length个不重复区间
    ranges = random.sample(range(1, 10**9), length*2)
    ranges.sort()
    for i in range(length):
        li = ranges[i*2]
        ri = ranges[i*2+1]
        weight = random.randint(1, max_weight)
        intervals.append([li, ri, weight])
    # 打乱顺序
    random.shuffle(intervals)
    return length, intervals

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    length, intervals = test_input
    formatted_input = f"{length}\n"
    for interval in intervals:
        formatted_input += f"{interval[0]} {interval[1]} {interval[2]}\n"
    return formatted_input

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
