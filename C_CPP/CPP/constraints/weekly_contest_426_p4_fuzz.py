import os
import subprocess
import random
import time

# TODO: Configure test case generation parameters
test_cases = 10  # Number of test cases to generate


# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_426_p4/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_426_p4.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
# Constraints
min_n = 2
max_n = 10**5
min_m = 2
max_m = 10**5

# Function to generate test input
def generate_test_input():
    random.seed(time.time())
    # Randomly select n and m within the valid range
    n = random.randint(min_n, max_n)
    m = random.randint(min_m, max_m)
    
    # Generate edges1 for the first tree
    edges1 = []
    for i in range(1, n):  # Create n-1 edges
        parent = random.randint(0, i - 1)  # Ensure it's a valid tree
        edges1.append([parent, i])
    
    # Generate edges2 for the second tree
    edges2 = []
    for i in range(1, m):  # Create m-1 edges
        parent = random.randint(0, i - 1)  # Ensure it's a valid tree
        edges2.append([parent, i])
    
    return edges1, edges2

# Function to format the test input for terminal simulation
def format_test_input(test_input):
    edges1, edges2 = test_input
    n = len(edges1) + 1  # Number of nodes in the first tree
    m = len(edges2) + 1  # Number of nodes in the second tree
    
    # Format input as required
    formatted_input = f"{n}\n"  # Number of nodes in both trees
    for edge in edges1:
        formatted_input += f"{edge[0]} {edge[1]}\n"
    formatted_input += f"{m}\n"  # Number of nodes in both trees
    for edge in edges2:
        formatted_input += f"{edge[0]} {edge[1]}\n"
    
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
                f.write(f"input:\n{formatted_input}")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
