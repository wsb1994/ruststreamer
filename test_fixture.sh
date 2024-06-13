#!/bin/bash

# cargo run && bash ./test_fixture.sh

# Define colors for output
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Remove all .mkv files
rm *.mkv

# Function to check the existence of a file
check_file_existence() {
    local filename=$1
    if [ -f "$filename" ]; then
        return 0
    else
        return 1
    fi
}

# Curl to start streaming to output.mkv
curl -X POST http://localhost:8000/start -H "filename: output.mkv"
# Wait for 3 seconds
sleep 3
# Check for the file output.mkv
if check_file_existence "output.mkv"; then
    echo "\n"
    echo "output.mkv present and accounted for."
else
    echo "\n"
    echo -e "output.mkv does not exist."
    exit 1
fi

# Curl to start streaming to different.mkv
curl -X POST http://localhost:8000/start -H "filename: different.mkv"
# Wait for 3 seconds
sleep 3
# Check for the file different.mkv
if check_file_existence "different.mkv"; then
    echo "\n"
    echo "different.mkv present and accounted for"
else
    echo "\n"
    echo -e "different.mkv does not exist."
    exit 1
fi

# If all checks passed
echo -e "${GREEN}[success]${NC} [All files are present as expected]"
echo -e "${GREEN}[success]${NC} [cleaning up]"
rm *.mkv
echo -e "${GREEN}[success]${NC} [Test fixture finished, all files cleaned up]"
exit 0
