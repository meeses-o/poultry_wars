#!/bin/bash

# Check if arguments are provided
if [ $# -lt 2 ]; then
    echo "Usage: $0 <receiver_ip> <message>"
    exit 1
fi

RECEIVER_IP=$1
MESSAGE=$2
PORT=3000

echo "$MESSAGE" | nc -w 1 $RECEIVER_IP $PORT

if [ $? -eq 0 ]; then
    echo "Message sent successfully to $RECEIVER_IP"
else
    echo "Failed to send message. Is the receiver listening?"
fi