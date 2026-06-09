#This script reads the entirety of the assets folder and returns all files as a Vec<String> let statement._
#Created by Oliver

import os
from pathlib import Path

def generate_rust_asset_list(assets_dir="assets"):
    base_path = Path(__file__).parent.parent / assets_dir
    
    asset_paths = []

    for root, _, files in os.walk(base_path):
        for file in files:
            if file.startswith('.') or file.endswith(".ogg"):
                continue
            full_path = Path(root) / file
            relative_path = full_path.relative_to(base_path.parent)
            asset_paths.append(f'        "{relative_path.as_posix()}"')

    formatted_paths = ",\n".join(asset_paths)
    rust_code = f"let all_assets = vec![\n{formatted_paths}\n    ];"
    
    return rust_code

if __name__ == "__main__":
    try:
        result = generate_rust_asset_list()
        print(result)
    except FileNotFoundError:
        print("Error: 'assets' folder not found. Ensure you are running this from the poultry_wars directory.")