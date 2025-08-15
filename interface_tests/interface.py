import subprocess
import os

def call_rust_program(input_data_1: str, input_data_2: str) -> str:
    # Defina o caminho para o executável Rust
    rust_program_path = os.path.join("arc_lib", "target", "release", "arc_lib")

    # Run the Rust program with input_data as an argument
    result = subprocess.run(
        [rust_program_path, input_data_1, input_data_2], capture_output=True, text=True
    )
    # Capture the output from the Rust program
    return result.stdout

if __name__ == "__main__":
    input_data_1 = "2"
    input_data_2 = "10_c.txt"
    output = call_rust_program(input_data_1, input_data_2)
    print(f"Output from Rust program: {output}")
