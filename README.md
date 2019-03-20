# rustrace
Strace in Rust :mag:

> strace is the system-call tracer, which traces the calls that a program makes into the kernel in order to interact with the outside world. 

This code uses nix library which is platform dependent, hence there's a dockerfile which you can use to run this code!
docker run -it --security-opt seccomp=unconfined <image_name>

Since, ptrace(the syscall that strace implementation uses) allows you to look at and modify the registers of a program(which is pretty shady you'd need to disable seccomp mode.

Inspired by [strace in C](https://blog.nelhage.com/2010/08/write-yourself-an-strace-in-70-lines-of-code/) and https://jvns.ca/strace-zine-unfolded.pdf

Made at the Recurse Center :heart:
