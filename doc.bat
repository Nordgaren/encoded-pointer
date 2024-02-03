cargo doc --no-deps -p "encoded-pointer" --all-features
rmdir /s ./docs
robocopy target/doc docs /s
echo|set /p="<meta http-equiv="refresh" content="0; url=encoded_pointer/index.html">" > docs/index.html