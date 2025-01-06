import os

examples = [
    {
        "input": "6\n2 8 4 32 16 1\n3\n0 2\n1 4\n0 5",
        "output": "12 60 60"
    },
    {
        "input": "7\n0 7 3 2 8 5 1\n5\n0 3\n1 5\n2 4\n2 6\n5 6",
        "output": "7 14 11 14 5"
    }
]

output_file = "fuzz_outputs/CPP/weekly_contest_413_p4/outputs"

def write_examples_to_file():
    with open(output_file, "r") as f:
        existing_content = f.read()

    example_content = ""
    for example in examples:
        example_content += f"input:\n{example['input']}\noutput:\n{example['output']}\n-------------------------\n"

    with open(output_file, "w") as f:
        f.write(example_content + existing_content)

if __name__ == "__main__":
    write_examples_to_file()