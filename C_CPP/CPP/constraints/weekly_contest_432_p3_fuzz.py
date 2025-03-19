import os
import subprocess
import random
import string
import time
from collections import deque, defaultdict

# TODO: Configure test case generation parameters
test_cases = 20  # Number of test cases to generate
max_n = 10**4  # Maximum number of nodes
max_Wi = 10**5  # Maximum weight of an edge

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_432_p3/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_432_p3.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# TODO: Generate a single test case
def generate_test_input():
    random.seed(time.time())
    # 2 <= n <= 10^5, 1 <= threshold <= n - 1, 1 <= edges.length <= min(105, n * (n - 1) / 2)., edges[i].length == 3, 0 <= Ai, Bi < n, Ai != Bi, 1 <= Wi <= 10^6. There may be multiple edges between a pair of nodes, but they must have unique weights.
    # 生成节点数 n 和阈值 threshold
    n = random.randint(2, 10000)
    threshold = random.randint(1, n - 1)
    
    # 计算边的数量 m
    max_m = min(10000, n * (n - 1) // 2)
    m = random.randint(n - 1, max_m)
    
    # 初始化边列表和权重记录
    edges = []
    used_wi = defaultdict(set)
    
    # 生成反向树
    parent = [-1] * n
    visited = [False] * n
    queue = deque([0])
    visited[0] = True
    
    while queue:
        current = queue.popleft()
        unvisited = [i for i in range(n) if not visited[i]]
        random.shuffle(unvisited)
        num_children = random.randint(0, min(len(unvisited), threshold))
        for i in range(num_children):
            child = unvisited[i]
            parent[child] = current
            queue.append(child)
            visited[child] = True
    
    # 添加树边
    for child in range(1, n):
        if parent[child] != -1:
            Ai, Bi = child, parent[child]
            Wi = random.randint(1, 100000)
            while Wi in used_wi[(Ai, Bi)]:
                Wi = random.randint(1, 100000)
            used_wi[(Ai, Bi)].add(Wi)
            edges.append([Ai, Bi, Wi])
    
    # 添加额外边
    additional_m = m - len(edges)
    for _ in range(additional_m):
        while True:
            Ai = random.randint(0, n - 1)
            Bi = random.randint(0, n - 1)
            if Ai != Bi and (Ai, Bi) not in [(e[0], e[1]) for e in edges]:
                Wi = random.randint(1, 100000)
                while Wi in used_wi[(Ai, Bi)]:
                    Wi = random.randint(1, 100000)
                used_wi[(Ai, Bi)].add(Wi)
                edges.append([Ai, Bi, Wi])
                break
    
    return n, threshold, edges

# TODO: Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    n, threshold, edges = test_input
    formatted_input = f"{n} {threshold}\n"
    for edge in edges:
        formatted_input += f"{edge[0]} {edge[1]} {edge[2]}\n"
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
