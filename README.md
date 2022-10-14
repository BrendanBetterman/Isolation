# Isolation
Game of isolation

## Dependencies

llvm needs to be installed to compile rust to gdnative - Windows, Mac, Linux
- https://github.com/llvm/llvm-project/releases/tag/llvmorg-15.0.2

LLVM might need Visual Studio Preview - Windows
- https://visualstudio.microsoft.com/vs/preview/#download-preview
- Install the launcher, Community is fine, Select either modify or install on the top option.
Go down to Desktop Development with c++ and on the side bar select C++ Clang tools for windows

The Version of GoDot need is 3.5.1 or newer
- https://godotengine.org/download

To compile the files
---------
cargo build --release
move the .dll, .so or non extenision files in the target/release folder to the bin in the godot folder
