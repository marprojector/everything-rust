#!/bin/sh
# Renders docs/assets/social-preview.png from docs/assets/social.html with the current star and fork counts.
set -eu
cd "$(dirname "$0")/../docs/assets"
counts=$(gh repo view marprojector/everything-rust --json stargazerCount,forkCount --jq '"\(.stargazerCount) \(.forkCount)"')
stars=$(printf "%s" "$counts" | cut -d' ' -f1 | python3 -c 'import sys; print(f"{int(sys.argv[1]):,}")' "$(cat)")
forks=$(printf "%s" "$counts" | cut -d' ' -f2 | python3 -c 'import sys; print(f"{int(sys.argv[1]):,}")' "$(cat)")
sed -e "s/{{stars}}/$stars/" -e "s/{{forks}}/$forks/" social.html > .social.render.html
chrome="${CHROME:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
"$chrome" --headless=new --disable-gpu --hide-scrollbars --force-device-scale-factor=1 \
  --window-size=1280,640 --screenshot="$PWD/social-preview.png" "file://$PWD/.social.render.html" 2>/dev/null
rm -f .social.render.html
echo "social-preview.png: $stars stars, $forks forks"
