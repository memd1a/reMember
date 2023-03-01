#!/bin/bash
sea-orm-cli migrate down
sea-orm-cli migrate up

sea-orm-cli generate entity -o src/entities