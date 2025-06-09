#![deny(rust_2018_idioms)]

use lazy_static::lazy_static;

use regex::Captures;
use regex::Regex;
use rustc_demangle::demangle;
use std::io;
use std::io::prelude::*;

fn demangle_line(no_verbose: bool, line: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[_a-zA-Z$][_a-zA-Z$0-9.]*").unwrap();
    }

    RE.replace_all(line, |caps: &Captures<'_>| {
        let demangled = demangle(caps.get(0).unwrap().as_str());
        if no_verbose {
            format!("{:#}", demangled)
        } else {
            format!("{}", demangled)
        }
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::demangle_line;

    #[test]
    fn passes_text() {
        assert_eq!(
            demangle_line(true, "mo fo\tboom      hello  "),
            "mo fo\tboom      hello  "
        );
    }

    #[test]
    fn demangles() {
        assert_eq!(
            demangle_line(true, "_ZN7example4main17h0db00b8b32acffd5E:"),
            "example::main:"
        );
    }

    #[test]
    fn handles_mid_demangling() {
        assert_eq!(
            demangle_line(true, "        lea     rax, [rip + _ZN55_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h510ed05e72307174E]"),
                "        lea     rax, [rip + <&\'a T as core::fmt::Display>::fmt]",
        );
    }

    #[test]
    fn handles_call_plt() {
        assert_eq!(
            demangle_line(true, "        call    _ZN3std2io5stdio6_print17he48522be5b0a80d9E@PLT"),
            "        call    std::io::stdio::_print@PLT"
        );
    }

    #[test]
    fn includes_hash_in_verbose_mode() {
        assert_eq!(
            demangle_line(false, "_ZN4core3fmt9Arguments9new_const17hf7eafdf6c5e03508E"),
            "core::fmt::Arguments::new_const::hf7eafdf6c5e03508",
        );
    }

    #[test]
    fn demangles_v0_symbols() {
        assert_eq!(
            demangle_line(true, "_RNvCslMLAjg8TrSC_7example4meow"),
            "example::meow",
        );
    }

    #[test]
    fn demangles_double_underscore_prefixed_symbols() {
        assert_eq!(
            demangle_line(true, "__RINvMs2_NtCslVVlGMZyABT_4core3fmtNtB6_9Arguments9new_constKj1_ECslMLAjg8TrSC_7example"),
            "<core::fmt::Arguments>::new_const::<1>",
        );
        assert_eq!(
            demangle_line(true, "__ZN4core3fmt9Arguments9new_const17hf7eafdf6c5e03508E"),
            "core::fmt::Arguments::new_const",
        );
    }
}

fn main() {
    let no_verbose = std::env::args().any(|arg| arg == "--no-verbose");
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        println!("{}", demangle_line(no_verbose, &line.unwrap()));
    }
}
