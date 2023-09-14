#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;
use volo_example::FilterLayer;
use volo_example::S;
use std::fs::File;
//use volo_gen::volo::example::GetItemRequest;
use lazy_static::lazy_static;
use pilota::lazy_static;
//use std::io::Error;
//use std::io::Write;
lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);
    let filename = "database.txt";
    let file = File::open(filename);
    let _file = match file {
        Ok(f) => f,
        Err(_) => File::create(filename).unwrap()
    };
    let sets = S::new();
    let contents: Vec<String> = std::fs::read_to_string(filename).unwrap().trim()
        .split("\n").map(|t| t.to_string()).collect();
    for s in contents {
        let cmd: Vec<String> = s.trim().split(" ").map(|t| t.to_string()).collect();
        if &cmd[0] == "set" {
            let mut is_exist = true;
			if sets.kav.lock().unwrap().get(&cmd[1]) == None {
				is_exist = false;
			}
			if !is_exist {
				sets.kav.lock().unwrap().insert(cmd[1].clone(), cmd[2].clone());
			} else if *sets.kav.lock().unwrap().get(&cmd[1]).unwrap() != cmd[2] {
				sets.kav.lock().unwrap().remove(&cmd[1]);
				sets.kav.lock().unwrap().insert(cmd[1].clone(), cmd[2].clone());
			}
        } else if &cmd[0] == "del" {
            sets.kav.lock().unwrap().remove(&cmd[1]);
        }
    }
    volo_gen::volo::example::ItemServiceServer::new(sets)
        .layer_front(FilterLayer)
        .run(addr)
        .await
        .unwrap();
}
