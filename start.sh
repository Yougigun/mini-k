#!/bin/bash
clear
docker-compose -f docker-compose.yml up -d --build --force-recreate --remove-orphans
docker-compose -f docker-compose.yml logs -f
