#!/bin/bash
# 🔧 Quick Fix Script - Addressing Compilation Issues! 🔧

echo "🚢 Applying quick fixes to get Feedbacker compiling..."

# Allow unused imports and variables for now
sed -i '' '1i\
#![allow(unused_imports)]\
#![allow(unused_variables)]\
#![allow(dead_code)]\
' src/main.rs

echo "✅ Applied compilation fixes!"
echo "🚀 Now run: cargo build"