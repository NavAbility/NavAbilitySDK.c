
// WORK IN PROGRESS


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
  nvacl = NavAbilityClient_new(url,atk);

  NavAbilityDFG *nvafg = NULL;
  nvafg = NavAbilityDFG_new(
      nvacl,
      "FG001",
      "BOT_01",
      NULL,
      true,  // addAgentIfAbsent
      true   // addGraphIfAbsent
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
  normal = FullNormal_new(6,mn,cv); // must freeR(normal) later

  // BUILD A FACTOR GRAPH with multiple variables
  VariableDFG* v = NULL;
  FactorDFG* f = NULL;
  PriorPose3_FullNormal *pf = NULL;
  const char *vl[2]; 

  // inputs: (nvafg,label,variableType, [_tags,_solvable,_timestamp,_nstime,_metadata])
  v = addVariable(nvafg, "x0", "Pose2",  NULL, NULL, NULL, NULL, NULL); freeR(v);
  v = addVariable(nvafg, "x1", "Pose2",  NULL, NULL, NULL, NULL, NULL); freeR(v);
  vl[0] = "x0"; vl[1] = "x1";
  f = addFactor(nvafg, vl, 2, Pose3Pose3,  NULL, NULL, NULL, NULL, NULL); freeR(f);
  v = addVariable(nvafg, "x2", "Pose2",  NULL, NULL, NULL, NULL, NULL); freeR(v);
  vl[0] = "x1"; vl[1] = "x2";
  f = addFactor(nvafg, vl, 2, Pose3Pose3,  NULL, NULL, NULL, NULL, NULL); freeR(f);
  v = addVariable(nvafg, "x3", "Pose2",  NULL, NULL, NULL, NULL, NULL); freeR(v);
  vl[0] = "x2"; vl[1] = "x3";
  f = addFactor(nvafg, vl, 2, Pose3Pose3,  NULL, NULL, NULL, NULL, NULL); freeR(f);
  v = addVariable(nvafg, "x4", "Pose2",  NULL, NULL, NULL, NULL, NULL); freeR(v);
  vl[0] = "x3"; vl[1] = "x4";
  f = addFactor(nvafg, vl, 2, Pose3Pose3,  NULL, NULL, NULL, NULL, NULL); freeR(f);


  // and prior factor indicating the starting location
  pf = PriorPose3_new(normal);
    vl[0] = "x0";
    struct FactorDFG_PriorPose2_FullNormal *f = NULL;
  f = addFactor(vl,1,pf); freeR(f);freeR(pf);

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
  pf = PriorPose3_new(normal);
    vl[0] = "x2";
    struct FactorDFG_PriorPose2_FullNormal *f = NULL;
  f = addFactor(vl,1,pf); freeR(f);freeR(pf);

  v = addVariable(nvafg, "x5", "Pose2",  NULL, NULL, NULL, NULL, NULL); freeR(v);
  vl[0] = "x4"; vl[1] = "x5";
  f = addFactor(nvafg, vl, 2, Pose3Pose3,  NULL, NULL, NULL, NULL, NULL); freeR(f);
  v = addVariable(nvafg, "x6", "Pose2",  NULL, NULL, NULL, NULL, NULL); freeR(v);
  vl[0] = "x5"; vl[1] = "x6";
  f = addFactor(nvafg, vl, 2, Pose3Pose3,  NULL, NULL, NULL, NULL, NULL); freeR(f);

  // Solve the basic graph
  startWorker_solveParametric(nvafg);

  // See other examples for combining other data including IMU, Camera, Lidar

  freeR(nvafg); freeR(nvacl);
  printf("All done.\n");
  return 0;
}