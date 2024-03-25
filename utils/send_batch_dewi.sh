#!/bin/bash

# Path to your CSV file
CSV_FILE=$1
PROJECT_ID=74

# Calculate the total number of lines to process (minus the header)
TOTAL_LINES=$(($(wc -l < "$CSV_FILE") - 1))

# Current line number
CURRENT_LINE=0

# Skip the header line
tail -n +2 "$CSV_FILE" | while IFS=, read -r device_id connections; do
  # Increment the current line counter
  ((CURRENT_LINE++))

  # Construct the JSON data
  JSON_DATA="{\"device_id\":$device_id,\"connections\":$connections,\"receipt_type\":\"Snark\"}"
  echo $JSON_DATA

  # Execute your command here, replace the actual JSON data with the constructed one
  ioctl-unstable ws message send --project-id "$PROJECT_ID" --project-version "0.1" --data "$JSON_DATA" 

  # Calculate and output the percentage of completion
  PERCENTAGE=$(echo "scale=2; $CURRENT_LINE / $TOTAL_LINES * 100" | bc)
  echo "Sent to Project $PROJECT_ID: $CURRENT_LINE of $TOTAL_LINES, $PERCENTAGE% completed."

  # Optional: Wait a bit between commands if needed to avoid overwhelming the server or getting rate-limited
  sleep 0.25
done
