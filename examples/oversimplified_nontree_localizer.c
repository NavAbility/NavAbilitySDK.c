
// WORK IN PROGRESS


#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
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
  // reusable pointers
  PriorPose3_FullNormal *pf = NULL;
  Pose3Pose3_FullNormal *rf = NULL;

  // inputs: (nvafg,label,variableType, [_tags,_solvable,_timestamp,_nstime,_metadata])
  addVariable(nvafg, "x0", "Pose2", "", "", 0, 1);
  addVariable(nvafg, "x1", "Pose2", "", "", 0, 1);
  rf = new_Pose3Pose3(normal);  // new_ means must freeR(rf) later
  const char* fid1 = addFactor(
      nvafg,
      "x0;x1;",
      rf,
      "", 
      "", 0, 
      1
  );
  printf("Added factor id: %s\n", fid1);
  
  addVariable(nvafg, "x2", "Pose2", "", "", 0, 1);
  const char* fid2 = addFactor(
    nvafg,
    "x1;x2;",
    rf,
    "", 
    "", 0, 
    1
  );
  printf("Added factor id: %s\n", fid2);

  addVariable(nvafg, "x3", "Pose2", "", "", 0, 1);
  const char* fid3 = addFactor(
    nvafg,
    "x2;x3;",
    rf,
    "", 
    "", 0, 
    1
  );
  printf("Added factor id: %s\n", fid3);

  addVariable(nvafg, "x4", "Pose2", "", "", 0, 1);
  const char* fid4 = addFactor(
    nvafg,
    "x3;x4;",
    rf,
    "", 
    "", 0, 
    1
  );
  printf("Added factor id: %s\n", fid4);

  // and prior factor indicating the starting location
  pf = new_PriorPose3(normal);
  const char* fid5 = addFactor(
      nvafg,
      "x0;",
      pf,
      "", 
      "", 0, 
      1
  );
  printf("Added factor id: %s\n", fid5);

  // Solve the basic graph
  startWorker_solveParametric(nvafg);

  // wait for solve to finish (THIS IS THE OVERSIMPLEFIED PART)

  // delete variables now marginalized out
  deleteFactorById(nvafg, fid1);
  deleteFactorById(nvafg, fid2);
  deleteVariable(nvafg, "x0");
  deleteVariable(nvafg, "x1");

  // GET MEAN AND COV OF X2 AND USE AS PRIOR
  VariableDFG* X2 = NULL;
  X2 = getVariable(nvafg, "x2");
  RVec_f64* mn_ = getPPEMean(X2, "parametric"); 
  RVec_f64* cv_ = getPPECov(X2, "parametric");
  // and prior factor by reusing the existing estimate for the oldest variable left in the graph
  freeR(normal); freeR(pf);
  normal = new_FullNormal(6,mn,cv); // FIXME fix, use mn_, cv_ instead
  pf = new_PriorPose3(normal);
  const char* fid6 = addFactor(
      nvafg,
      "x2;",
      pf,
      "", 
      "", 0, 
      1
  );
  printf("Added factor id: %s\n", fid6);

  addVariable(nvafg, "x5", "Pose2", "", "", 0, 1);
  const char* fid7 = addFactor(
    nvafg,
    "x4;x5;",
    rf,
    "", 
    "", 0, 
    1
  );
  addVariable(nvafg, "x6", "Pose2", "", "", 0, 1);
  const char* fid8 = addFactor(
    nvafg,
    "x5;x6;",
    rf,
    "", 
    "", 0, 
    1
  );

  // Solve the basic graph
  startWorker_solveParametric(nvafg);

  // See other examples for combining other data including IMU, Camera, Lidar

  freeR(pf); freeR(rf); freeR(normal); freeR(X2); freeR(nvafg); freeR(nvacl);
  printf("All done.\n");
  return 0;
}