use aki_txpr_macro::*;

fn test_01() -> anyhow::Result<String> {
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
        let sioe = runnel::RunnelIoeBuilder::new().pg_in(next_in).pg_out(sout).build();
        for line in sioe.pg_in().lines() {
            let line_s = line?;
            let line_ss = line_s.as_str();
            #[rustfmt::skip]
            sioe.pg_out().lock().write_fmt(format_args!("{line_ss}\n"))?;
        }
        #[rustfmt::skip]
        let x = sioe.pg_out().lock().buffer_to_string();
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
    let next_in = runnel::medium::stdio::StdIn::default();
    let handles = Vec::new();
    let (next_in, handles) = pipe_line! {
        (next_in, handles);
        libaki_xcat  "xcat" "-n" "-f" "fixtures/target-list.txt";
        libaki_mline "mline" "-e" "gnu" "--color" "never";
        libaki_stats "stats" "-a" "--locale" "en";
    };
    // main thread
    let string = {
        let sout = runnel::medium::stringio::StringOut::default();
        #[rustfmt::skip]
        let sioe = runnel::RunnelIoeBuilder::new().pg_in(next_in).pg_out(sout).build();
        for line in sioe.pg_in().lines() {
            let line_s = line?;
            let line_ss = line_s.as_str();
            #[rustfmt::skip]
            sioe.pg_out().lock().write_fmt(format_args!("{line_ss}\n"))?;
        }
        #[rustfmt::skip]
        let x = sioe.pg_out().lock().buffer_to_string();
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
        assert!(r.is_ok());
        assert_eq!(
            r.unwrap(),
            "lines:\"85\", bytes:\"4,239\", chars:\"4,239\", words:\"274\", max:\"68\"\n"
        );
    }
    #[test]
    fn test_2() {
        let r = super::test_02();
        assert!(r.is_ok());
        assert_eq!(
            r.unwrap(),
            "lines:\"23\", bytes:\"1,290\", chars:\"1,290\", words:\"81\", max:\"67\"\n"
        );
    }
}
