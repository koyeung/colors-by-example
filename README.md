# colors-by-example

Select [ANSI-256 colors](https://en.wikipedia.org/wiki/ANSI_escape_code#Colors) on terminal emulator by color indexes example on perceived lightness, chroma or hue.

Features:
- Find similar colors by [Oklch](https://bottosson.github.io/posts/oklab/) attributes
- Oklch attributes are provided by example ANSI-256 color indexes
- Predefined system dependent [4-bit colors](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit) for common platforms, e.g. Terminal.app
- Show sample text with selected colors as foreground or background

## Usage

- Find colors with similar hue to color `148`, sort by hue: `select_colors -H148  |sort -g -k8`
- Show more similar colors by using `--hue-extent`: `select_colors -H148 --hue-extent 5 |sort -g -k8`

  ![Colors similar to color 148 by hue](/assets/images/h148.png)

- Show colors opposite in hue to the reference color: `select_colors -H148 --hue-extent 5 --opposite|sort -g -k8`

  ![Colors opposite to color 148 by hue](/assets/images/h148_opposite.png)

- Colors with similar lightness to color `148`: `select_colors -L148|sort -g -k8`

  ![Colors similar to color 148 by lightness](/assets/images/l148.png)

- Colors similar or brighter than color `148`: `select_colors -L148 --brighten true |sort -g -k8`

  ![Colors similar or brighter than color 148](/assets/images/l148_brighten.png)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
