
// WORK IN PROGRESS


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
  nvacl = new_NavAbilityClient(url,atk);

  NavAbilityDFG *nvafg = NULL;
  char fglbl[30];
  sprintf(fglbl, "FG_%03d", rand());
  printf("fglbl: %s\n", fglbl);
  nvafg = new_NavAbilityDFG(
      nvacl,
      fglbl,
      "BOT_01",
      NULL,
      1,  // addAgentIfAbsent
      1   // addGraphIfAbsent
  ); // must freeR(nvafg) later


  // a basic normal distribution
  FullNormal *normal = NULL;
    double mn[6] = { 0.0 };
    // a basic covariance matrix
    double cv[36] = { 0.0 };
    cv[0] = 1.0;
    cv[7] = 1.0;
    cv[14] = 1.0;
    cv[21] = 1.0;
    cv[28] = 1.0;
    cv[35] = 1.0;
  normal = new_FullNormal(6,mn,cv); // must freeR(normal) later

  // BUILD A FACTOR GRAPH with multiple variables
  VariableDFG* v = NULL;
  FactorDFG* f = NULL;
  PriorPose3_FullNormal *pf = NULL;
  Pose3Pose3_FullNormal *rf = NULL;

  // inputs: (nvafg,label,variableType, [_tags,_solvable,_timestamp,_nstime,_metadata])
  addVariable(nvafg, "x0", "Pose2", "", "", 0, 1);
  addVariable(nvafg, "x1", "Pose2", "", "", 0, 1);
  rf = new_Pose3Pose3(normal);  // new_ means must freeR(rf) later
  char* fid = addFactor(
      nvafg,
      "x0;x1;",
      rf,
      "", 
      "", 0, 
      1
  ); freeR(f2); freeR(normal); // because new_ was used
  printf("Added factor id: %s\n", fid);
  
  addVariable(nvafg, "x2", "Pose2", "", "", 0, 1);
  fid = addFactor(
    nvafg,
    "x1;x2;",
    rf,
    "", 
    "", 0, 
    1
  ); freeR(f2); freeR(normal); // because new_ was used
  printf("Added factor id: %s\n", fid);

  addVariable(nvafg, "x3", "Pose2", "", "", 0, 1);
  fid = addFactor(
    nvafg,
    "x2;x3;",
    rf,
    "", 
    "", 0, 
    1
  ); freeR(f2); freeR(normal); // because new_ was used
  printf("Added factor id: %s\n", fid);

  addVariable(nvafg, "x4", "Pose2", "", "", 0, 1);
  fid = addFactor(
    nvafg,
    "x3;x4;",
    rf,
    "", 
    "", 0, 
    1
  ); freeR(f2); freeR(normal); // because new_ was used
  printf("Added factor id: %s\n", fid);


  // and prior factor indicating the starting location
  pf = new_PriorPose3(normal);
  fid = addFactor(
      nvafg,
      "x0;",
      pf,
      "", 
      "", 0, 
      1
  ); freeR(f2); freeR(normal); // because new_ was used
  printf("Added factor id: %s\n", fid);

  // Solve the basic graph
  startWorker_solveParametric(nvafg);

  // wait for solve to finish (THIS IS THE OVERSIMPLEFIED PART)

  // delete variables now marginalized out
  deleteVariable(nvafg, "x0");
  deleteVariable(nvafg, "x1");
  deleteFactor(nvafg, /*LABEL HERE*/);

  // GET MEAN AND COV OF X2 AND USE AS PRIOR
  X2 = getVariable(nvafg, "x2");
  mn = getMean(X2); cv = getCov(X2);
  // and prior factor indicating the starting location
  pf = new_PriorPose3(normal);
  fid = addFactor(
      nvafg,
      "x2;",
      pf,
      "", 
      "", 0, 
      1
  ); freeR(f2); freeR(normal); // because new_ was used
  printf("Added factor id: %s\n", fid);

  addVariable(nvafg, "x5", "Pose2", "", "", 0, 1);
  fid = addFactor(
    nvafg,
    "x4;x5;",
    rf,
    "", 
    "", 0, 
    1
  ); freeR(f2); freeR(normal); // because new_ was used
  addVariable(nvafg, "x6", "Pose2", "", "", 0, 1);
  fid = addFactor(
    nvafg,
    "x5;x6;",
    rf,
    "", 
    "", 0, 
    1
  ); freeR(f2); freeR(normal); // because new_ was used

  // Solve the basic graph
  startWorker_solveParametric(nvafg);

  // See other examples for combining other data including IMU, Camera, Lidar

  freeR(nvafg); freeR(nvacl);
  printf("All done.\n");
  return 0;
}