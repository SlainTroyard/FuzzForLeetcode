import os
import subprocess
import random

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
max_rows = 50  # Maximum number of rows
max_cols = 50  # Maximum number of columns
max_value = 2499  # Maximum value in the grid

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_430_p1/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_430_p1.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    rows = random.randint(1, max_rows)  # Random number of rows
    cols = random.randint(1, max_cols)  # Random number of columns
    grid = [[random.randint(0, max_value) for _ in range(cols)] for _ in range(rows)]
    return grid

# TODO: Format grid as a string for terminal input simulation
def format_grid(grid):
    formatted = f"{len(grid)} {len(grid[0])}\n"
    formatted += "\n".join(" ".join(map(str, row)) for row in grid)
    return formatted

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
def simulate_output(grid):
    # Format input for the C++ program
    formatted_input = format_grid(grid)
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
                formatted_input = format_grid(test_input)
                simulated_output = simulate_output(test_input)

                f.write(f"input:\n{formatted_input}\n")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
