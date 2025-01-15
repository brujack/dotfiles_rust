#!/bin/bash

# Get the current date in the format yyyymmdd
DATE=$(date +"%Y%m%d")

# Check if any existing tags match today's date and increment
EXISTING_TAGS=$(git tag | grep "^${DATE}")
if [ -z "$EXISTING_TAGS" ]; then
    NEW_TAG="${DATE}-1"
else
    MAX_INCREMENT=$(echo "$EXISTING_TAGS" | awk -F '-' '{print $2}' | sort -nr | head -n1)
    NEW_INCREMENT=$((MAX_INCREMENT + 1))
    NEW_TAG="${DATE}-${NEW_INCREMENT}"
fi

# Create the new tag
git tag -a "$NEW_TAG" -m "Release for $NEW_TAG"
echo "Created tag: $NEW_TAG"

# Update the 'latest' tag to point to the new release
git tag -f latest
echo "Updated 'latest' tag to point to $NEW_TAG"

# Push tags to the remote repository
git push origin "$NEW_TAG"
git push origin -f latest
