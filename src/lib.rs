//! the more easy to use libaki-*.
//!
//! # Examples
//!
//! ## Example:
//!
//! The pipe line text processing.
//!
//! shell command:
//!
//! ```text
//! aki-xcat -n -f "fixtures/target-list.txt" |\
//! aki-mline -e "gnu" |\
//! aki-stats -a --locale "en"
//! ```
//!
//! convert this to rust:
//!
//! ```text
//!  let (next_in, handles) = pipe_line! {
//!      (next_in, handles);
//!      libaki_xcat  "xcat" "-n" "-f" "fixtures/target-list.txt";
//!      libaki_mline "mline" "-e" "gnu";
//!      libaki_stats "stats" "-a" "--locale" "en";
//!  };
//! ```
//!
//! the full rust example:
//!
//! ```rust
//! fn test_02() -> anyhow::Result<String> {
//!     use aki_txpr_macro::*;
//!     use std::io::BufRead;
//!     //
//!     let next_in = runnel::medium::stdio::StdIn::default();
//!     let handles = Vec::new();
//!     let (next_in, handles) = pipe_line! {
//!         (next_in, handles);
//!         libaki_xcat  "xcat" "-n" "-f" "fixtures/target-list.txt";
//!         libaki_mline "mline" "-e" "gnu";
//!         libaki_stats "stats" "-a" "--locale" "en";
//!     };
//!     // main thread
//!     let string = {
//!         let sout = runnel::medium::stringio::StringOut::default();
//!         #[rustfmt::skip]
//!         let sioe = runnel::RunnelIoeBuilder::new().pin(next_in).pout(sout).build();
//!         for line in sioe.pin().lock().lines() {
//!             let line_s = line?;
//!             let line_ss = line_s.as_str();
//!             #[rustfmt::skip]
//!             sioe.pout().lock().write_fmt(format_args!("{}\n", line_ss))?;
//!         }
//!         #[rustfmt::skip]
//!         let x = sioe.pout().lock().buffer_str().to_string();
//!         x
//!     };
//!     //
//!     for handle in handles {
//!         let _ = handle.join();
//!     }
//!     //
//!     Ok(string)
//! }
//! ```
//!

#[allow(unused_imports)]
use anyhow::*;
use runnel::*;

#[macro_export]
macro_rules! pipe_line {
    (($next_in:expr,$handles:expr) ; $($rest:tt)*) => {{
        $crate::pipe_line!(($next_in,$handles,$crate::_pipe_sz!()) ; $($rest)*)
    }};
    (($next_in:expr,$handles:expr,$psz:expr) ; $($rest:tt)*) => {{
        let next_in = { $next_in };
        let mut handles = { $handles };
        let next_in = $crate::pipe_line!((0,handles,next_in,$psz) $($rest)*);
        //
        (next_in, handles)
    }};
    (($n:expr,$f:ident,$next_in:ident,$psz:expr) $cmd:ident $cmd_nm:literal $($x:expr)* ; $($rest:tt)*) => {{
        let (a_out, a_in) = runnel::medium::pipeio::pipe($psz);
        let handle = $crate::_txtpc!(($n, $next_in, a_out) $cmd $cmd_nm $crate::_txtpc_args!($($x)*));
        $f.push(handle);
        let next_in = pipe_line!(($n+1,$f,a_in,$psz) $($rest)*);
        next_in
    }};
    (($n:expr,$f:ident,$next_in:ident,$psz:expr) $cmd:ident $cmd_nm:literal $($x:expr)* ;) => {{
        let (a_out, a_in) = runnel::medium::pipeio::pipe($psz);
        let handle = $crate::_txtpc!(($n, $next_in, a_out) $cmd $cmd_nm $crate::_txtpc_args!($($x)*));
        $f.push(handle);
        let next_in = a_in;
        next_in
    }};
    (($n:expr,$f:ident,$next_in:ident,$psz:expr)) => {{ $next_in }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _txtpc {
    (($n:expr, $pin:ident, $pout:ident) $cmd:ident $cmd_nm:literal $args:expr) => {
        std::thread::spawn(move || {
            let _n = $n;
            let prog_name = $cmd_nm;
            let sioe = runnel::RunnelIoeBuilder::new()
                .pin($pin)
                .pout($pout)
                .build();
            let args = $args;
            if let Err(err) = $cmd::execute(&sioe, prog_name, args) {
                let _ = write_error(&sioe, prog_name, err);
            }
        })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _txtpc_args {
    () => (
        &[]
    );
    ($($x:expr)+) => (
        &[$($x),+]
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! _pipe_sz {
    () => {
        16
    };
}

pub fn write_error(sioe: &RunnelIoe, prog_name: &str, err: anyhow::Error) -> anyhow::Result<()> {
    let mut p_err = sioe.perr().lock();
    p_err.write_fmt(format_args!("{prog_name}: {err}\n"))?;
    p_err.flush()?;
    Ok(())
}
