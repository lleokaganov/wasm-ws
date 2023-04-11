// #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

// use crate::kambella_patches;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod mainjs;
use crate::mainjs::*;

use serde_json::{Value,Value::Null};

// mod mokele_mbember;
// use mokele_mbember::{moke_send_money}; // {moke, moke_send_money, get_chain, };


fn ram() {
    zabil("ram",&format!("<div>genesis_hash: <font color='green'>{}</font></div>
<div>spec_version: <font color='green'>{}</font> transaction_version: <font color='green'>{}</font></div>
<div>block_hash: <font color='green'>{}</font></div>"
	,get("genesis_hash")
	,get("spec_version")
	,get("transaction_version")
	,get("block_hash")
));
    otkryl("ram");
}


#[wasm_bindgen]
pub fn ws_event(mode: &str,text: &str) {
    if mode == "get" {
	let v: Value = serde_json::from_str(&text).unwrap_or("{null}".into());

	// E r r o r s
	if v["error"] != Null {
	    idie(&format!("<font color=red>ERROR #{} {}</font>",&v["error"]["code"],&v["error"]["message"].as_str().unwrap_or("") ));
	    return;
	}

	// N u m b e r s
	let id: u64 = v["id"].as_u64().unwrap_or(0);
	match id {

	    1 => { // genesis_hash
		set("genesis_hash", v["result"].as_str().unwrap_or(""));
		ram();
	    },

	    2 => { // block_hash
		set("block_hash", v["result"].as_str().unwrap_or(""));
		ram();
	    },

	    3 => { // specVersion & transactionVersion
		set("spec_version", &format!("{}",v["result"]["specVersion"].as_u64().unwrap_or(0)) );
		set("transaction_version", &format!("{}",v["result"]["transactionVersion"].as_u64().unwrap_or(0)) );
		ram();
	    },

	    0 => idie("ws Error"),

            other => idie(&format!("Other: {}",other)),
	}
    }
}

#[wasm_bindgen]
pub fn dodo() {
    ws_open("ws://q.lleo.me:9944");
    ws_send("{\"jsonrpc\":\"2.0\",\"method\":\"chain_getBlockHash\",\"params\": [0],\"id\": 1}");
    ws_send("{\"jsonrpc\":\"2.0\",\"method\":\"chain_getBlockHash\",\"params\": [],\"id\": 2}");
    ws_send("{\"jsonrpc\":\"2.0\",\"method\":\"chain_getRuntimeVersion\",\"params\":[],\"id\": 3}");
//    ws_send("{\"jsonrpc\":\"2.0\",\"method\":\"chgetRuntimeVersion\",\"plarams\":[],\"id\": 4}");
// {\"jsonrpc\":\"2.0\",\"method\":\"chain_getBlockHash\",\"params\": [],\"id\": 1}");
// '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock", "params":[]}'
// '
//    idie("dodo");
    // idie( &format!("<b>WASM: ws_event: {mode}</b><br>{text}") );
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // open window with some info for 1000 ms
    salert(&format!("Alert 2 sec: <b>{}</b>",name),2000);
}

#[wasm_bindgen]
pub fn toclip(name: &str) {
    // copy to clipboard
    cpbuf(name);
}

#[wasm_bindgen]
pub fn do_plays() {

    // view window with photo
    bigfoto("https://lleo.me/dnevnik/2023/04/04/0.webp");

    // play sound
    plays("https://lleo.me/dnevnik/design/sound/bbm_tone.mp3");

}

#[wasm_bindgen]
pub fn do_resolutions() {

    let s: String = format!("
<p>Window: {}x{}
<p>Document: {}x{}
<p>Scroll: {}x{}
",
    getWinW().as_f64().unwrap(), getWinH().as_f64().unwrap(),
    getDocW().as_f64().unwrap(), getDocH().as_f64().unwrap(),
    getScrollW().as_f64().unwrap(), getScrollH().as_f64().unwrap()
);

    // create window id='myid'
    ohelpc("myid", "Screen Resolutions", &s);
}

#[wasm_bindgen]
pub fn do_progress() {
    let max: f64 = 100.0;

    // get from globals
    let mut x: f64 = get("x").parse().unwrap_or(0.0);

    if x > max {
	progress("progr", 0.0, 0.0, "" );
	x = 0.0;
    } else {
        progress("progr", x, max, &format!("Working... progress={}% Press again!",&x) );
        x += 5.0;
    }

    // set to globals
    set("x", &x.to_string() );
}

// wget JSON (by JS), parse and view
#[wasm_bindgen]
pub fn btc() {

    let x: u32 = get("xbit").parse().unwrap_or(0) + 1;
    set("xbit", &x.to_string() );

    let s: String = AGET("https://www.blockchain.com/ru/ticker").as_string().unwrap_or("{}".to_string());
    let v: Value = serde_json::from_str(&s).unwrap_or("{null}".into());

    dobavil1("BTC", &format!("{:03}) BTC = {} USD", x, v["USD"]["last"]) );
}

// lifehack to store Globals in <body> attributes
fn get(key: &str) -> String { at_get("globals", key).as_string().unwrap_or("".to_string()) }
fn set(key: &str, value: &str) { at_set("globals", key, value); }
