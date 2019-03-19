use nix::sys::ptrace;
use nix::sys::ptrace::*;
use libc::{c_void, user_regs_struct, PT_NULL};
use std::mem;
use std::process::Command;
use std::os::unix::process::CommandExt;
use nix::sys::wait::waitpid;
use std::ptr;
use std::collections::HashMap;

mod system_call_names;

fn traceme() -> std::io::Result<(())> {
    match ptrace::traceme() {
        Ok(()) => Ok(()),
        Err(::nix::Error::Sys(errno)) => Err(std::io::Error::from_raw_os_error(errno as i32)),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}


pub fn get_regs(pid: nix::unistd::Pid) -> Result<user_regs_struct, nix::Error> {
    // XXX : is it safe to drop regs if this fails?
    unsafe {
        let mut regs: user_regs_struct = mem::uninitialized();

        #[allow(deprecated)]
        let res = ptrace::ptrace(
            Request::PTRACE_GETREGS,
            pid,
            PT_NULL as *mut c_void,
            &mut regs as *mut _ as *mut c_void,
        );
        // this
        res.map(|_| regs)
    }
}


fn main() {
    let argv: Vec<_> = std::env::args().collect();
    let mut cmd = Command::new(&argv[1]);
    for arg in argv {
        println!("{}", arg);
        cmd.arg(arg);
    }
    //Hashmap to store the count call, can compare to strace for numbers!
    let mut map = HashMap::new();

    //allow the child to be traced
    let output = cmd.before_exec(traceme);

    let mut child = cmd.spawn().expect("child process failed");

    let pid = nix::unistd::Pid::from_raw(child.id() as libc::pid_t);

    //allow parent to be stopped everytime there is a syscall
    ptrace::setoptions(pid, Options::PTRACE_O_TRACESYSGOOD | Options::PTRACE_O_TRACEEXEC).unwrap();

    waitpid(pid, None);

    // let regs = get_regs(pid);

    // println!("{}", regs.orig_rax);
    // println!("{}", system_call_names::SYSTEM_CALL_NAMES[(regs.orig_rax) as usize]);
    
    /// Whether we are exiting (rather than entering) a syscall.
    let mut exit = true;

    loop {
      let regs = match get_regs(pid) {
          Ok(x) => x,
          Err(err) => {
              eprintln!("Got error {:?}", err);
              break;
          }
      };
      if exit {
        let mut syscallName = system_call_names::SYSTEM_CALL_NAMES[(regs.orig_rax) as usize];
        println!("{}", syscallName);
      

        match map.get(&syscallName) {
          Some(&number) => map.insert(syscallName, number+1),
          _ => map.insert(syscallName, 1),
        };

      
      }
       unsafe {
        ptrace(Request::PTRACE_SYSCALL, pid, ptr::null_mut(), ptr::null_mut());
      }

      waitpid(pid, None);
      exit = !exit;
    }

    for (syscall, &number) in map.iter() {
        println!("{}: {}", syscall, number); 
    }

    // loop {
    //     match waitpid(pid, None) {
    //         Err(_) | Ok(WaitStatus::Exited(_, _)) => break,
    //         Ok(WaitStatus::PtraceSyscall(_)) => {
    //             let regs = get_regs(pid);
    //             println!("{}", system_call_names::SYSTEM_CALL_NAMES[(regs.orig_rax) as usize]); 
               
    //         }
    //         _ => {}
    //     }
    //     unsafe {
    //     ptrace(ptrace::Request::PTRACE_SYSCALL, pid, ptr::null_mut(), ptr::null_mut()).map_err(|e| format!("failed to ptrace syscall {} ({})", pid, e))?;
    // }

    // println!("hi");
    // let output = cmd.before_exec(||{
    //     println!("hello");
    //     Ok(())
    // }).output().unwrap();
    // println!("hi");
   // println!("{}",output);

    // Command::new("ls")
    //     .env("PATH", "/bin")
    //     .spawn()
    //     .expect("ls command failed to start");

    
}
