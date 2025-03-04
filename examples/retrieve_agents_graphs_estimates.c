
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

  // list all agents
  RVec_Agent* agents = NULL;
  agents = getAgents(nvacl, ""); // must freeR(agents) later
  printf("get agents length: %ld\n", length(agents));
  for (int i = 0; i < length(agents); i++) {
    printf("agent label: %s\n", getLabel(getIndex(agents,i)));
  }
  freeR(agents);


  // // list all factor graphs
  RVec_NvaNode_Factorgraph* fgs = NULL;
  fgs = getFactorgraphs(nvacl, ""); // must freeR(fgs) later

  printf("getFactorgraphs length: %ld\n", length(fgs));

  NvaNode_Factorgraph* fgi = NULL;
  for (int i = 0; i < length(fgs); i++) {
    printf("factor graph label: %s\n", getLabel(getIndex(fgs,i)));
  }

  // look at a specific factor graph
  char* fglbl = getLabel(getIndex(fgs,length(fgs)-1));   freeR(fgs);

  NavAbilityDFG *nvafg = NULL;
  nvafg = new_NavAbilityDFG(
      nvacl,
      fglbl,
      "BOT_01",
      NULL,
      0,  // addAgentIfAbsent
      0   // addGraphIfAbsent
  ); // must freeR(nvafg) later


  // list all variables in the 
  RVec_String *variables = NULL;
  variables = listVariables(nvafg); // must freeR(nvafg) later
  printf("Factorgraph %s has %ld variables\n", fglbl, length(variables));
  for (int i = 0; i < 10; i++) {
    if (i < length(variables)) {
      printf("variable label: %s\n", getIndex(variables,i));
    }
  }
  freeR(variables);

  // get one specific variable
  VariableDFG* X1 = NULL;
  X1 = getVariable(nvafg, "x1"); // must freeR(X1) later

  // look at the numerically estimated value of X1
  RVec_f64* ppem = NULL;
  ppem = getPPEMean(X1, "parametric");
  if (0 < length(ppem)) {
    printf(
      "x1_ppe, body pose in world frame x,y,th:\n %d, %d, %d\n", 
      getIndex(ppem, 0), getIndex(ppem, 1), getIndex(ppem, 2)
    );
  } else {
    printf("x1 has no PPE mean yet, requires numerical operations such as a solve or prediction.\n"); 
  }

  // and a covariance estimate
  RVec_f64* ppec = NULL;
  ppec = getPPECov(X1, "parametric");
  
  freeR(ppem); freeR(ppec);
  freeR(X1);

  // See other examples for more concurrent usage, adding camera or lidar, launching more compute

  freeR(nvafg);  freeR(nvacl);
  printf("All done.\n");
  return 0;
}
