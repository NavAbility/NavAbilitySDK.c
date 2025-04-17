
// build.rs

use std::process::Command;
use std::fs;

fn main() {
  let path = "src/gql/schema.json";
  match fs::exists(path) {
    Ok(e) => {
      if e {
        if 100 < fs::metadata(path).unwrap().len() {
          println!("cargo::warning=NavAbilitySDK.rs build did not update existing schema.json.");
          return ();
        } else {
          println!("cargo::warning=NavAbilitySDK.rs build schema.json file is incomplete - trying to introspect again.");
          match fs::remove_file(path) {
            Ok(_) =>println!("cargo::warning=NavAbilitySDK.rs build, incomplete schema.json removed"),
            Err(e) => println!("cargo::warning=ERROR NavAbilitySDK.rs build unable to remove incomplete schema.json {}", e)
        }
        }
      }
      let mut fetchschema = Command::new("sh");
      fetchschema.arg("-c").arg("make fetch-schema");
      match fetchschema.output() {
        Ok(o) => {
          println!("cargo::warning={}: {:?}","NavAbilitySDK.rs trying schema introspection (ensure env variables NVA_API_API/TOKEN)", o);
        }
        Err(e) => {
          let emsg = format!("NavAbilitySDK.rs build schema introspection failed with error: {:?}", e);
          println!("cargo::warning={}",&emsg);
          panic!("{}", emsg);
        }
      }
    }
    Err(e) => {
      println!("cargo::warning=NavAbilitySDK.rs build unable to check for src/schema.json: {:?}", e);
    }
  }
  // println!("cargo::rerun-if-changed=build.rs");
}

