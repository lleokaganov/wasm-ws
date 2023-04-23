use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};
use parity_scale_codec::Decode;
use scale_info::{form::PortableForm, Type, TypeDef, Variant};

// https://habr.com/ru/articles/550460/
// macro_rules! ebal {
//     ($s:expr) => (
//         salert(&format!("s: {:#?}",$s),10000);
//     );
//     // ($e:expr) => {
//     //     $e
//     // };
// }

macro_rules! nah {
    ($s:expr) => {
        match $s {
            Some(l) => { l },
            None => {
                salert("Что-то пошло не так",1000);
                return;
            }
        }
    }
}

macro_rules! neh {
    ($s:expr) => {
        match $s {
            Ok(l) => { l },
            Err(l) => {
                salert(&format!("Ошибка: {:#?}",l),1000);
                return;
            },
        }
    }
}


macro_rules! log {
    ($s:expr, $p:expr) => { dobavil("zombuka",&format!("<div style='font-color:grey' class=r>{} : {:#?}</div>",$s,$p)); };
    ($s:expr) => { dobavil("zombuka",&format!("<div class=r style='font-color:grey'>{}</r>",$s)); };
}


use std::sync::Mutex;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref X: Mutex<f64> = Mutex::new(0.0);
    pub static ref S: Mutex<String> = Mutex::new("".to_string());

    pub static ref GENESIS_HASH: Mutex<String> = Mutex::new("".to_string());
    pub static ref BLOCK_HASH: Mutex<String> = Mutex::new("".to_string());
    pub static ref SPEC_VERSION: Mutex<u32> = Mutex::new(0);
    pub static ref TRANSACTION_VERSION: Mutex<u32> = Mutex::new(0);

    pub static ref METADATA: Mutex<String> = Mutex::new("".to_string());
}

// #![feature(proc_macro, wasm_custom_section, wasm_import_module)]
// use crate::kambella_patches;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod mainjs;
use crate::mainjs::*;

use serde_json::{Value,Value::Null};

// mod mokele_mbember;
// use mokele_mbember::{moke_send_money}; // {moke, moke_send_money, get_chain, };


#[wasm_bindgen]
pub fn do_pallet(s: &str) { // выбрана палета s
        zabil("zombuka","");
        log!("<hr>Работаем с: ",s);
    let metadata: RuntimeMetadataV14 = nah!( parse_metadata() );
        log!("Получили Метадату");
    let pal = nah!( metadata.pallets.iter().find(|p| p.name == s) );
        log!("Получили палету с именем",&s);
    let id = nah!(&pal.calls).ty.id;
        log!("Получили id",id);
    let resolve = nah!(metadata.types.resolve(id));
        log!("Получили resolve, docs",&resolve.docs);
    if let TypeDef::Variant(variant) = &resolve.type_def {
      zabil(s,"");
      variant.variants.clone().into_iter().for_each(|l| {
        dobavil(s,&format!("<div class='br' style='padding-left:30px;color:green'>{}</div>",l.name) );
        log!("name = ",l.name);
      });
    }

    // center("idie");
}


/*
Type {
    path: Path { segments: [ "pallet_peerreview", "pallet", "Call", ], },
    type_params: [ TypeParameter { name: "T", ty: None, }, ],
    type_def: Variant( TypeDefVariant {
        variants: [
            Variant { name: "react_to_doi", fields: [ Field { name: Some( "doi", ), ty: UntrackedSymbol { id: 42, marker: PhantomData core::any::TypeId>, }, type_name: Some( "Doi", ), docs: [], }, Field { name: Some( "opinion", ), ty: UntrackedSymbol { id: 43, marker: PhantomData core::any::TypeId>, }, type_name: Some( "Opinion", ), docs: [], }, ], index: 0, docs: [ "React to published article backed by authority publisher with DOI", ], },
            Variant { name: "react", fields: [ Field { name: Some( "id", ), ty: UntrackedSymbol { id: 11, marker: PhantomData core::any::TypeId>, }, type_name: Some( "T::Hash", ), docs: [], }, Field { name: Some( "opinion", ), ty: UntrackedSymbol { id: 43, marker: PhantomData core::any::TypeId>, }, type_name: Some( "Opinion", ), docs: [], }, ], index: 1, docs: [], },
            Variant { name: "post", fields: [ Field { name: Some( "id", ), ty: UntrackedSymbol { id: 11, marker: PhantomData core::any::TypeId>, }, type_name: Some( "T::Hash", ), docs: [], }, Field { name: Some( "authors", ), ty: UntrackedSymbol { id: 113, marker: PhantomData core::any::TypeId>, }, type_name: Some( "BoundedVec", ), docs: [], }, ], index: 2, docs: [], },
            Variant { name: "reffer_to_doi", fields: [ Field { name: Some( "newer", ), ty: UntrackedSymbol { id: 11, marker: PhantomData core::any::TypeId>, }, type_name: Some( "T::Hash", ), docs: [], }, Field { name: Some( "older", ), ty: UntrackedSymbol { id: 42, marker: PhantomData core::any::TypeId>, }, type_name: Some( "Doi", ), docs: [], }, ], index: 3, docs: [], },
        ],
    },),
    docs: [ "Contains one variant per dispatchable that can be called by an extrinsic.", ],
}
*/



#[wasm_bindgen]
pub fn do_progress() {
    let max: f64 = 100.0;
    let mut x = neh!( X.lock() );
    let mut s = neh!( S.lock() );
    *s = format!("{}*",&s);
    if *x > max { progress("progr", 0.0, 0.0, ""); *x = 0.0; }
    else { progress("progr", *x, max, &format!("Working... progress={}% {} Press again!",&s,&x) ); *x += 5.0; }
}


