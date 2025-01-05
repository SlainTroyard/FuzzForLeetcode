import os

examples = [
    {
        "input": "a1\nc3\n",
        "output": "true"
    },
    {
        "input": "a1\nh3\n",
        "output": "false"
    }
]

output_file = "fuzz_outputs/CPP/weekly_contest_413_p1/outputs"

def write_examples_to_file():
    with open(output_file, "r") as f:
        existing_content = f.read()

    example_content = ""
    for example in examples:
        example_content += f"input:\n{example['input']}output:\n{example['output']}\n-------------------------\n"

    with open(output_file, "w") as f:
        f.write(example_content + existing_content)

if __name__ == "__main__":
    write_examples_to_file()