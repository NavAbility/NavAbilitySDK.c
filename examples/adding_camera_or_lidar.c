// This is a simple example using the NavAbility SDK and API to add blob data to variables in a factor graph.
// It demonstrates how to create a factor graph with multiple variables and manage blob data.
// It also shows how to compute lidar registration and image processing tasks like white balance and affordance detection.
// See other complementory examples different usage and features.
// This is an introductory example.
//
// Copyright (c) 2025 The NavAbility(TM) Contributors.
//  WhereWhen.ai supports open-source (science, algorithms, and standards), 
//  including the permissive/free use of the Caesar.jl and NavAbilitySDKs 
//  as is provided under the Apache License, Version 2.0 (the "License").
//  You may use this file according to the public License, including commercial use, free of charge. 
//  The License is available at http://www.apache.org/licenses/LICENSE-2.0
//  Unless required by applicable law or agreed to in writing, software distributed under the License is distributed
//  on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and limitations under the License.
// Contact info@wherewhen.ai regarding warranties, support, or cost savings through economies of scale.

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

  // Initialize the SubscriptionManager with the NavAbilityClient -- used later for synchronizing events
  SubscriptionManager* nvasm = NULL;
  nvasm = start_SubscriptionManager(nvacl, 64); // keep up to 64 events in the manager

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
  bool success = block_on(nvasm, wid, 2000); 
  // printf("received worker id: %s\n", wid);
  freeR(wid); // free the worker id returned by computeLidarRegistration

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
  success = block_on(nvasm, wid, 1000); 
  // printf("received worker id: %s\n", wid);
  freeR(wid); // free the worker id returned by computeImageWhitebalance

  // compute visual priors on image data
  wid = addAffordance_kNNvisual(nvafg, "x3", "map01;map02;", 5, 5);
  success = block_on(nvasm, wid, 5000); 
  printf("Visual affordance worker success: %d\n", success);
  freeR(wid); // free the worker id returned by addAffordance_kNNvisual

  // See other example for different usage of the same agent/graph/model

  freeR(nvasm); // stop the subscription manager
  freeR(nvafg); freeR(nvacl); freeR(store);
  printf("All done.\n");
  return 0;
}
