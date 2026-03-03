# generate_rules_fn.py

import sys

def main():
    if len(sys.argv) < 2:
        print("Usage: python generate_rules_fn.py <file.rs>")
        sys.exit(1)

    file_path = sys.argv[1]
    structs = []

    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                # Find public structs
                if line.startswith("pub struct "):
                    parts = line.split()
                    if len(parts) >= 3:
                        struct_name = parts[2].split('{')[0]  # remove '{' if inline
                        struct_name = struct_name.rstrip(';')  # remove semicolon
                        structs.append(struct_name)
    except FileNotFoundError:
        print(f"File not found: {file_path}")
        sys.exit(1)
    except Exception as e:
        print(f"Error reading file: {e}")
        sys.exit(1)

    # Generate Rust function
    print("pub fn rules() -> Vec<Box<dyn Rule>> {")
    print("    vec![")
    for s in structs:
        print(f"        Box::new({s}),")
    print("    ]")
    print("}")

if __name__ == "__main__":
    main()