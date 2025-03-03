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
  double mn[3] = { 0.0 };
  // a basic covariance matrix
  double cv[9] = { 0.0 };
  cv[0] = 1.0;
  cv[4] = 1.0;
  cv[8] = 1.0;

    // Test that accessor methods use ref not Box<> whereby Rust retakes ownership and drops (leads to segfault)
    printf("getLabel(nvafg): %s\n", getLabel(nvafg));

  // BUILD A FACTOR GRAPH 
  // ROBOT STARTS ITS JOURNEY AT (0,0,0)
  // Assume a zero starting location
  // inputs: (nvafg,label,variableType, [_tags,_solvable,_timestamp,_nstime,_metadata])
  char* v = addVariable(nvafg, "x0", "Pose2", "TESTTAG;", "", 0, 1);
  printf("added variable id: %s\n", v);

  // and prior factor indicating the starting location at 0 ( a prior belief is used)
  normal = new_FullNormal(3,mn,cv); // new_ means must freeR(normal) later
  PriorPose2_FullNormal *f1 = NULL;
  f1 = new_PriorPose2(normal);  // new_ means must freeR(f1) later
  const char* fid1 = addFactor(
      nvafg,
      "x1;",
      f1,
      "TESTTAG;", 
      "", 0, 
      1
  ); freeR(f1); freeR(normal); // because new_ was used
  printf("Added factor id: %s\n", fid1);
  

  // ROBOT MOVES TO (10,0,0)
  v = addVariable(nvafg, "x1", "Pose2", "TESTTAG;", "", 0, 1);
  printf("added variable id: %s\n", v);
  
  // a relative motion factor indicating the robot moved 10 units in the x direction
  mn[0] = 10.0;
  normal = new_FullNormal(3,mn,cv);
  Pose2Pose2_FullNormal *f2 = NULL;
  f2 = new_Pose2Pose2(normal);  // new_ means must freeR(pf) later
  const char* fid2 = addFactor(
      nvafg,
      "x1;x2;",
      f2,
      "TESTTAG;", 
      "", 0, 
      1
  ); freeR(f2); freeR(normal); // because new_ was used
  printf("Added factor id: %s\n", fid2);

  // Solve the basic graph
  startWorker_solveParametric(nvafg);

  // Also see deleteFactor and deleteVariable

  // See the next example for retrieving variable values

  freeR(nvafg); freeR(nvacl);
  printf("All done.\n");
  return 0;
}
