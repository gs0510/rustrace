# rustrace
Strace in Rust :mag:

> strace is the system-call tracer, which traces the calls that a program makes into the kernel in order to interact with the outside world. 

This code uses nix crate which is platform dependent, hence there's a dockerfile which you can use to run this code!

``` docker run -it --security-opt seccomp=unconfined <image_name> ```

Since, ptrace(the syscall that strace implementation uses) allows you to look at and modify the registers of a program(which is pretty shady you'd need to disable seccomp mode.

Inspired by [strace in C](https://blog.nelhage.com/2010/08/write-yourself-an-strace-in-70-lines-of-code/) and by [Julia Evans' zine](https://jvns.ca/strace-zine-unfolded.pdf)

<div style='text-align:center; margin:auto;'>
<a href='http://www.recurse.com' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11322973/9e557144-910b-11e5-959a-8fdaaa4a88c5.png' height='14px'/></a>
</div>
