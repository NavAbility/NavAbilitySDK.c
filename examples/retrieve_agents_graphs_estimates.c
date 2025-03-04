
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
  agents = getAgents(nvacl); // must freeR(agents) later
  printf("get agents length: %ld\n", length(agents));
  for (int i = 0; i < length(agents); i++) {
    printf("agent label: %s\n", getLabel(getIndex(agents,i)));
  }
  freeR(agents);


  // // list all factor graphs
  RVec_Factorgraphs* fgs = NULL;
  // fgs = listGraphs(nvacl); // must freeR(fgs) later

  // printf("listGraphs length: %ld\n", length(fgs));
  // for (int i = 0; i < length(fgs); i++) {
  //   printf("factor graph label: %s\n", getLabel(getIndex(fgs,i)));
  // }
  // freeR(fgs);


  // // look at a specific factor graph
  // char* fglbl = "FG001";
  // NavAbilityDFG *nvafg = NULL;
  // nvafg = new_NavAbilityDFG(
  //     nvacl,
  //     fglbl,
  //     "BOT_01",
  //     NULL,
  //     false,  // addAgentIfAbsent
  //     false   // addGraphIfAbsent
  // ); // must freeR(nvafg) later

  // // list all variables in the 
  // RVec_String variables = NULL;
  // variables = listVariables(nvafg); // must freeR(nvafg) later
  // printf("Factorgraph %s has %ld variables\n", fglbl, length(agents));
  // for (int i = 0; i < 10; i++) {
  //   if i < length(variables) {
  //     printf("variable label: %s\n", getIndex(variables,i));
  //   }
  // }
  // freeR(variables);

  // // get one specific variable
  // VariableDFG* X1 = NULL;
  // X1 = getVariable(nvafg, "x1"); // must freeR(X1) later

  // // look at the numerically estimated value of X1
  // MeanMaxPPE* x1_ppe = NULL;
  // x1_ppe = getPPE(nvafg, X1);
  // printf(
  //   "x1_ppe, body pose in world frame x,y,th:\n %d, %d, %d\n", 
  //   getMean(x1_ppe)[0], getMean(x1_ppe)[1], getMean(x1_ppe)[2]
  // );
  //   freeR(x1_ppe);
  //   freeR(X1);

  // // See other examples for more concurrent usage, adding camera or lidar, launching more compute

  //   freeR(nvafg);  freeR(nvacl);
  printf("All done.\n");
  return 0;
}
