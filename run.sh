#!/bin/bash
deno run --allow-read --allow-net https://deno.land/std@0.98.0/http/file_server.ts
# Access to http://0.0.0.0:4507/public
