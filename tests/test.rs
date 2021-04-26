use aki_txpr_macro::*;

fn test_01() -> anyhow::Result<String> {
    use std::io::BufRead;
    //
    let next_in = runnel::medium::stdio::StdIn::default();
    let handles = Vec::new();
    let (next_in, handles) = pipe_line! {
        (next_in, handles);
        libaki_xcat  "xcat" "-n" "-f" "fixtures/target-list.txt";
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

fn test_02() -> anyhow::Result<String> {
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

mod test {
    #[test]
    fn test_1() {
        let r = super::test_01();
        assert_eq!(r.is_ok(), true);
        assert_eq!(
            r.unwrap(),
            "lines:\"85\", bytes:\"4,239\", chars:\"4,239\", words:\"274\", max:\"68\"\n"
        );
    }
    #[test]
    fn test_2() {
        let r = super::test_02();
        assert_eq!(r.is_ok(), true);
        assert_eq!(
            r.unwrap(),
            "lines:\"23\", bytes:\"1,543\", chars:\"1,543\", words:\"81\", max:\"78\"\n"
        );
    }
}
