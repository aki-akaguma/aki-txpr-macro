# aki-txpr-macro

*aki-txpr-macro* is the more easy to use libaki-*.

## Features

- old style rust macro
- multi-threaded libaki-*
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)


## Examples

### Example:

The pipe line text processing.

shell command:

```
aki-xcat -n -f "fixtures/target-list.txt" |\
aki-mline -e "gnu" |\
aki-stats -a --locale "en"
```

convert this to rust code:

```
 let (next_in, handles) = pipe_line! {
     (next_in, handles);
     libaki_xcat  "xcat" "-n" "-f" "fixtures/target-list.txt";
     libaki_mline "mline" "-e" "gnu";
     libaki_stats "stats" "-a" "--locale" "en";
 };
```

the full rust example:

```rust
fn test_02() -> anyhow::Result<String> {
    use aki_txpr_macro::*;
    use std::io::BufRead;
    //
    let next_in = runnel::medium::stdio::StdIn::default();
    let handles = Vec::new();
    let (next_in, handles) = pipe_line! {
        (next_in, handles);
        libaki_xcat  "xcat" "-n" "-f" "fixtures/target-list.txt";
        libaki_mline "mline" "-e" "gnu";
        libaki_stats "stats" "-a" "--locale" "en";
    };
    // main thread
    let string = {
        let sout = runnel::medium::stringio::StringOut::default();
        #[rustfmt::skip]
        let sioe = runnel::RunnelIoeBuilder::new().pin(next_in).pout(sout).build();
        for line in sioe.pin().lock().lines() {
            let line_s = line?;
            let line_ss = line_s.as_str();
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
        }
        #[rustfmt::skip]
        let x = sioe.pout().lock().buffer_str().to_string();
        x
    };
    //
    for handle in handles {
        let _ = handle.join();
    }
    //
    Ok(string)
}
```

## libaki-*

| command | description |
|:--------|:------------|
| [aki-gsub]   | substitude text command, replace via regex. |
| [aki-mcolor] | mark up text with color |
| [aki-mcycle] | mark up text with cycling color |
| [aki-mline]  | match line, regex text filter like a grep of linux command. |
| [aki-resort] | sort lines of text. You can use regex to specify the KEY. |
| [aki-stats]  | output the statistics of text, like a wc of linux command. |
| [aki-unbody] | output first or last n lines, like a head and tail of linux command. |
| [aki-xcat]   | concatenate files that are plain, gzip, xz and zstd. |
| [aki-xtee]   | copy standard input to each files and standard output. |

[aki-gsub]:https://crates.io/crates/aki-gsub
[aki-mcolor]:https://crates.io/crates/aki-mcolor
[aki-mcycle]:https://crates.io/crates/aki-mcycle
[aki-mline]:https://crates.io/crates/aki-mline
[aki-resort]:https://crates.io/crates/aki-resort
[aki-stats]:https://crates.io/crates/aki-stats
[aki-unbody]:https://crates.io/crates/aki-unbody
[aki-xcat]:https://crates.io/crates/aki-xcat
[aki-xtee]:https://crates.io/crates/aki-xtee
