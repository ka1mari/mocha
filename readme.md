p100

```bash
> cargo run --target x86_64-unknown-linux-mocha.json -Zbuild-std=core --release --package hello-world
    Finished release [optimized] target(s) in 0.02s
     Running `target/x86_64-unknown-linux-mocha/release/hello-world`
hello world!
mocha: panic: panicked at 'called `Option::unwrap()` on a `None` value', crates/hello-world/src/main.rs:30:18

> l target/x86_64-unknown-linux-mocha/release/hello-world
0755 10k 1 1 target/x86_64-unknown-linux-mocha/release/hello-world

> strace -f target/x86_64-unknown-linux-mocha/release/hello-world
execve("target/x86_64-unknown-linux-mocha/release/hello-world", ["target/x86_64-unknown-linux-moch"...], 0x7ffcccc40148 /* 47 vars */) = 0
write(1, "hello world!\n", 13hello world!
)          = 13
write(2, "\33[38;5;9mmocha:\33[m panic: panick"..., 122mocha: panic: panicked at 'called `Option::unwrap()` on a `None` value', crates/hello-world/src/main.rs:30:18
) = 122
exit_group(1)                           = ?
+++ exited with 1 +++
```
