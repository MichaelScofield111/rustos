## Testing

this post explores unit and intergration testing in `no_std` executables. <br>
We will use Rust's support for custom test frameworks to execute test functions inside our kernel.

### Testing in rust
Rust has a `build-in test framework` that is capable of running unit tests without the need to set anything up.
<br>
just create function that checks some result through assertions and the #[test] attribute to the function header. Then `cargo test`.
<br>
Unfortunately, it’s a bit more complicated for no_std applications such as our kernel. The problem is that Rust’s test framework implicitly uses the built-in test library, which depends on the standard library. This means that we can’t use the default test framework for our #[no_std] kernel.

```bash
> cargo test
   Compiling blog_os v0.1.0 (/…/blog_os)
error[E0463]: can't find crate for `test`
```


#### Custom test framework
Rust supports replacing the default custom test frameworks through the unstable `custom_test_frameworks` feature. this feature requires no externel libraries and can work in `no_std` environments.
<br>
It works by collecting all functions annotated with a #[test_case] attribute and then invoking a user-specified runner function with the list of tests as an argument. 

### Printing to the console
To see the test output on the console, we need to send the data from our kernel to the host system. There are several ways to do this.
#### serial port
A simple way to send the data is to use the `serial port`
It's easy to program and QEMU can redirect the bytes sent over serial to the host's standard output or a file.
<br>
the chips implementing a serial port usually are called `UARTs`. The common UARTs today are all compatible with the 16550 UART, so we will use that model for our testing framework.