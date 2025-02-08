#include <stdlib.h>
#include <stdio.h>
#include <time.h>    // for clock_t, clock()
#include "NavAbilitySDK.h"
#include <string.h>


#define BILLION  1000000000.0

// File: main.c
//
// Sample library usage.
int main(void) {

    const char* url = getenv ("NVA_API_URL");
    printf("NVA_API_URL: %s\n", (url != NULL) ? "***" : "getenv returned NULL");
    const char* oid = getenv ("NVA_ORG_ID");
    printf("NVA_ORG_ID: %s\n", (oid != NULL) ? "***" : "getenv returned NULL");
    const char* atk = getenv ("NVA_API_TOKEN");
    printf("NVA_API_TOKEN: %s\n", (atk != NULL) ? "***" : "getenv returned NULL");

    struct timespec start, end;

        clock_gettime(CLOCK_REALTIME, &start);

    NavAbilityClient* nvacl = NULL;
    nvacl = NavAbilityClient_new(url,oid,atk);

        clock_gettime(CLOCK_REALTIME, &end);
        // time_spent = end - start
        double time_spent = (end.tv_sec - start.tv_sec) +
                            (end.tv_nsec - start.tv_nsec) / BILLION;
        printf("The elapsed time creating a NavAbilityClient is %f seconds\n", time_spent);
    
    char* api = get_apiurl(nvacl);
    printf("nvacl.apiurl = %s\n", "***");

    RVec_Agent* agents = NULL;
    agents = getAgents(nvacl);

    printf("get agents length: %ld\n", length(agents));
    int i;
    for (i = 0; i < length(agents); i++) {
        printf("getLabel(agents[%d]): %s\n", i, getLabel(get_index(agents,i)));
    }

    free_rvecagent(agents);

    NavAbilityBlobStore *store = NULL;
    store = NavAbilityBlobStore_new(nvacl, "default");

    printf("getLabel(store): %s\n", getLabel(store));

    NavAbilityDFG *nvafg = NULL;
    nvafg = NavAbilityDFG_new(
        nvacl,
        "FG001",
        "BOT_01",
        NULL,
        NULL,
        NULL
    );

    // Test that accessor methods use ref not Box<> whereby Rust retakes ownership and drops (leads to segfault)
    printf("getLabel(nvafg): %s\n", getLabel(nvafg));

    BlobEntry *be = NULL;
    be = BlobEntry_basic("test_entry","text/plain");
    printf("getLabel(bentry): %s\n", getLabel(be));
    free_BlobEntry(be);

    FullNormal *normal = NULL;
    double mn[3] = { 0.0 };
    double cv[9] = { 0.0 };
    cv[0] = 1.0;
    cv[4] = 1.0;
    cv[8] = 1.0;
    normal = FullNormal_new(3,mn,cv);
    
    Pose3Pose3_FullNormal *pf = NULL;
    pf = Pose3Pose3(normal);

    free_Pose3Pose3(pf);
    free_FullNormal(normal);

    printf("About to getVariable with nvafg\n");
    // test getVariable
    VariableDFG *variable = NULL;
    char* vlb = "test";
    variable = getVariable(nvafg, vlb);
    if (variable) {
        //
    } else {
        printf("WARN: variable '%s' not found in '%s'\n", vlb, "TODO"/*getLabel(nvafg)*/);
    }
    // int x;
    // typeof(x) y;

    free_VariableDFG(variable);
    free_NavAbilityDFG(nvafg);
    free_NavAbilityBlobStore(store);
    free_NavAbilityClient(nvacl);
    free_cstr(api); // possibly redundant but doesn't hurt

    printf("check for NullPtr arg handling\n");
    get_apiurl(NULL);
    getAgents(NULL); 
    getVariable(NULL,"");

    printf("All done.\n");
    return 0;
}

    // Error2 *e = NULL;
    // int result = 0;
    
    // char *s = NULL;

    // result = get_some_cstr_2(&s);
    // if (0 == result) {
    //     //printf("get_some_cstr_2 returned %s\n", s);
    //     free(s);
    //     s = NULL;
    // } else {
    //     printf("get_some_cstr_2 Result = %d\n", result);
    //     return 10;
    // }

    // e = error_new();
    // const char *msg = error_msg_get(e);
    // if (msg) {
    //     printf("error message = %s\n", msg);
    //     printf("error code = %d\n", error_code_get(e));
    // } else {
    //     printf("Error msg is null :-/\n");
    //     return 1;
    // }

    // error_free(e);

    // e = NULL;
    // result = error_create_with_result(&e);
    // if (result == 0) {
    //     printf("error message = %s\n", error_msg_get(e));
    //     printf("error code = %d\n", error_code_get(e));

    //     printf("Result of freeing %d\n", error_free_with_result(&e));
    //     printf("Value of e = %p (expecting nil)\n", e);
    // } else {
    //     printf("Error: error_create_with_result = %d\n", result);
    // }