// Мы будем каждый раз парсить метадату заново из байт потому что я уже знаю, как хранить байты в Гллобале
// но пока не понимаю, как хранить RuntimeMetadataV14, что ей писать в new() при инициализации
// Кроме того, я подозреваю, что распакованная Метадата занимает сильно дохуя. А оно нам нахуя?
fn parse_metadata() -> Option<RuntimeMetadataV14> {
    let metadata = get("metadata");
    let meta = hex::decode(metadata).unwrap_or("".into());
    if !(*meta).starts_with(&[109, 101, 116, 97]) { return None; }
    let e=RuntimeMetadata::decode(&mut &meta[4..]);
    match e {
        Ok(RuntimeMetadata::V14(out)) => Some(out),
        Ok(_) => None,
        Err(_) => None,
    }
}


fn ram() {
    zabil("ram",&format!("<div>genesis_hash: <font color='green'>{}</font></div>
<div>spec_version: <font color='green'>{}</font></div>
<div>transaction_version: <font color='green'>{}</font></div>
<div>block_hash: <font color='green'>{}</font></div>
<div>Metadata: <font color='green'>{} kb</font></div>"
	,get_oru("genesis_hash")
	,get_oru("spec_version")
	,get_oru("transaction_version")
	,get_oru("block_hash")
    ,get_oru("metadata").len()/1024
));
    otkryl("ram");
}

const WS_GENESIS_HASH: u32 = 1;
const WS_BLOCK_HASH:   u32 = 2;
const WS_VERSION:      u32 = 3;
const WS_METADATA:     u32 = 4;

// #[derive(FromPrimitive)]
// #[repr(u32)]
// enum WS {
//      GenesisHash = 1,
//      BlockHash = 2,
//      Version = 3,
// }

#[wasm_bindgen]
pub fn ws_event(mode: &str,text: &str) {

  if mode == "get" {
	let v: Value = neh!( serde_json::from_str(&text) );

	// E r r o r s
	if v["error"] != Null {
	    idie(&format!("<font color=red>ERROR #{} {}</font>",&v["error"]["code"],&v["error"]["message"].as_str().unwrap_or("") ));
	    return;
	}

	// N u m b e r s
	let id: u32 = nah!(v["id"].as_u64()) as u32;

	match id {
	    WS_GENESIS_HASH => { // genesis_hash
            let x=nah!(v["result"].as_str()); set("genesis_hash", x);
            // let mut x = GENESIS_HASH.lock().unwrap(); *x=v["result"].as_str().unwrap_or("").to_string();
		    ram();
	    },

	    WS_BLOCK_HASH => { // block_hash
            let x = nah!(v["result"].as_str()); set("block_hash", x);
            // let mut x = BLOCK_HASH.lock().unwrap(); *x=v["result"].as_str().unwrap_or("").to_string();
		    ram();
	    },

	    WS_VERSION => { // specVersion & transactionVersion
            // idie(&format!("ws_event [{mode}] = {id}"));
            let x = nah!( v["result"]["specVersion"].as_u64() ); set("spec_version", &format!("{}",x) );
            let x = nah!( v["result"]["transactionVersion"].as_u64() ); set("transaction_version", &format!("{}",x) );
            // let mut x = SPEC_VERSION.lock().unwrap(); *x=v["result"].as_u64().unwrap_or(0) as u32;
            // let mut x = TRANSACTION_VERSION.lock().unwrap(); *x=v["result"].as_u64().unwrap_or(0) as u32;
            ram();
	    },

        WS_METADATA => {
            let mut metadata = nah!( v["result"].as_str() );
            if &metadata[0..2] == "0x" { metadata = metadata[2..].into(); }
            set("metadata",metadata);
            ram();

            // окно выбора паллет
            let metadata = nah!(parse_metadata());
            let o = metadata.pallets.iter().map(|p| {
                format!("<div><input style='margin:5px;' type='button' onclick='do_pallet(this.value)' value='{x}'>
<div id='{x}'></div>
</div>",x=p.name)
            }).collect::<String>();

            idie(&format!("{}<div id='zombuka'></div>",&o));
        },

        other => idie(&format!("Other: {}",other)),
	}
  }
}

#[wasm_bindgen]
pub fn dodo() {
    // ws_open("ws://q.lleo.me:9944");
    ws_open("wss://westend-rpc.polkadot.io:443");
    ws_snd("chain_getBlockHash","0",WS_GENESIS_HASH);
    ws_snd("chain_getBlockHash","",WS_BLOCK_HASH);
    ws_snd("chain_getRuntimeVersion","",WS_VERSION);
    ws_snd("state_getMetadata","",WS_METADATA);
}

    // play sound
    //plays("https://lleo.me/dnevnik/design/sound/bbm_tone.mp3");

// lifehack to store Globals in <body> attributes
fn get(key: &str) -> String { at_get("globals", key).as_string().unwrap_or("".to_string()) }
fn get_oru(key: &str) -> String { 
    let mut o = at_get("globals", key).as_string().unwrap_or("".to_string());
    if o == "" { o = "<img src='http://q.lleo.me/design/img/ajaxm.gif'>".to_string(); }
    o
}

fn set(key: &str, value: &str) { at_set("globals", key, value); }

fn ws_snd(method: &str, params: &str, id: u32) {
    ws_send( &format!("{{\"jsonrpc\":\"2.0\",\"method\":\"{method}\",\"params\": [{params}],\"id\": {id}}}") );
}