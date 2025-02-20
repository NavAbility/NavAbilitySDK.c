
// build.rs

use std::process::Command;
use std::fs;

fn main() {
  let path = "src/schema.json";
  match fs::exists(path) {
    Ok(e) => {
      if e {
        if 100 < fs::metadata(path).unwrap().len() {
          println!("cargo::warning=NavAbilitySDK.rs build did not update existing schema.json.");
          return ();
        } else {
          println!("cargo::warning=NavAbilitySDK.rs build schema.json file is incomplete - trying to introspect again.");
        }
      }
      let mut fetchschema = Command::new("sh");
      fetchschema.arg("-c").arg("make fetch-schema");
      match fetchschema.output() {
        Ok(o) => {
          println!("cargo::warning={}: {:?}","NavAbilitySDK.rs trying schema introspection (ensure env variables NVA_API_API/TOKEN)", o);
        }
        Err(e) => {
          println!("cargo::warning={} {:?}","NavAbilitySDK.rs build schema introspection failed with error:", e);
        }
      }
    }
    Err(e) => {
      println!("cargo::warning=NavAbilitySDK.rs build unable to check for src/schema.json: {:?}", e);
    }
  }
  // println!("cargo::rerun-if-changed=build.rs");
}

