@echo off

cd "C:\Users\aryan\Documents\Rust\Euler Angles\eulermat"

cls

echo "Enter rotations (euler matrix followed by angle):"
echo.
set /p args=""


target\release\eulermat.exe %args%

cd %USERPROFILE%




