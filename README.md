# vm
Toy vm from scratch to test some stuff. Used as references the following blogs:

- https://blog.subnetzero.io/post/building-language-vm-part-00/
- https://rust-hosted-langs.github.io/book/chapter-interp-vm-design.html
- https://interpreterbook.com/
- https://nnethercote.github.io/perf-book/title-page.html

Its usage is pretty straightforward, just run `cargo run` and start feeding instructions via the CLI. The commands available are the following ones:

- `.program` to show the current program as a `Vec<u8>`
- `.registers` the same for registers
- `.quit` to exit the program
- `A B C D`, where each value is a valid `u8`. The tuple can be repeated sequentially and each four numbers represent an instruction.