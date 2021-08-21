#!/bin/bash

# Script to setup and configure marvin daemon with default settings

echo "Starting up marvin..."

echo "Creating marvin directory under /etc"
mkdir /etc/marvin

echo "Copying config file to /etc/marvin/marvin.conf"

cp marvin.conf /etc/marvin/marvin.conf