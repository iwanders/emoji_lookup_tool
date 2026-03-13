# Emoji Lookup Tool

For when you just want to copy paste an emoji and know it is called `party` - something...
```
$ ./target/release/emoji_lookup_tool search party
🎉     party popper
👯‍♀️     women with bunny ears
👯‍♂️     men with bunny ears
🥳     face with party horn and party hat
$ ./target/release/emoji_lookup_tool search :party_
🎉     party popper
🥳     face with party horn and party hat
$ time ./target/release/emoji_lookup_tool search :party_popper:
🎉     party popper

real	0m0.009s


```

Uses data from [https://github.com/iamcal/emoji-data](https://github.com/iamcal/emoji-data), so slack short names, but
search also matches against the unicode `name` field.
This json is embedded in the binary (retrieve it prior to build with `retrieve_thirdparty.sh`).

It also has a subcommand to retrieve images from the [noto-emoji](https://github.com/googlefonts/noto-emoji) font.
By default this retrieves the highest quality png and writes it to `/tmp`, these are not embedded in the binary and retrieved
over  https.

```
$ emoji_lookup_tool noto 🎉
🎉 -> UnicodePoints { code_points: [127881] }
  https://raw.githubusercontent.com/googlefonts/noto-emoji/refs/heads/main/png/512/emoji_u1f389.png
  Writing to "/tmp/emoji_u1f389.png" done!
```
Also supports `emoji_lookup_tool noto :party_popper:`, or search terms there.


And an info command:
```
$ ./target/release/emoji_lookup_tool info :party_popper:
🎉
           emoji: 🎉
  codepoints dec: [127881]
  codepoints hex: [1f389]
  escaped hex   : \u1f389
        noto png: https://raw.githubusercontent.com/googlefonts/noto-emoji/refs/heads/main/png/512/emoji_u1f389.png
        noto svg: https://raw.githubusercontent.com/googlefonts/noto-emoji/refs/heads/main/svg/emoji_u1f389.svg
            name: PARTY POPPER
        category: Activities
     subcategory: event
      short_name: tada
           slack: :tada:

```




# License
License is [`MIT`](./LICENSE-MIT).
