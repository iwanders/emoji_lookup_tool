# Emoji Lookup Tool

For when you just want to copy paste an emoji and know it is called `party` - something...
```
$ cargo r -- search party
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/emoji_lookup_tool search party`
🎉     PARTY POPPER
...
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


# License
License is [`MIT`](./LICENSE-MIT).
