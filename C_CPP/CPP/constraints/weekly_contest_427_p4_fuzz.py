import os
import subprocess
import random

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
min_points = 4
max_points = 20000
max_coord = 8000000

# File Configs
output_file = "../../../fuzz_outputs/CPP/weekly_contest_427_p4/outputs"  # Output file to store test cases and results
cpp_folder = "../src"  # Folder containing the C++ source code
cpp_file = "weekly_contest_427_p4.cpp"  # C++ source file name
executable_name = "solution"  # Executable name

# Generate a single test case
def generate_test_input():
    n = random.randint(min_points, max_points)

    points = set()
    # Ensure at least 50% of cases contain valid rectangles
    if random.random() < 0.5:
        x1, y1 = random.randint(0, max_coord / 2), random.randint(0, max_coord / 2)
        width, height = random.randint(1, 100), random.randint(1, 100)
        x2, y2 = x1 + width, y1
        x3, y3 = x1, y1 + height
        x4, y4 = x1 + width, y1 + height

        points.update([(x1, y1), (x2, y2), (x3, y3), (x4, y4)])

        # Add random points outside the rectangle
        while len(points) < n:
            px, py = random.randint(0, max_coord), random.randint(0, max_coord)
            if not (x1 < px < x4 and y1 < py < y3):  # Exclude points inside the rectangle
                points.add((px, py))
    else:
        # Fully random points
        while len(points) < n:
            px, py = random.randint(0, max_coord), random.randint(0, max_coord)
            points.add((px, py))

    xCoord, yCoord = zip(*points)
    return list(xCoord), list(yCoord)

# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    xCoord, yCoord = test_input
    n = len(xCoord)
    formatted_input = f"{n}\n"
    formatted_input += "\n".join(f"{xCoord[i]} {yCoord[i]}" for i in range(n))
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
    formatted_input = format_test_input(test_input)
    try:
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

                f.write(f"input:\n{formatted_input}\n")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()
