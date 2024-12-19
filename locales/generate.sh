#!/usr/bin/env sh
(printf "# This file was generated from the localedata in 'github.com/chronotope/pure-rust-locales'.\n"; \
 curl https://api.github.com/repos/chronotope/pure-rust-locales/contents/localedata/locales | jq -r '.[].name') \
 | sed '/^translit_/d; /^iso/d; s/@/_/g' \
 > "$(dirname "$(realpath "$0")")/pure-rust-locales.txt"
