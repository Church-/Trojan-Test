extern crate daemonize;

use daemonize::{Daemonize};

extern crate irc;
use irc::client::prelude::*;

//extern crate ion_shell;

//use ion_shell::{ShellBuilder,Capture};

use std::io::Read;
use std::process::{Stdio, Child, Output};
use std::process;
use std::error::Error;
use std::io::Write;
use std::str;

fn eval(input: &str)  -> Result<Vec<u8>,Box<Error>> {
	let cmd_vec: Vec<&str> = input.split("|").collect();
	let cmd_vec_len = cmd_vec.len();
	let mut counter = 0;
	let mut cmd_out: Vec<u8> = Vec::new();;
	for part in cmd_vec {
		let part_vec: Vec<&str> = part.split(" ").filter(|s| s != &"").collect();
		println!("{:?}",part_vec);
		if counter == 0 {
			let first_cmd = process::Command::new(part_vec[0])
				.args(&part_vec[1..])
				.stdout(Stdio::piped())
				.output()
				.expect("Fail");

			cmd_out = first_cmd.stdout;
			}
		else if counter > 0 {
			if counter < cmd_vec_len {
				let mut mid_cmd = process::Command::new(part_vec[0])
						.args(&part_vec[1..])
						.stdin(Stdio::piped())
						.stdout(Stdio::piped())
						.spawn()
						.expect("Fail");
			{
				let mut mid_stdin = mid_cmd.stdin.as_mut().unwrap();
				mid_stdin.write_all(&cmd_out);
			}
				let mut output = mid_cmd.wait_with_output().unwrap();

				cmd_out = output.stdout;
				
			}
		}
		else if  counter == cmd_vec_len {
			let mut final_cmd = process::Command::new(part_vec[0])
				.args(&part_vec[1..])
				.stdin(Stdio::piped())
				.stdout(Stdio::piped())
				.spawn()
				.expect("Fail");

			{
				let mut final_stdin = final_cmd.stdin.as_mut().unwrap();
				final_stdin.write_all(&cmd_out);
			}
			
				let mut output = final_cmd.wait_with_output().unwrap();

				cmd_out = output.stdout;

			//return output.stdout.as_slice();
		}
		counter += 1;
	}
	Ok(cmd_out)
}





/*
fn eval(msg: &str) -> Vec<String> {
	let mut shell = ShellBuilder::new().as_library();
	let shell_result = shell.fork(Capture::Stdout, |shell| {
		shell.execute_command(msg);
	}).unwrap();
	
	let mut shell_output = shell_result.stdout.unwrap();
	let mut output_string = String::new();
	shell_output.read_to_string(&mut output_string);

	let output_vec: Vec<String> = output_string.split("\n").map(|s| s.to_string()).collect();

	output_vec
}
*/
fn main() -> Result<(),Box<Error>> {
	 let daemonize = Daemonize::new().pid_file("/tmp/test.pid")
	                            	 .chown_pid_file(true)
	                                 .working_directory("/tmp")
	                                 .user("root")
	                                 .group("root") // Group name
	                                 .privileged_action(|| "Executed before drop privileges");


    let config = Config {
            nickname: Some("the-irc-crate".to_owned()),
            server: Some("irc.freenode.org".to_owned()),
            channels: Some(vec!["#rust-bot-church".to_owned()]),
            ..Config::default()
        };
        
//	daemonize.start();
	
    let client = IrcClient::from_config(config).unwrap();

    client.identify().unwrap();

    client.for_each_incoming(|irc_msg| {

    	if let Command::PRIVMSG(channel, message) = irc_msg.command {
    		if message.contains(client.current_nickname()) {

					let msg_vec: Vec<&str> = message.split(client.current_nickname()).collect();
					println!("{:?}",msg_vec);
					let output = eval(msg_vec[1]).unwrap();
					let mut out_vec: Vec<&str> = (str::from_utf8(&output)).unwrap().split("\n").collect();
					println!("{:?}",out_vec);
//					out_vec = out_vec.split(&"\n").collect();
					out_vec.iter().for_each(|msg| client.send_privmsg(&channel, &msg).unwrap());
    			
    		}
		}
	}).unwrap();
    
	
    Ok(())
}