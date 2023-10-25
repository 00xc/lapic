lapic
=====

A self-contained crate implementing safe types for the local APIC
registers on x86_64 systems.

More information on the local APIC standard can be obtained on
chapter 16: Local APIC from the
[AMD Architecture Programmer's Manual Vol. 2: System Programming](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24593.pdf).

This crate does not depend on the standard Rust library and uses
[`#![forbid(unsafe_code)]`](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html#how-safe-and-unsafe-interact).

Usage
-----

Refer to the generated cargo documentation.
