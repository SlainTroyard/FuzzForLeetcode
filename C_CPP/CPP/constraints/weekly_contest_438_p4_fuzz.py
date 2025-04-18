import os
import subprocess
import random
import string
import time

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
max_side = 10**9  # Maximum side length of the square
max_points = 15 * 10**3  # Maximum number of points
max_k = 25  # Maximum number of points to select
min_side = 1  # Minimum side length of the square
min_points = 4  # Minimum number of points
min_k = 4  # Minimum number of points to select

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_438_p4/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_438_p4.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    # 1 <= side <= 10^9, 4 <= points.length <= min(4 * side, 15 * 10^3), points[i] == [xi, yi]. The input is generated such that: points[i] lies on the boundary of the square. All points[i] are unique. 4 <= k <= min(25, points.length)
    side = random.randint(min_side, max_side)
    points = []
    points_count = random.randint(min_points, min(4 * side, max_points))
    for _ in range(points_count):
        x = random.randint(0, side)
        y = random.randint(0, side)
        while [x, y] in points:
            x = random.randint(0, side)
            y = random.randint(0, side)
        points.append([x, y])
    k = random.randint(min_k, min(25, points_count))
    return side, points_count, k, points

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    side, points_count, k, points = test_input
    formatted_input = f"{side} {points_count} {k}\n"
    for point in points:
        formatted_input += f"{point[0]} {point[1]}\n"
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
