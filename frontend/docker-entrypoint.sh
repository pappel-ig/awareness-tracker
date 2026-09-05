#!/bin/sh
set -eu

: "${API_BASE:=}"
: "${TURNSTILE_SITE_KEY:=}"

envsubst '${API_BASE} ${TURNSTILE_SITE_KEY}' < /usr/share/nginx/html/env.js.template > /usr/share/nginx/html/env.js

exec "$@"
