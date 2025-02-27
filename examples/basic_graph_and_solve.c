
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
      1,  // addAgentIfAbsent
      1   // addGraphIfAbsent
  ); // must freeR(nvafg) later

  // a basic normal distribution
  FullNormal *normal = NULL;
  double mn[3] = { 0.0 };
  // a basic covariance matrix
  double cv[9] = { 0.0 };
  cv[0] = 1.0;
  cv[4] = 1.0;
  cv[8] = 1.0;
  normal = FullNormal_new(3,mn,cv); // must freeR(normal) later

  // BUILD A FACTOR GRAPH 
  // ROBOT STARTS ITS JOURNEY AT (0,0,0)
  // Assume a zero starting location
  // inputs: (nvafg,label,variableType, [_tags,_solvable,_timestamp,_nstime,_metadata])
  char* v = addVariable(nvafg, "x0", "Pose2", "TESTTAG;", "", 0, 1);

  // and prior factor indicating the starting location at 0 ( a prior belief is used)
  PriorPose2_FullNormal *pf = NULL;
  pf = PriorPose2_new(normal);
    const char *vl[1];
    vl[0] = "x0";
    struct FactorDFG_PriorPose2_FullNormal *f = NULL;
  f = addFactor(vl,1,pf);
    freeR(f); freeR(pf); free(vl);
  

  // ROBOT MOVES TO (1,0,0)
  char* v = addVariable(nvafg, "x1", "Pose2", "TESTTAG;", "", 0, 1);
  
  // a relative motion factor indicating the robot moved 10 units in the x direction
  freeR(normal);
  mn[0] = 10.0;
  normal = FullNormal_new(3,mn,cv);
  Pose2Pose2_FullNormal *pf = NULL;
  pf = Pose2Pose2_new(normal);
    const char *vl[2]; vl[0] = "x0"; vl[1] = "x1";
    struct FactorDFG_PriorPose2_FullNormal *f = NULL;
  f = addFactor(vl,2,pf);
    freeR(f);freeR(pf); free(vl);

  // Solve the basic graph
  startWorker_solveParametric(nvafg);

  // Also see deleteFactor and deleteVariable

  // See the next example for retrieving variable values

  freeR(nvafg); freeR(nvacl);
  printf("All done.\n");
  return 0;
}
