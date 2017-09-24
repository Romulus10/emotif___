# emotif___
An emojified dialect of brainf**k written in Rust.

## Brainf**k
Brainf**k is an esoteric programming language created in 1993 by Urban Muller. It consists of 8 commands and an instruction pointer.

## Emotif**k
This is our own special take on brainf**k - instead of the traditional commands, it uses UTF-8 emojis. The commands are as follows:

|        Emoji         | UTF-8 Code Points |                 Unicode                  | Meaning  |
| :------------------: | :---------------: | :--------------------------------------: | :------: |
| :fire:               |    F0 9F 94 A5    | [U+1F525](https://apps.timwhitlock.info/unicode/inspect/hex/1F525) |  Move Right   |
| :100:                |    F0 9F 92 AF    | [U+1F4AF](http://www.unicode.org/emoji/charts/full-emoji-list.html#1f4af) |   Move Left   |
| :poop:               |    F0 9F 92 A9    | [U+1F4A9](http://www.unicode.org/emoji/charts/full-emoji-list.html#1f4a9) |   Decrement   |
| :thumbsup:           |    F0 9F 91 8D    | [U+1F44D](http://www.unicode.org/emoji/charts/full-emoji-list.html#1f44d) |   Increment   |
| :revolving_hearts:   |    F0 9F 92 9E    | [U+1F49E](http://www.unicode.org/emoji/charts/full-emoji-list.html#1f49e) |    Output     |
| :pray:               |    F0 9F 99 8F    | [U+1F64F](http://www.unicode.org/emoji/charts/full-emoji-list.html#1f64f) |     Input     |
| :new_moon_with_face: |    F0 9F 8C 9A    | [U+1F31A](https://apps.timwhitlock.info/unicode/inspect/hex/1F31A) | Jump Forward  |
| :frog:               |    F0 9F 90 B8    | [U+1F438](https://apps.timwhitlock.info/unicode/inspect/hex/1F438) | Jump Backward |

Programs are represented by strings of these emojis in text files. In brainf**k, a Hello World program might look something like this:


`++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.`
👍👍👍👍👍👍👍👍👍👍🌚🔥👍👍👍👍👍👍👍🔥👍👍👍👍👍👍👍👍👍👍🔥👍👍👍🔥👍💯💯💯💯💩🐸🔥👍👍💞🔥👍💞👍👍👍👍👍👍👍💞💞👍👍👍💞🔥👍👍💞💯💯👍👍👍👍👍👍👍👍👍👍👍👍👍👍👍💞🔥💞👍👍👍💞💩💩💩💩💩💩💞💩💩💩💩💩💩💩💩💞🔥👍💞🔥💞
