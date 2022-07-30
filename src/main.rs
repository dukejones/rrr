extern crate skim;
use skim::prelude::*;
use ssh2::{Channel, Error, Session};
use std::io::Cursor;
use std::{io::Read, net::TcpStream, path::*};

fn main() {
    // match do_ssh() {
    //     Ok(msg) => println!("{}", msg),
    //     Err(err) => println!("error! {:?}", err),
    // }
    do_skim();
}

fn do_ssh() -> Result<String, Error> {
    // Almost all APIs require a `Session` to be available
    // let sess = Session::new()?;
    // let mut agent = sess.agent()?;

    // // Connect the agent and request a list of identities
    // agent.connect().unwrap();
    // agent.list_identities().unwrap();

    // for identity in agent.identities().unwrap() {
    //     println!("{}", identity.comment());
    //     // let pubkey = identity.blob();
    // }

    // Connect to the local SSH server
    let tcp = TcpStream::connect("dukedorje.com:22").unwrap();
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    sess.userauth_pubkey_file(
        "duke",
        None,
        Path::new("/Users/dukejones/.ssh/id_ed25519"),
        None,
    )?;

    let mut channel = sess.channel_session()?;

    // // send a ssh command to the shell
    // let mut remote_path = PathBuf::new();
    // remote_path.push("/Media");
    // let p = format!("ls {:?}", remote_path);
    // println!("{}", exec(&mut channel, p.as_str()));

    // SFTP
    let sftp = sess.sftp()?;
    let dir = sftp.readdir(&Path::new("/Media"))?;
    for (path, _) in dir {
        match sftp.readdir(&path) {
            Ok(dir) => {
                for (path, _) in dir {
                    println!("{:?}", path);
                }
            }
            Err(e) => println!("error for {:?}! {:?}", path, e),
        }
    }

    // EXIT
    channel.close()?;
    channel.wait_close()?;
    // println!("{}", channel.exit_status()?);

    Ok("session complete".into())
}

// fn exec(channel: &mut Channel, command: &str) -> String {
//     channel.exec(command).unwrap();
//     let mut s = String::new();
//     channel.read_to_string(&mut s).unwrap();
//     s
// }

pub fn do_skim() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let input = "aaaaa\nbbbb\nccc".to_string();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
    }
}
