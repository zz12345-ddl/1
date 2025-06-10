@echo off
chcp 65001 >nul
echo ====================================
echo PakePlus Docker Build Script
echo ====================================
echo.

echo Building Docker image...
docker build -t pakeplus-builder .
if %errorlevel% neq 0 (
    echo Docker image build failed!
    pause
    exit /b 1
)

echo.
echo Docker image built successfully!
echo.
echo Starting container to build...
docker run --rm -v "%cd%\output:/app/src-tauri/target" pakeplus-builder
if %errorlevel% neq 0 (
    echo Build failed!
    pause
    exit /b 1
)

echo.
echo ====================================
echo Build completed!
echo Build files located at: output\release\bundle\
echo ====================================
pause