# emotif___
An emojified dialect of brainf**k written in Rust.

Created By [Romulus10](https://github.com/Romulus10) and [InsidiousMind](https://github.com/InsidiousMind)

## Brainf**k
Brainf**k is an esoteric, Turing-complete programming language created in 1993 by Urban Müller. It consists of 8 commands and an instruction pointer. Usually, the commands are used to move along a single array of individual bytes and increment, decrement, input, and output.

## Emotif**k
This is our own special take on brainf**k - instead of the traditional commands, it uses UTF-8 emojis. The commands are as follows:

| Emoji | UTF-8 Code Points |                 Unicode                  |    Meaning    | Brainf**k Equivalent |
| :---: | :---------------: | :--------------------------------------: | :-----------: | :------------------: |
|  🔥   |    F0 9F 94 A5    | [U+1F525](https://apps.timwhitlock.info/unicode/inspect/hex/1F525) |  Move Right   |  >   |
|  💯   |    F0 9F 92 AF    | [U+1F4AF](http://www.unicode.org/emoji/charts/full-emoji-list.html#1f4af) |   Move Left   |  <   |
|  💩   |    F0 9F 92 A9    | [U+1F4A9](http://www.unicode.org/emoji/charts/full-emoji-list.html#1f4a9) |   Decrement   |  -   |
|  👍   |    F0 9F 91 8D    | [U+1F44D](http://www.unicode.org/emoji/charts/full-emoji-list.html#1f44d) |   Increment   |  +   |
|  💞   |    F0 9F 92 9E    | [U+1F49E](http://www.unicode.org/emoji/charts/full-emoji-list.html#1f49e) |    Output     |  .   |
|  🙏   |    F0 9F 99 8F    | [U+1F64F](http://www.unicode.org/emoji/charts/full-emoji-list.html#1f64f) |     Input     |  ,   |
|  🌚   |    F0 9F 8C 9A    | [U+1F31A](https://apps.timwhitlock.info/unicode/inspect/hex/1F31A) | Jump Forward  |  [   |
|  🐸   |    F0 9F 90 B8    | [U+1F438](https://apps.timwhitlock.info/unicode/inspect/hex/1F438) | Jump Backward |  ]   |

Programs are represented by strings of these emojis in text files. In brainf**k, a Hello World program might look something like this:


`++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.`

The emotif\*\*k equivalent is:

`👍👍👍👍👍👍👍👍🌚🔥👍👍👍👍🌚🔥👍👍🔥👍👍👍🔥👍👍👍🔥👍💯💯💯💯💩🐸🔥👍🔥👍🔥💩🔥🔥👍🌚💯🐸💯💩🐸🔥🔥💞🔥💩💩💩💞👍👍👍👍👍👍👍💞💞👍👍👍💞🔥🔥💞💯💩💞💯💞👍👍👍💞💩💩💩💩💩💩💞💩💩💩💩💩💩💩💩💞🔥🔥👍💞🔥👍👍💞`

*"The only reason someone would do something like this if they could, which they can't, would be because they could, which they can't."* - Rick Sanchez
