import os
import subprocess
import random
import time

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate


# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_425_p4/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_425_p4.cpp"  # C++ source file name
executable_name = "solution"  # Executable name


# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    n = random.randint(2, 100000)  # n nodes (between 2 and 10^5)
    k = random.randint(1, n - 1)  # k edges to select (1 to n-1)
    
    # We will generate a valid tree by randomly selecting edges between nodes.
    edges = []
    # Generate the tree with random edges (using a simple DFS/BFS approach or random pairs)
    for i in range(1, n):
        u = random.randint(0, i - 1)  # Random parent node
        v = i  # Current node
        weight = random.randint(1, 1000000)  # Random edge weight between 1 and 10^6
        edges.append([u, v, weight])

    # Format test input as a string
    test_input = f"{n} {k}\n"
    for edge in edges:
        test_input += f"{edge[0]} {edge[1]} {edge[2]}\n"
    
    return test_input

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    # Test input is already a formatted string, we just return it
    return test_input

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
                f.write(f"input:\n{formatted_input}")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
