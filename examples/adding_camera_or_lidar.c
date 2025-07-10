#include <time.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "NavAbilitySDK.h"


// Sample library usage.
int main(void) {
  // change the random seed
  srand(time(NULL));

  // basic setup
  const char* url = getenv ("NVA_API_URL");
  printf("NVA_API_URL: %s\n", (url != NULL) ? "***" : "getenv returned NULL");
  const char* atk = getenv ("NVA_API_TOKEN");
  printf("NVA_API_TOKEN: %s\n", (atk != NULL) ? "***" : "getenv returned NULL");

  NavAbilityClient* nvacl = NULL;
  nvacl = new_NavAbilityClient(url,atk, "");

  NavAbilityDFG *nvafg = NULL;
  char fglbl[30];
  snprintf(fglbl, sizeof(fglbl), "FG_%05d", rand() % 100000);
  printf("fglbl: %s\n", fglbl);
  nvafg = new_NavAbilityDFG(
      nvacl,
      fglbl,
      "BOT_01",
      NULL,
      1,  // addAgentIfAbsent
      1   // addGraphIfAbsent
  ); // must freeR(nvafg) later

  NavAbilityBlobStore *store = NULL;
  store = new_NavAbilityBlobStore(nvacl, "default"); // must freeR(store) later

  // BUILD A FACTOR GRAPH with multiple variables
  // inputs: (nvafg,label,variableType, [_tags,_timestamp,_nstime, _solavble])

  printf("Adding variables...\n");
  addVariable(nvafg, "x0", "RoME.Pose3", "", "", 0, 1);
  addVariable(nvafg, "x1", "RoME.Pose3", "DEMO_LIDAR;", "", 0, 1);
  addVariable(nvafg, "x2", "RoME.Pose3", "", "", 0, 1);
  addVariable(nvafg, "x3", "RoME.Pose3", "DEMO_CAM;", "", 0, 1);
  addVariable(nvafg, "x4", "RoME.Pose3", "", "", 0, 1);
  addVariable(nvafg, "x5", "RoME.Pose3", "DEMO_LIDAR;", "", 0, 1);
  char* v = addVariable(nvafg, "x6", "RoME.Pose3", "", "", 0, 1);
  printf("last variable: %s\n", v);


  // upload lidar data on x1
  // Async versions of these calls are also available
  // char* bid = NULL;
  char* buffer = "add x1 lidar data here, e.g. from a .las file";
  const char* mimetype = "application/octet-stream;ext=las";
  char* bid = addBlob(store, "testdata", mimetype, buffer, strlen(buffer)); 
  printf("Uploaded blobId: %s, size %ld\n", bid, strlen(buffer));

  // connect the newly uploaded lidar data to the graph
  BlobEntry *be = NULL;
  be = new_BlobEntry(    
    bid,                                 // associated blobId
    "left_lidar.las",                    // label
    "default",                           // blobstore
    "SDK.c ex camera_lidar",             // origin
    strlen(buffer),                      // blob size // strlen(buffer)
    "lidar EPROM set for zipco cfg 7",   // description
    mimetype,                            // data mimetype
    "",                                  // metadata
    ""                                   // timestamp UTC
  );



  printf("Created blobentry with label %s\n", getLabel(be));
  char* vbe = addVariableBlobEntry(nvafg, "x1", be); freeR(be);
  printf("added variable blob entry with return id: %s\n", vbe);

  // upload lidar data on x5
  // Async versions of these calls are also available
  // char* bid = NULL;
  buffer = "add x5 lidar data here, e.g. from a .las file";
  bid = addBlob(store, "testdata", "application/octet-stream", buffer, strlen(buffer)); 
  printf("Uploaded blobId: %s\n", bid);
  // connect the newly uploaded lidar data to the graph
  be = new_BlobEntry(    
    bid,                                // associated blobId
    "left_lidar.las",                   // label
    "default",                          // blobstore
    "SDK.c ex camera_lidar",            // origin
    strlen(buffer),                     // blob size
    "lidar EPROM set for zipco cfg 7",  // description
    mimetype,                           // data mimetype
    "",                                 // metadata
    ""                                  // timestamp UTC
  );
  vbe = addVariableBlobEntry(nvafg, "x5", be); freeR(be);
  printf("added variable blob entry with return id: %s\n", vbe);

  // compute registration between two lidar pointclouds
  char* wid = computeLidarRegistration(nvafg, "x1", "left_lidar.las", "x5", "left_lidar.las");
  printf("received worker id: %s\n", wid);

  // Add camera data to x3
  // upload lidar data on x1
  // Async versions of these calls are also available
  buffer = "add x3 camera data here, e.g. from a .jpg file";
  const char* mimetype2 = "image/jpeg";
  bid = addBlob(store, "testdata", mimetype, buffer, strlen(buffer)); 
  printf("Uploaded blobId: %s\n", bid);
  // connect the newly uploaded lidar data to the graph
  be = new_BlobEntry(    
    bid,                          // associated blobId
    "center_camera",              // label
    "default",                    // blobstore
    "SDK.c ex camera_lidar",      // origin
    strlen(buffer),               // blob size
    "",                           // description
    mimetype2,                     // data mimetype
    "",                           // metadata
    ""                            // timestamp UTC
  );
  vbe = addVariableBlobEntry(nvafg, "x3", be); freeR(be);
  printf("added variable blob entry with return id: %s\n", vbe);

  // compute registration between two lidar pointclouds
  wid = computeImageWhitebalance(nvafg, "x3", "center_camera", "center_camera_whitebalanced");
  printf("received worker id: %s\n", wid);

  // compute visual priors on image data
  wid = addAffordance_kNNvisual(nvafg, "x3", "map01;map02;", 5, 5);
  printf("received worker id: %s\n", wid);

  // See other example for different usage of the same agent/graph/model

  freeR(nvafg); freeR(nvacl); freeR(store);
  printf("All done.\n");
  return 0;
}
