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



# License
License is [`MIT`](./LICENSE-MIT).
