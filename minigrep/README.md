A Simple Command line app built in rust. 

As the name implies minigrep is simplified version of the classic command line search tool grep (globally search a regular expression and print). In the simplest use case, grep searches a specified file for a specified string. To do so, grep takes as its arguments a file path and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.

By comparison, our version will be fairly simple.

The code follows [chapter 12 of The Rust Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)