#!/bin/bash -xe

# We don't use a submodule because the repo sizes for both cldr and iamcal/emoji-data are large.


cd thirdparty/
# https://github.com/iamcal/emoji-data/blob/v16.0.0/emoji.json
curl https://raw.githubusercontent.com/iamcal/emoji-data/v16.0.0/emoji.json  -o iamcal_emoji-data_emoji.json
# https://github.com/iamcal/emoji-data/blob/v16.0.0/emoji_pretty.json
curl https://raw.githubusercontent.com/iamcal/emoji-data/v16.0.0/emoji_pretty.json  -o iamcal_emoji-data_emoji_pretty.json
