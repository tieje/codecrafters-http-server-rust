# Codecrafters HTTP Server Project in Rust

- [Codecrafters HTTP Server Project in Rust](#codecrafters-http-server-project-in-rust)
  - [My Review](#my-review)
    - [Liked](#liked)
    - [Disliked](#disliked)
    - [Evaluation](#evaluation)
  - [Initial Content](#initial-content)
    - [Passing the first stage](#passing-the-first-stage)
    - [Stage 2 \& beyond](#stage-2--beyond)

## My Review

### Liked

I liked that each challenge built on top of fundamental concepts and then made me want to restructure my code to efficiently provide answers.

### Disliked

The only thing that I disliked was for the file creation, you needed to test which directory was your file actually being created in and write that in statically. Or maybe that's just me and I should have figured out how to read and create files in the programs current directory.

### Evaluation

This project increased my confidence in topics that I thought that I knew well. Initially, I wanted to write spaghetti code in a single `main.rs` file, but the developer (and laziness) in me made me want to organize my code. I'm glad I picked up this project during a time when I was dealing with some burnout. This project was enjoyable, challenging, and well-structured. Unintuitively, one of the cures for burnout is leaning into the work and picking up coding projects outside of my day-job.

## Initial Content

[![progress-banner](https://backend.codecrafters.io/progress/http-server/44580858-2784-4f6a-8e2c-1ecebe54b952)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own HTTP server" Challenge](https://app.codecrafters.io/courses/http-server/overview).

[HTTP](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol) is the
protocol that powers the web. In this challenge, you'll build a HTTP/1.1 server
that is capable of serving multiple clients.

Along the way you'll learn about TCP servers,
[HTTP request syntax](https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html),
and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

### Passing the first stage

The entry point for your HTTP server implementation is in `src/main.rs`. Study
and uncomment the relevant code, and push your changes to pass the first stage:

```sh
git commit -am "pass 1st stage" # any msg
git push origin master
```

Time to move on to the next stage!

### Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.87)` installed locally
1. Run `./your_program.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.
