cd isolation-lib
cargo build --release
cd ..
move "isolation-lib\target\release\isolation_lib.dll" "Isolation-GoDot\Bin"
echo success
pause