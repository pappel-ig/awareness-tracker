#!/bin/sh
set -eu

: "${API_BASE:=}"
: "${TURNSTILE_SITE_KEY:=}"
: "${BACKEND_UPSTREAM:=}"

envsubst '${API_BASE} ${TURNSTILE_SITE_KEY}' < /usr/share/nginx/html/env.js.template > /usr/share/nginx/html/env.js
envsubst '${BACKEND_UPSTREAM}' < /etc/nginx/templates/default.conf.template > /etc/nginx/conf.d/default.conf

exec "$@"
