
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "NavAbilitySDK.h"


// Sample library usage.
int main(void) {

  // basic setup
  const char* url = getenv ("NVA_API_URL");
  printf("NVA_API_URL: %s\n", (url != NULL) ? "***" : "getenv returned NULL");
  const char* atk = getenv ("NVA_API_TOKEN");
  printf("NVA_API_TOKEN: %s\n", (atk != NULL) ? "***" : "getenv returned NULL");

  NavAbilityClient* nvacl = NULL;
  nvacl = new_NavAbilityClient(url,atk);

  NavAbilityDFG *nvafg = NULL;
  nvafg = new_NavAbilityDFG(
      nvacl,
      "FG001",
      "BOT_01",
      NULL,
      true,  // addAgentIfAbsent
      true   // addGraphIfAbsent
  ); // must freeR(nvafg) later

  NavAbilityBlobStore *store = NULL;
  store = new_NavAbilityBlobStore(nvacl, "default"); // must freeR(store) later

  // BUILD A FACTOR GRAPH with multiple variables
  // inputs: (nvafg,label,variableType, [_tags,_timestamp,_nstime, _solavble])
  addVariable(nvafg, "x0", "Pose2", "TESTTAG;", "", 0, 1);
  addVariable(nvafg, "x1", "Pose2", "TESTTAG;", "", 0, 1);
  addVariable(nvafg, "x2", "Pose2", "TESTTAG;", "", 0, 1);
  addVariable(nvafg, "x3", "Pose2", "TESTTAG;", "", 0, 1);
  addVariable(nvafg, "x4", "Pose2", "TESTTAG;", "", 0, 1);
  addVariable(nvafg, "x5", "Pose2", "TESTTAG;", "", 0, 1);
  addVariable(nvafg, "x6", "Pose2", "TESTTAG;", "", 0, 1);


  // upload lidar data on x1
  // Async versions of these calls are also available
  char* bid = NULL;
  char* buffer = "add lidar data here, e.g. from a .las file";
  char* mimetype = "application/octet-stream;ext=las";
  blobId = addBlob(store, "testdata", mimetype, buffer, strlen(buffer)); 
  printf("Uploaded blobId: %s\n", bid);
  // connect the newly uploaded lidar data to the graph
  BlobEntry *be = NULL;
  be = new_BlobEntry(    
    blobId,
    "left_lidar.las", // label
    "default", // blobstore
    "", // hash - ignored
    "SDK.c LidarCamera Example", // origin
    strlen(buffer), // blob size
    "configuration 7, with new ziptie", // description
    mimetype,
    NULL, // metadata
    NULL, // timestamp UTC
  );
  addVariableBlobEntry(nvafg, "x1", be); freeR(be);
  

  // upload lidar data on x5
  // Async versions of these calls are also available
  char* bid = NULL;
  char* buffer = "add lidar data here, e.g. from a .las file";
  blobId = addBlob(store, "testdata", "application/octet-stream", buffer, strlen(buffer)); 
  printf("Uploaded blobId: %s\n", bid);
  // connect the newly uploaded lidar data to the graph
  BlobEntry *be = NULL;
  be = new_BlobEntry(    
    blobId,
    "left_lidar.las", // label
    "default", // blobstore
    "", // hash - ignored
    "SDK.c LidarCamera Example", // origin
    strlen(buffer), // blob size
    "configuration 7, with new ziptie", // description
    mimetype,
    NULL, // metadata
    NULL, // timestamp UTC
  );
  addVariableBlobEntry(nvafg, "x5", be); freeR(be);

  // compute registration between two lidar pointclouds
  startWorker_LidarRegistration(nvafg, "x1", "left_lidar.las", "x5", "left_lidar.las");


  // Add camera data to x3
  // upload lidar data on x1
  // Async versions of these calls are also available
  char* bid = NULL;
  char* buffer = "add camera data here, e.g. from a .jpg file";
  mimetype = "image/jpeg";
  blobId = addBlob(store, "testdata", mimetype, buffer, strlen(buffer)); 
  printf("Uploaded blobId: %s\n", bid);
  // connect the newly uploaded lidar data to the graph
  BlobEntry *be = NULL;
  be = new_BlobEntry(    
    blobId,
    "center_camera", // label
    "default", // blobstore
    "", // hash - ignored
    "SDK.c LidarCamera Example", // origin
    strlen(buffer), // blob size
    "", // description
    mimetype,
    NULL, // metadata
    NULL, // timestamp UTC
  );
  addVariableBlobEntry(nvafg, "x3", be); freeR(be);


  // compute registration between two lidar pointclouds
  startWorker_ImageWhitebalance(nvafg, "x3", "center_camera", "center_camera_whitebalanced");

  // compute visual priors on image data
  startWorker_VisualAffordancePriors(nvafg, "x3", "center_camera_whitebalanced");


  // See other example for different usage of the same agent/graph/model

  freeR(nvafg); freeR(nvacl); freeR(store)
  printf("All done.\n");
  return 0;
}
