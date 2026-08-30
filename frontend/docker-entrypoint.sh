#!/bin/sh
set -eu

: "${API_BASE:=}"

envsubst '${API_BASE}' < /usr/share/nginx/html/env.js.template > /usr/share/nginx/html/env.js

exec "$@"
