#!/bin/bash

cargo doc -p c-ares-resolver
echo "<meta http-equiv=refresh content=0;url=c_ares_resolver/index.html>" > target/doc/index.html

git config user.name "Travis CI"
~/.local/bin/ghp-import -n target/doc

openssl aes-256-cbc -K "$encrypted_25079192cbe2_key" -iv "$encrypted_25079192cbe2_iv" -in publish-key.enc -out ~/.ssh/publish-key -d
chmod u=rw,og= ~/.ssh/publish-key
echo "Host github.com" >> ~/.ssh/config
echo "  IdentityFile ~/.ssh/publish-key" >> ~/.ssh/config
git remote set-url origin git@github.com:dimbleby/c-ares-resolver.git
git push origin +gh-pages
