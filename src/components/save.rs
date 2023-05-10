use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{ui::UIState, wasm::html_body};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    // app
    //   .add_system(download.in_schedule(OnEnter(UIState::Save)));

    app
      .add_startup_system(download);
  }
}


fn download() {

  info!("download");
  let body = html_body();
  
  let res = body.query_selector("#download");
  
  let a_ops = match res {
    Ok(ops) => ops,
    Err(e) => { 
      info!("{:?}", e);
      return ()
    }
  };
  info!("download1");

  if a_ops.is_some() {
    info!("download2");
    // let data = Data {
    //   player: Player {
    //     position: [0.0, 1.0, 0.0],
    //   },
    //   terrains: Terrains { keys: vec![[0, 0, 0]], voxels: vec!["Test".to_string()] }
    // };

    // let config = config::standard();
    // // let encoded: Vec<u8> = bincode::encode_to_vec(&data, config).unwrap();
    // // let str = array_bytes::bytes2hex("", encoded);

    // // let str = toml::to_string_pretty(&data).unwrap();
    // let str = toml::to_string(&data).unwrap();
    // // let encoded: Vec<u8> = bincode::encode_to_vec(&data, config).unwrap();
    // // let str = array_bytes::bytes2hex("", encoded);
    
    // info!("a: {:?}", str);
    // let a = a_ops.unwrap();
    // a.set_attribute("download", "save.toml");
    // a.set_attribute("href", format!("data:,{:?}", str).as_str());
    // // a.set_attribute("innerHTML", "download");

    // let a1: HtmlElement = a.dyn_into::<HtmlElement>().unwrap();
    // a1.click();



    let data = Data {
      player: Player {
        position: [0.0, 1.0, 0.0],
      },
      terrains: Terrains { keys: vec![[0, 0, 0]], voxels: vec!["Test".to_string()] }
    };

    let str = toml::to_string_pretty(&data).unwrap();
    

    let parts = js_sys::Array::of1(&unsafe {
      js_sys::Uint8Array::view(str.as_bytes())
          .into()
    });
    info!("download3");
    let blob_res = web_sys::Blob::new_with_u8_array_sequence(&parts);
    if blob_res.is_err() {
      return;
    }
    info!("download4");
    let blob = blob_res.unwrap();

    let url_res = web_sys::Url::create_object_url_with_blob(&blob);
    if url_res.is_err() {
      return;
    }
    info!("download5");

    let a = a_ops.unwrap();
    a.set_attribute("download", "save.toml");
    a.set_attribute("href", &url_res.unwrap());
    let a1: HtmlElement = a.dyn_into::<HtmlElement>().unwrap();
    a1.click();
  }




  // let opt = body.child_nodes().get(0);
  // if opt.is_some() {
  //   let o = opt.unwrap();
  //   o.
  // }
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Data {
  pub player: Player,
  pub terrains: Terrains,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Player {
  pub position: [f32; 3]
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Terrains {
  pub keys: Vec<[i64; 3]>,
  pub voxels: Vec<String>,
}