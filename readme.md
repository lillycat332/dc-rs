# dc.rs

dc.rs is a reverse Polish notation (postfix) calculator, roughly akin to the unix "dc" command, written in Rust.
It lacks many of the more advanced features of dc, but still has core mathematical operations needed for most simple usage.

In addition to the command-line utility, dc.rs provides library components for other programs to use, including the RPN evaluator, the types and utility functions for said types.
