import os
import subprocess
import random
import time

# Configure test case generation parameters
test_cases = 100  # Number of test cases to generate
min_points = 1
max_points = 10  # Maximum number of points
coordinate_range = 100  # Range for coordinates (0 <= xi, yi <= 100)

# File Configs
output_file = "../../../fuzz_outputs/C/weekly_contest_427_p2/outputs"  # Output file to store test cases and results
c_folder = "../src"  # Folder containing the C source code
c_file = "weekly_contest_427_p2.c"  # C source file name
executable_name = "solution"  # Executable name

# Generate a single test case
def generate_test_input():
    random.seed(time.time())
    num_points = random.randint(4, 10)  # Minimum of 4 points
    points = []

    case_type = random.choice(["valid_rectangle", "rectangle_with_extra", "invalid"])

    if case_type == "valid_rectangle":
        # Generate a valid rectangle
        x1, y1 = random.randint(0, 50), random.randint(0, 50)
        x2, y2 = random.randint(x1 + 3, 100), random.randint(y1 + 3, 100)
        points = [[x1, y1], [x1, y2], [x2, y1], [x2, y2]]

        # Add random additional points outside the rectangle
        while len(points) < num_points:
            new_point = [random.randint(0, 100), random.randint(0, 100)]
            if new_point not in points:  # Ensure no duplicate points
                points.append(new_point)

    elif case_type == "rectangle_with_extra":
        # Generate a valid rectangle
        x1, y1 = random.randint(0, 50), random.randint(0, 50)
        x2, y2 = random.randint(x1 + 3, 100), random.randint(y1 + 3, 100)
        points = [[x1, y1], [x1, y2], [x2, y1], [x2, y2]]

        # Add random points inside or on the rectangle
        extra_points = [
            [x1, random.randint(y1 + 1, y2 - 1)],
            [x2, random.randint(y1 + 1, y2 - 1)],
        ]
        points.extend(extra_points)

        # Fill with random points
        while len(points) < num_points:
            new_point = [random.randint(0, 100), random.randint(0, 100)]
            if new_point not in points:  # Ensure no duplicate points
                points.append(new_point)

    else:  # case_type == "invalid"
        # Generate random points without forming a valid rectangle
        while len(points) < num_points:
            new_point = [random.randint(0, 100), random.randint(0, 100)]
            if new_point not in points:  # Ensure no duplicate points
                points.append(new_point)

    return points


# Format test_input as a string for terminal input simulation
def format_test_input(test_input):
    formatted_points = "\n".join(f"{x} {y}" for x, y in test_input)
    return f"{len(test_input)}\n{formatted_points}\n"


# Compile the C program
def compile_c():
    try:
        compile_command = ["gcc", os.path.join(c_folder, c_file), "-o", os.path.join(c_folder, executable_name)]
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

                f.write(f"input:\n{formatted_input}")
                f.write(f"output:\n{simulated_output}\n")
                f.write("-------------------------\n")
    finally:
        cleanup()