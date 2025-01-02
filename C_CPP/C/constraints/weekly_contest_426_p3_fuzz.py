import os
import subprocess
import random

# TODO: Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
min_n, max_n = 2, 1000  # n (number of nodes in the first tree)
min_m, max_m = 2, 1000  # m (number of nodes in the second tree)
min_k, max_k = 0, 1000  # k value range

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_426_p3/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_426_p3.c"  # C source file name
executable_name = "solution"  # Executable name

# Function to generate test input for the problem
def generate_test_input():
    # Randomly select n and m (between 2 and 1000)
    n = random.randint(min_n, max_n)
    m = random.randint(min_m, max_m)
    
    # Generate the edges for the first tree (edges1) - ensure edges are sorted and valid
    edges1 = []
    for i in range(1, n):  # Create edges for tree 1 (n-1 edges)
        u = random.randint(0, i - 1)  # Random parent node from previous nodes
        edges1.append([u, i])  # Add edge to the list
    
    # Generate the edges for the second tree (edges2) - ensure edges are sorted and valid
    edges2 = []
    for i in range(1, m):  # Create edges for tree 2 (m-1 edges)
        u = random.randint(0, i - 1)  # Random parent node from previous nodes
        edges2.append([u, i])  # Add edge to the list
    
    # Calculate the maximum depth of tree1 and tree2
    def calculate_max_depth(edges, n):
        # Create adjacency list
        adj = [[] for _ in range(n)]
        for u, v in edges:
            adj[u].append(v)
            adj[v].append(u)

        # DFS to find maximum depth
        def dfs(node, parent):
            max_depth = 0
            for neighbor in adj[node]:
                if neighbor != parent:
                    max_depth = max(max_depth, dfs(neighbor, node) + 1)
            return max_depth
        
        # Start DFS from node 0 (root)
        return dfs(0, -1)

    # Calculate the maximum depth for both trees
    max_depth1 = calculate_max_depth(edges1, n)
    max_depth2 = calculate_max_depth(edges2, m)

    # Random k within the range [0, min(max_depth1 + max_depth2, 1000)]
    k = random.randint(0, min(max_depth1 + max_depth2, 1000))

    # Sorting the edges to ensure they are in the correct order (lexicographically sorted)
    edges1.sort()  # Sort edges for tree 1
    edges2.sort()  # Sort edges for tree 2
    
    return (edges1, edges2, k)


# Function to format the test input for terminal simulation
def format_test_input(test_input):
    edges1, edges2, k = test_input
    
    # Debugging: Print the test case details
    # print(f"Test Case: {len(edges1)+1} nodes in edges1, {len(edges2)+1} nodes in edges2, k = {k}")
    
    # Format the input as required by the C++ program
    formatted_input = f"{len(edges1)}\n"  # Number of edges for tree 1
    for edge in edges1:
        formatted_input += f"{edge[0]} {edge[1]}\n"
    
    formatted_input += f"{len(edges2)}\n"  # Number of edges for tree 2
    for edge in edges2:
        formatted_input += f"{edge[0]} {edge[1]}\n"
    
    formatted_input += f"{k}\n"  # The value of k
    
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
                f.write(f"input:\n{formatted_input}")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
