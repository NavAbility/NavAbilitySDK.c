// This is a simple example using the NavAbilitySDK to build and solve a graph using the NavAbility cloud infrastructure. 
// It demonstrates how to create a factor graph with multiple variables, add factors, solve the graph
// and manage the subscription to events. It uses a basic normal distribution for the factors.
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
  const char* orlb = NULL;
  nvacl = new_NavAbilityClient(url, atk, orlb);

  // Initialize the SubscriptionManager with the NavAbilityClient -- used later for synchronizing events
  SubscriptionManager* nvasm = NULL;
  nvasm = start_SubscriptionManager(nvacl, 64); // keep up to 64 events in the manager

  // The array char[] is allocated on the stack (not the heap), so its memory is automatically managed.  No free is needed.
  // - Stack allocation (`char fglbl[30];`): No need to free.
  // - Heap allocation (`char* fglbl = malloc(30);`): Must free with `free(fglbl);`.
  char fglbl[30];
  snprintf(fglbl, sizeof(fglbl), "FG_%05d", rand() % 100000);
  printf("fglbl: %s\n", fglbl);

  // Create a new NavAbilityDFG (Distributed Factor Graph) instance
  NavAbilityDFG *nvafg = NULL;
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
  const char* time_utc = "2020-01-01 06:30:01.250 UTC";
  char* v = addVariable(nvafg, "x0", "RoME.Pose2", "TESTTAG;", time_utc, 0, 1);
  printf("added variable label: %s\n", v);

  // and prior factor indicating the starting location at 0 ( a prior belief is used)
  normal = new_FullNormal(3,mn,cv); // new_ means must freeR(normal) later
  PriorPose2_FullNormal *f1 = NULL;
  f1 = new_PriorPose2(normal);  // new_ means must freeR(f1) later
  const char* flb1 = addFactor(
      nvafg,
      "x0;",
      f1,
      "TESTTAG;", 
      "", 0, 
      1
  ); freeR(f1); freeR(normal); // because new_ was used
  printf("Added factor: %s\n", flb1);
  

  // ROBOT MOVES TO (10,0,0)
  const char* time_z = "2020-01-01T06:30:08.500Z";
  v = addVariable(nvafg, "x1", "RoME.Pose2", "TESTTAG;", time_z, 0, 1);
  printf("added variable label: %s\n", v);
  
  // a relative motion factor indicating the robot moved 10 units in the x direction
  mn[0] = 10.0;
  normal = new_FullNormal(3,mn,cv);
  Pose2Pose2_FullNormal *f2 = NULL;
  f2 = new_Pose2Pose2(normal);  // new_ means must freeR(pf) later
  const char* flb2 = addFactor(
      nvafg,
      "x0;x1;",
      f2,
      "TESTTAG;", 
      "", 0, 
      1
  ); freeR(f2); freeR(normal); // because new_ was used
  printf("Added factor: %s\n", flb2);


  // Solve the basic graph and wait for the worker to finish, with timeout in milliseconds
  char* wrkid = solveGraphParametric(nvafg, "x1");
  bool success = block_on(nvasm, wrkid, 5000); 
  printf("Graph solve success: %d\n", success);
  freeR(wrkid); // free the worker id returned by solveGraphParametric

  // GET MEAN AND COV OF X2 AND USE AS PRIOR
  VariableDFG* X1 = NULL;
  X1 = getVariable(nvafg, "x1");
  RVec_f64* mn_ = getPPEMean(X1, "parametric");
  printf("Mean of x1: [%.3g %.3g %.3g]\n", 
    *getIndex(mn_, 0), *getIndex(mn_, 1), *getIndex(mn_, 2)
  ); 
  RVec_f64* cv_ = getPPECov(X1, "parametric");
  // TBD confirm column major order, covariance is symmetric
  printf("Cov of x1:\n[%.2g %.2g %.2g]\n[%.2g %.2g %.2g]\n[%.2g %.2g %.2g]\n", 
    *getIndex(cv_, 0), *getIndex(cv_, 1), *getIndex(cv_, 2),
    *getIndex(cv_, 3), *getIndex(cv_, 4), *getIndex(cv_, 5),
    *getIndex(cv_, 6), *getIndex(cv_, 7), *getIndex(cv_, 8)
  ); 

  // // TODO use getVariableState to extract the results (deprecate PPE)
  // StateValue* sv = getVariableState(dfg, variableLabel, "parametric");
  // if (sv != NULL) {
  //   printf("StateValue for variable %s:\n", variableLabel);
  //   RVec_f64* pt_ = getPoint(sv);
  //   printf(
  //     "point of %s:\n[%.3f %.3f %.3f %.3f %.3f %.3f]\n", variableLabel,
  //       *getIndex(pt_, 0), *getIndex(pt_, 1), *getIndex(pt_, 2),
  //       *getIndex(pt_, 3), *getIndex(pt_, 4), *getIndex(pt_, 5)
  //       );
  //   RVec_f64* cv_ = getCovariance(sv);
  //   printf(
  //     "vectorized cov of %s:\n[%.3f %.3f %.3f ...]\n", variableLabel,
  //       *getIndex(cv_, 0), *getIndex(cv_, 1), *getIndex(cv_, 2)
  //   );
  // } else {
  //   printf("Failed to get state for variable %s\n", variableLabel);
  // }
  // freeR(sv);

  // freeR(fglbl); // free the string created by sprintf
  freeR(nvafg); 
  freeR(nvasm);
  freeR(nvacl);
  printf("All done.\n");
  return 0;
}
