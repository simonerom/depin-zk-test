#!/bin/bash

# Path to your CSV file
CSV_FILE="messages.csv"
PROJECT_ID=17

# Calculate the total number of lines to process (minus the header)
TOTAL_LINES=$(($(wc -l < "$CSV_FILE") - 1))

# Current line number
CURRENT_LINE=0

# Skip the header line
tail -n +2 "$CSV_FILE" | while IFS=, read -r device_id signature latitude longitude receipt_type; do
  # Increment the current line counter
  ((CURRENT_LINE++))

  # Construct the JSON data
  JSON_DATA="{\"device_id\":$device_id,\"signature\":$signature,\"latitude\":$latitude,\"longitude\":$longitude, \"receipt_type\":\"Snark\"}"

  # Execute your command here, replace the actual JSON data with the constructed one
  ioctl-unstable ws message send --project-id "$PROJECT_ID" --project-version "0.1" --data "$JSON_DATA" --did-vc-token $DID_TOKEN

  # Calculate and output the percentage of completion
  PERCENTAGE=$(echo "scale=2; $CURRENT_LINE / $TOTAL_LINES * 100" | bc)
  echo "Processing: $CURRENT_LINE of $TOTAL_LINES, $PERCENTAGE% completed."

  # Optional: Wait a bit between commands if needed to avoid overwhelming the server or getting rate-limited
  sleep 0.25
done