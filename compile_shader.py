import os
import sys
import subprocess

def compile_shaders():
    print("Compiling shaders...")
    for root, dirs, files in os.walk("src"):
        for file in files:
            if file.endswith(".glsl"):
                print("Compiling " + file)
                file_path = os.path.join(root, file)
                output_file = file_path.replace(".glsl", ".rs")
                shdc_path = "sokol-shdc"
                if sys.platform == "win32":
                    # get running directory
                    running_dir = os.path.dirname(os.path.abspath(__file__))
                    shdc_path = os.path.join(running_dir, "sokol-tools-bin/bin/win32/sokol-shdc.exe")
                    print("shdc_path: " + shdc_path)
                elif sys.platform == "linux":
                    running_dir = os.path.dirname(os.path.abspath(__file__))
                    shdc_path = os.path.join(running_dir, "sokol-tools-bin/bin/linux/sokol-shdc")
                    print("shdc_path: " + shdc_path)
                elif sys.platform == "darwin":
                    running_dir = os.path.dirname(os.path.abspath(__file__))
                    shdc_path = os.path.join(running_dir, "sokol-tools-bin/bin/osx/sokol-shdc")
                    print("shdc_path: " + shdc_path)

                subprocess.call([shdc_path, "-i", file_path, "-o", output_file, "-l", "glsl330:metal_macos:hlsl4", "-f", "sokol_rust"])
                print("Done!")

if __name__ == "__main__":
    compile_shaders()