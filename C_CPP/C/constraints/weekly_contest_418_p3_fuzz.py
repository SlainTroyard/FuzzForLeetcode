import os
import subprocess
import random
import string
import time
import math

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate
'''
2 <= n <= 5 * 10^4
1 <= edges.length <= 10^5
edges[i] = [ui, vi]
0 <= ui < vi < n
All the edges are distinct.
The input is generated such that edges can form a 2D grid that satisfies the conditions.
'''
max_n = 50000
min_n = 2

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_418_p3/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_418_p3.c"  # C source file name
executable_name = "solution"  # Executable name

def generate_grid_layout(n):
    cols = math.isqrt(n)
    while cols > 1:
        if n % cols == 0:
            rows = n // cols
            break
        cols -= 1
    else:
        rows = math.isqrt(n)
        cols = math.ceil(n / rows)
    
    grid = [[-1 for _ in range(cols)] for _ in range(rows)]
    node = 0
    for r in range(rows):
        for c in range(cols):
            if node < n:
                grid[r][c] = node
                node += 1
            else:
                break
    return grid, rows, cols

def grid_to_edges(grid, rows, cols, n):
    edges = []
    for r in range(rows):
        for c in range(cols):
            if grid[r][c] == -1:
                continue
            current = grid[r][c]
            if c + 1 < cols and grid[r][c + 1] != -1:
                u, v = current, grid[r][c + 1]
                if u < v:
                    edges.append([u, v])
                else:
                    edges.append([v, u])
            if r + 1 < rows and grid[r + 1][c] != -1:
                u, v = current, grid[r + 1][c]
                if u < v:
                    edges.append([u, v])
                else:
                    edges.append([v, u])
    return edges

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(n, edges):
    formatted_input = f"{n} {len(edges)}\n"
    for edge in edges:
        formatted_input += f"{edge[0]} {edge[1]}\n"
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
    formatted_input = test_input
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
                n = random.randint(min_n, max_n)
                grid, rows, cols = generate_grid_layout(n)
                edges = grid_to_edges(grid, rows, cols, n)
                if len(edges) > 100000:
                    edges = edges[:100000]
                formatted_input = format_test_input(n, edges)
                simulated_output = simulate_output(formatted_input)
                if formatted_input.endswith("\n"):
                    formatted_input = formatted_input[:-1]
                f.write(f"input:\n{formatted_input}\n")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
