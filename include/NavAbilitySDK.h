#ifdef __cplusplus
extern "C" {
#endif

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct Agent Agent;

/**
 * A `BlobEntry` is a small amount of structured data that holds contextual/reference information to find an actual blob.
 * A `BlobEntry` does not have to point to a particular blobId, e.g. storing metadata or providing topological context.
 * Many `BlobEntry`s can exist on different graph nodes spanning Robots, and Sessions which can all reference the same `Blob`.
 * A `BlobEntry` is also a equivalent to a bridging entry between local `.originId` and a remotely assigned `.blobIds`.
 *
 * Notes:
 * - `blobId`s should be unique within a blobstore and are immutable; or
 *   - if blobless, should have UUID("00000000-0000-0000-000000000000").
 */
typedef struct BlobEntry BlobEntry;

typedef struct FactorDFG_Point2Point2_FullNormal FactorDFG_Point2Point2_FullNormal;

typedef struct FactorDFG_Point3Point3_FullNormal FactorDFG_Point3Point3_FullNormal;

typedef struct FactorDFG_Pose2Pose2_FullNormal FactorDFG_Pose2Pose2_FullNormal;

typedef struct FactorDFG_Pose3Pose3_FullNormal FactorDFG_Pose3Pose3_FullNormal;

typedef struct FactorDFG_PriorPoint2_FullNormal FactorDFG_PriorPoint2_FullNormal;

typedef struct FactorDFG_PriorPoint3_FullNormal FactorDFG_PriorPoint3_FullNormal;

typedef struct FactorDFG_PriorPose2_FullNormal FactorDFG_PriorPose2_FullNormal;

typedef struct FactorDFG_PriorPose3_FullNormal FactorDFG_PriorPose3_FullNormal;

/**
 * Multidimensional normal distribution specified by means and a covariance matrix.
 */
typedef struct FullNormal FullNormal;

typedef struct NavAbilityBlobStore NavAbilityBlobStore;

typedef struct NavAbilityClient NavAbilityClient;

typedef struct NavAbilityDFG NavAbilityDFG;

typedef struct NvaNode_Factorgraph NvaNode_Factorgraph;

typedef struct Point2Point2_FullNormal Point2Point2_FullNormal;

typedef struct Point3Point3_FullNormal Point3Point3_FullNormal;

/**
 * Create a Pose2->Pose2 factor with a distribution Z representing the (x,y,th) relationship
 * between the variables, e.g. `FullNormal([1;0;0], diagm(0.01*ones(3)))`.
 *
 * Example value: Z = `FullNormal(zeros(3), diagm(0.01*ones(3)))`.
 */
typedef struct Pose2Pose2_FullNormal Pose2Pose2_FullNormal;

/**
 * Create a Pose3->Pose3 factor with a distribution Z representing the (x,y,z,a,b,c) relationship
 * between the variables, e.g. `FullNormal([1;zeros(5)], diagm(0.01*ones(6)))`.
 *
 * Example value: Z = `FullNormal(zeros(6), diagm(0.01*ones(6)))`.
 */
typedef struct Pose3Pose3_FullNormal Pose3Pose3_FullNormal;

typedef struct PriorPoint2_FullNormal PriorPoint2_FullNormal;

typedef struct PriorPoint3_FullNormal PriorPoint3_FullNormal;

/**
 * Create a PriorPose2 factor with a distribution Z representing the (x,y,th) relationship
 * between the variables, e.g. `FullNormal([1;0;0], diagm(0.01*ones(3)))`.
 *
 * Example value: Z = `FullNormal(zeros(3), diagm(0.01*ones(3)))`.
 */
typedef struct PriorPose2_FullNormal PriorPose2_FullNormal;

/**
 * Create a PriorPose3 factor with a distribution Z representing the (x,y,z,a,b,c) relationship
 * between the variables, e.g. `FullNormal([1;zeros(5)], diagm(0.01*ones(6)))`.
 *
 * Example value: Z = `FullNormal(zeros(6), diagm(0.01*ones(6)))`.
 */
typedef struct PriorPose3_FullNormal PriorPose3_FullNormal;

typedef struct String String;

/**
 * Manages subscription events from NavAbilityClient subscriptions
 * SPECIAL NOTE1, can use standalone Self::subscription_listener(_)
 * SPECIAL_NOTE2, both non-blocking and blocking interfaces are provided (for wasm or tokio)
 */
typedef struct SubscriptionManager SubscriptionManager;

typedef struct SubscriptionManagerI SubscriptionManagerI;

typedef struct Tuple Tuple;

/**
 * The Variable information packed in a way that accomdates multi-lang using json.
 */
typedef struct VariableDFG VariableDFG;

typedef struct RVec_Agent {
  struct Agent *ptr;
  size_t len;
} RVec_Agent;

typedef struct RVec_NvaNode_Factorgraph {
  struct NvaNode_Factorgraph *ptr;
  size_t len;
} RVec_NvaNode_Factorgraph;

typedef struct RVec_String {
  struct String *ptr;
  size_t len;
} RVec_String;

typedef struct RVec_f64 {
  double *ptr;
  size_t len;
} RVec_f64;

struct BlobEntry *BlobEntry_basic(const char *label, const char *mimeType);

char *addAffordance_kNNvisual(const struct NavAbilityDFG *nvafg,
                              const char *variableLabel,
                              const char *mapsessions,
                              size_t n_matches,
                              size_t total_n_matches);

const char *addAgentBlobEntry(const struct NavAbilityClient *nvacl_,
                              const char *agent_label,
                              const struct BlobEntry *entry_);

char *addBlob(const struct NavAbilityBlobStore *nvabs_,
              const char *filename,
              const char *mime,
              const char *data,
              size_t nbytes);

char *addVariable(const struct NavAbilityDFG *nvafg,
                  const char *label,
                  const char *variableType,
                  const char *_tags,
                  const char *_timestamp,
                  size_t _nstime,
                  size_t _solvable);

char *addVariableBlobEntry(const struct NavAbilityDFG *nvafg_,
                           const char *variable_label,
                           const struct BlobEntry *entry_);

struct Tuple *assign_SubscriptionManager(const struct NavAbilityClient *_nvacl,
                                         size_t size,
                                         struct Tuple *tup_,
                                         void (*callback)(void*));

bool block_on(struct SubscriptionManager *_nvasm, const char *wrk_id, size_t tout_millis);

char *computeImageWhitebalance(const struct NavAbilityDFG *nvafg,
                               const char *v_lbl,
                               const char *be_lbl,
                               const char *be_out_lbl);

char *computeLidarRegistration(const struct NavAbilityDFG *nvafg,
                               const char *v1_lbl,
                               const char *be1_lbl,
                               const char *v2_lbl,
                               const char *be2_lbl);

void deleteAgentBlobEntry(const struct NavAbilityClient *nvacl_,
                          const char *agent_label,
                          const char *bentry_label);

void deleteBlob(const struct NavAbilityClient *nvacl_, const char *blob_id, const char *store);

void deleteFactorById(const struct NavAbilityDFG *nvafg, const char *fid);

void deleteVariable(const struct NavAbilityDFG *nvafg, const char *label);

char *deriveRobotConfig(const struct NavAbilityDFG *nvafg);

struct SubscriptionManagerI *dummy(void);

void free_Agent(struct Agent*);

void free_BlobEntry(struct BlobEntry*);

void free_FactorDFG_Point2Point2_FullNormal(struct FactorDFG_Point2Point2_FullNormal*);

void free_FactorDFG_Point3Point3_FullNormal(struct FactorDFG_Point3Point3_FullNormal*);

void free_FactorDFG_Pose2Pose2_FullNormal(struct FactorDFG_Pose2Pose2_FullNormal*);

void free_FactorDFG_Pose3Pose3_FullNormal(struct FactorDFG_Pose3Pose3_FullNormal*);

void free_FactorDFG_PriorPoint2_FullNormal(struct FactorDFG_PriorPoint2_FullNormal*);

void free_FactorDFG_PriorPoint3_FullNormal(struct FactorDFG_PriorPoint3_FullNormal*);

void free_FactorDFG_PriorPose2_FullNormal(struct FactorDFG_PriorPose2_FullNormal*);

void free_FactorDFG_PriorPose3_FullNormal(struct FactorDFG_PriorPose3_FullNormal*);

void free_FullNormal(struct FullNormal*);

void free_NavAbilityBlobStore(struct NavAbilityBlobStore*);

void free_NavAbilityClient(struct NavAbilityClient*);

void free_NavAbilityDFG(struct NavAbilityDFG*);

void free_Point2Point2(struct Point2Point2_FullNormal*);

void free_Point3Point3(struct Point3Point3_FullNormal*);

void free_Pose2Pose2(struct Pose2Pose2_FullNormal*);

void free_Pose3Pose3(struct Pose3Pose3_FullNormal*);

void free_PriorPoint2(struct PriorPoint2_FullNormal*);

void free_PriorPoint3(struct PriorPoint3_FullNormal*);

void free_PriorPose2(struct PriorPose2_FullNormal*);

void free_PriorPose3(struct PriorPose3_FullNormal*);

void free_RVec_Agent(struct RVec_Agent *rvec);

void free_RVec_NvaNode_Factorgraph(struct RVec_NvaNode_Factorgraph *rvec);

void free_RVec_String(struct RVec_String *rvec);

void free_RVec_f64(struct RVec_f64 *rvec);

void free_SubscriptionManager(struct SubscriptionManager*);

void free_VariableDFG(struct VariableDFG*);

void free_cstr(char *pointer);

struct RVec_Agent *getAgents(const struct NavAbilityClient *_nvacl, const char *label_contains);

struct RVec_NvaNode_Factorgraph *getFactorgraphs(const struct NavAbilityClient *_nvacl,
                                                 const char *label_contains);

struct Agent *getIndex_RVec_Agent(const struct RVec_Agent *rv_agent, size_t index);

struct NvaNode_Factorgraph *getIndex_RVec_NvaNode_Factorgraph(const struct RVec_NvaNode_Factorgraph *rv_fgs,
                                                              size_t index);

char *getIndex_RVec_String(const struct RVec_String *rv_s, size_t index);

const double *getIndex_RVec_f64(const struct RVec_f64 *rv_s, size_t index);

const char *getLabel_Agent(const struct Agent *agent);

const char *getLabel_BlobEntry(const struct BlobEntry *bentry);

const char *getLabel_NavAbilityBlobStore(const struct NavAbilityBlobStore *store);

const char *getLabel_NavAbilityClient(const struct Agent *input);

const char *getLabel_NavAbilityDFG(const struct NavAbilityDFG *input);

const char *getLabel_NvaNode_Factorgraph(const struct NvaNode_Factorgraph *input);

struct RVec_f64 *getPPECov(const struct VariableDFG *vari_, const char *_solveKey);

struct RVec_f64 *getPPEMean(const struct VariableDFG *vari_, const char *_solveKey);

struct VariableDFG *getVariable(const struct NavAbilityDFG *nvafg, const char *label);

char *get_apiurl(const struct NavAbilityClient *nvacl);

size_t length_RVec_Agent(const struct RVec_Agent *rv_agent);

size_t length_RVec_NvaNode_Factorgraph(const struct RVec_NvaNode_Factorgraph *rv_fgs);

size_t length_RVec_String(const struct RVec_String *rv_s);

size_t length_RVec_f64(const struct RVec_f64 *rv_s);

struct RVec_String *listVariables(const struct NavAbilityDFG *_nvafg);

struct BlobEntry *new_BlobEntry(const char *blobId,
                                const char *label,
                                const char *blobstore,
                                const char *origin,
                                int64_t size,
                                const char *description,
                                const char *mimeType,
                                const char *metadata,
                                const char *timestamp);

struct FullNormal *new_FullNormal(size_t dim, const double *array_mean, const double *array_covr);

struct NavAbilityBlobStore *new_NavAbilityBlobStore(const struct NavAbilityClient *nvacl,
                                                    const char *label);

struct NavAbilityClient *new_NavAbilityClient(const char *api_url, const char *api_token);

struct NavAbilityDFG *new_NavAbilityDFG(const struct NavAbilityClient *_nvacl,
                                        const char *fgLabel,
                                        const char *agentLabel,
                                        const char *storeLabel,
                                        size_t addAgentIfAbsent,
                                        size_t addGraphIfAbsent);

struct Point2Point2_FullNormal *new_Point2Point2(const struct FullNormal *Z);

struct Point3Point3_FullNormal *new_Point3Point3(const struct FullNormal *Z);

struct Pose2Pose2_FullNormal *new_Pose2Pose2(const struct FullNormal *Z);

struct Pose3Pose3_FullNormal *new_Pose3Pose3(const struct FullNormal *Z);

struct PriorPoint2_FullNormal *new_PriorPoint2(const struct FullNormal *Z);

struct PriorPoint3_FullNormal *new_PriorPoint3(const struct FullNormal *Z);

struct PriorPose2_FullNormal *new_PriorPose2(const struct FullNormal *Z);

struct PriorPose3_FullNormal *new_PriorPose3(const struct FullNormal *Z);

struct Tuple *new_SubsChannels(void);

char *new_uuid4(void);

char *solveGraphParametric(const struct NavAbilityDFG *nvafg, const char *variableLabel);

const char *updateAgentMetadata(const struct NavAbilityClient *_nvacl,
                                const char *agent_label,
                                const char *metadata);

#ifdef __cplusplus
}
#endif


// manually define the prototypes for the FactorDFG functions we need

#ifdef __cplusplus
extern "C" {
#endif

// // FIXME -- DONT USE YET
// #define GET_MACRO(_1, _2, _3, NAME, ...) NAME
// #define SArr(...) GET_MACRO(__VA_ARGS__, SArr3, SArr2, SArr1)(__VA_ARGS__)
// // #define SArr0() printf("%s","")
// #define SArr1(a) strcat(strcat("",a),";")
// #define SArr2(a, b) strcat(strcat(strcat(strcat("",a),";"),b),";")
// #define SArr3(a, b, c) strcat(strcat(strcat(strcat(strcat("",a),";"),b),";"),";")


// typedef struct FactorDFG_##FNCTYPE ##FNCTYPE;

// struct FactorDFG_##FNCTYPE *
#define GEN_ADD_FACTOR(FNCTYPE) \
    const char* add_FactorDFG_##FNCTYPE( \
        const struct NavAbilityDFG *nvafg, \
        const char *_varlbls, \
        const struct FNCTYPE *fnc, \
        const char *_tags, \
        const char *_timestamp, \
        size_t _nstime, \
        size_t _solvable \
    );

GEN_ADD_FACTOR(PriorPoint2_FullNormal);
GEN_ADD_FACTOR(PriorPoint3_FullNormal);
GEN_ADD_FACTOR(PriorPose2_FullNormal);
GEN_ADD_FACTOR(PriorPose3_FullNormal);
GEN_ADD_FACTOR(Point2Point2_FullNormal);
GEN_ADD_FACTOR(Point3Point3_FullNormal);
GEN_ADD_FACTOR(Pose2Pose2_FullNormal);
GEN_ADD_FACTOR(Pose3Pose3_FullNormal);


#ifdef __cplusplus
}
#endif



#ifndef __cplusplus

// Overloading via C macros using _Generic

// https://stackoverflow.com/a/73458289 
// _Generic wont work since Rust type sizes likely unknown to C compiler
// https://thelinuxcode.com/function-overloading-c/
// http://www.robertgamble.net/2012/01/c11-generic-selections.html
// https://stackoverflow.com/a/76240760
// printf("[%s] @ line [%d]: \n", #obj, __LINE__);  


#define length(obj)                                           \
    _Generic(obj,                                             \
        RVec_f64*:                  length_RVec_f64,          \
        RVec_String*:               length_RVec_String,       \
        RVec_Agent*:                length_RVec_Agent,        \
        RVec_NvaNode_Factorgraph*:  length_RVec_NvaNode_Factorgraph \
    ) (obj)


#define getLabel(obj)                                         \
    _Generic(obj,                                             \
        Agent*:                getLabel_Agent,                \
        BlobEntry*:            getLabel_BlobEntry,            \
        NavAbilityBlobStore*:  getLabel_NavAbilityBlobStore,  \
        NavAbilityDFG*:        getLabel_NavAbilityDFG,        \
        struct NvaNode_Factorgraph*:  getLabel_NvaNode_Factorgraph \
    ) (obj)

    
#define getIndex(obj,i)                                       \
    _Generic(obj,                                             \
        RVec_f64*:                  getIndex_RVec_f64,        \
        RVec_String*:               getIndex_RVec_String,     \
        RVec_Agent*:                getIndex_RVec_Agent,      \
        RVec_NvaNode_Factorgraph*:  getIndex_RVec_NvaNode_Factorgraph \
    ) (obj,i)


#define freeR(obj)                                            \
    _Generic(obj,                                             \
        char*:                    free_cstr,                  \
        Agent*:                   free_Agent,                 \
        RVec_f64*:                free_RVec_f64,              \
        RVec_String*:             free_RVec_String,           \
        RVec_Agent*:              free_RVec_Agent,            \
        RVec_NvaNode_Factorgraph*: free_RVec_NvaNode_Factorgraph, \
        BlobEntry*:               free_BlobEntry,             \
        NavAbilityClient*:        free_NavAbilityClient,      \
        NavAbilityBlobStore*:     free_NavAbilityBlobStore,   \
        SubscriptionManager*:     free_SubscriptionManager,   \
        NavAbilityDFG*:           free_NavAbilityDFG,         \
        VariableDFG*:             free_VariableDFG,           \
        FullNormal*:              free_FullNormal,            \
        PriorPoint2_FullNormal*:  free_PriorPoint2,           \
        PriorPoint3_FullNormal*:  free_PriorPoint3,           \
        PriorPose2_FullNormal*:   free_PriorPose2,            \
        PriorPose3_FullNormal*:   free_PriorPose3,            \
        Point2Point2_FullNormal*: free_Point2Point2,          \
        Point3Point3_FullNormal*: free_Point3Point3,          \
        Pose2Pose2_FullNormal*:   free_Pose2Pose2,            \
        Pose3Pose3_FullNormal*:   free_Pose3Pose3,            \
        struct FactorDFG_PriorPoint2_FullNormal*:    free_FactorDFG_PriorPoint2_FullNormal,              \
        struct FactorDFG_PriorPoint3_FullNormal*:    free_FactorDFG_PriorPoint3_FullNormal,              \
        struct FactorDFG_PriorPose2_FullNormal*:    free_FactorDFG_PriorPose2_FullNormal,                \
        struct FactorDFG_PriorPose3_FullNormal*:    free_FactorDFG_PriorPose3_FullNormal,                \
        struct FactorDFG_Point2Point2_FullNormal*:    free_FactorDFG_Point2Point2_FullNormal,            \
        struct FactorDFG_Point3Point3_FullNormal*:    free_FactorDFG_Point3Point3_FullNormal,            \
        struct FactorDFG_Pose2Pose2_FullNormal*:    free_FactorDFG_Pose2Pose2_FullNormal,                \
        struct FactorDFG_Pose3Pose3_FullNormal*:    free_FactorDFG_Pose3Pose3_FullNormal                \
    ) (obj)


#define addFactor(nfg,vl,obj,tags,_timestamp,_nstime,_solvable)                                                   \
    _Generic(obj,                                                             \
        PriorPoint2_FullNormal*:    add_FactorDFG_PriorPoint2_FullNormal,        \
        PriorPoint3_FullNormal*:    add_FactorDFG_PriorPoint3_FullNormal,        \
        PriorPose2_FullNormal*:    add_FactorDFG_PriorPose2_FullNormal,        \
        PriorPose3_FullNormal*:    add_FactorDFG_PriorPose3_FullNormal,        \
        Point2Point2_FullNormal*:    add_FactorDFG_Point2Point2_FullNormal,        \
        Point3Point3_FullNormal*:    add_FactorDFG_Point3Point3_FullNormal,        \
        Pose2Pose2_FullNormal*:    add_FactorDFG_Pose2Pose2_FullNormal,        \
        Pose3Pose3_FullNormal*:    add_FactorDFG_Pose3Pose3_FullNormal        \
    ) (nfg,vl,obj,tags,_timestamp,_nstime,_solvable)


//

#endif



#ifdef __cplusplus

size_t length(const struct RVec_f64* s) { return length_RVec_f64(s); }
size_t length(const struct RVec_String* s) { return length_RVec_String(s); }
size_t length(const struct RVec_Agent* s) { return length_RVec_Agent(s); }
size_t length(const struct RVec_NvaNode_Factorgraph* s) { return length_RVec_NvaNode_Factorgraph(s); }


const char* getLabel(Agent* s) { return getLabel_Agent(s); }
const char* getLabel(BlobEntry* s) { return getLabel_BlobEntry(s); }
const char* getLabel(NavAbilityBlobStore* s) { return getLabel_NavAbilityBlobStore(s); }
const char* getLabel(NavAbilityDFG* s) { return getLabel_NavAbilityDFG(s); }
const char* getLabel(struct NvaNode_Factorgraph* s) { return getLabel_NvaNode_Factorgraph(s); }

const double* getIndex(RVec_f64* s, size_t i) { return getIndex_RVec_f64(s, i); }
char* getIndex(RVec_String* s, size_t i) { return getIndex_RVec_String(s, i); }
Agent* getIndex(RVec_Agent* s, size_t i) { return getIndex_RVec_Agent(s, i); }
NvaNode_Factorgraph* getIndex(RVec_NvaNode_Factorgraph* s, size_t i) { return getIndex_RVec_NvaNode_Factorgraph(s, i); }


void freeR(char* s) { free_cstr(s); }
void freeR(struct Agent* s) { free_Agent(s); }
void freeR(struct RVec_f64* s) { free_RVec_f64(s); }
void freeR(struct RVec_String* s) { free_RVec_String(s); }
void freeR(struct RVec_Agent* s) { free_RVec_Agent(s); }
void freeR(struct RVec_NvaNode_Factorgraph* s) { free_RVec_NvaNode_Factorgraph(s); }
void freeR(struct BlobEntry* s) { free_BlobEntry(s); }
void freeR(struct NavAbilityClient* s) { free_NavAbilityClient(s); }
void freeR(struct SubscriptionManager* s) { free_SubscriptionManager(s); }
void freeR(struct NavAbilityBlobStore* s) { free_NavAbilityBlobStore(s); }
void freeR(struct NavAbilityDFG* s) { free_NavAbilityDFG(s); }
void freeR(struct VariableDFG* s) { free_VariableDFG(s); }
void freeR(struct FullNormal* s) { free_FullNormal(s); }
void freeR(struct PriorPoint2_FullNormal* s) { free_PriorPoint2(s); }
void freeR(struct PriorPoint3_FullNormal* s) { free_PriorPoint3(s); }
void freeR(struct PriorPose2_FullNormal* s) { free_PriorPose2(s); }
void freeR(struct PriorPose3_FullNormal* s) { free_PriorPose3(s); }
void freeR(struct Point2Point2_FullNormal* s) { free_Point2Point2(s); }
void freeR(struct Point3Point3_FullNormal* s) { free_Point3Point3(s); }
void freeR(struct Pose2Pose2_FullNormal* s) { free_Pose2Pose2(s); }
void freeR(struct Pose3Pose3_FullNormal* s) { free_Pose3Pose3(s); }
void freeR(struct FactorDFG_PriorPoint2_FullNormal* s) { free_FactorDFG_PriorPoint2_FullNormal(s); }
void freeR(struct FactorDFG_PriorPoint3_FullNormal* s) { free_FactorDFG_PriorPoint3_FullNormal(s); }
void freeR(struct FactorDFG_PriorPose2_FullNormal* s) { free_FactorDFG_PriorPose2_FullNormal(s); }
void freeR(struct FactorDFG_PriorPose3_FullNormal* s) { free_FactorDFG_PriorPose3_FullNormal(s); }
void freeR(struct FactorDFG_Point2Point2_FullNormal* s) { free_FactorDFG_Point2Point2_FullNormal(s); }
void freeR(struct FactorDFG_Point3Point3_FullNormal* s) { free_FactorDFG_Point3Point3_FullNormal(s); }
void freeR(struct FactorDFG_Pose2Pose2_FullNormal* s) { free_FactorDFG_Pose2Pose2_FullNormal(s); }
void freeR(struct FactorDFG_Pose3Pose3_FullNormal* s) { free_FactorDFG_Pose3Pose3_FullNormal(s); }


//nfg,vl,obj,tags,_timestamp,_nstime,_solvable
const char* addFactor(
    NavAbilityDFG* nfg,
    const char* vls,
    PriorPoint2_FullNormal* s,
    const char* tags,
    const char *_timestamp,
    size_t _nstime,
    size_t _solvable
) { return add_FactorDFG_PriorPoint2_FullNormal(nfg,vls,s,tags,_timestamp,_nstime,_solvable); }
const char* addFactor(
    NavAbilityDFG* nfg,
    const char* vls,
    PriorPoint3_FullNormal* s,
    const char* tags,
    const char *_timestamp,
    size_t _nstime,
    size_t _solvable
) { return add_FactorDFG_PriorPoint3_FullNormal(nfg,vls,s,tags,_timestamp,_nstime,_solvable); }
const char* addFactor(
    NavAbilityDFG* nfg,
    const char* vls,
    PriorPose2_FullNormal* s,
    const char* tags,
    const char *_timestamp,
    size_t _nstime,
    size_t _solvable
) { return add_FactorDFG_PriorPose2_FullNormal(nfg,vls,s,tags,_timestamp,_nstime,_solvable); }
const char* addFactor(
    NavAbilityDFG* nfg,
    const char* vls,
    PriorPose3_FullNormal* s,
    const char* tags,
    const char *_timestamp,
    size_t _nstime,
    size_t _solvable
) { return add_FactorDFG_PriorPose3_FullNormal(nfg,vls,s,tags,_timestamp,_nstime,_solvable); }
const char* addFactor(
    NavAbilityDFG* nfg,
    const char* vls,
    Point2Point2_FullNormal* s,
    const char* tags,
    const char *_timestamp,
    size_t _nstime,
    size_t _solvable
) { return add_FactorDFG_Point2Point2_FullNormal(nfg,vls,s,tags,_timestamp,_nstime,_solvable); }
const char* addFactor(
    NavAbilityDFG* nfg,
    const char* vls,
    Point3Point3_FullNormal* s,
    const char* tags,
    const char *_timestamp,
    size_t _nstime,
    size_t _solvable
) { return add_FactorDFG_Point3Point3_FullNormal(nfg,vls,s,tags,_timestamp,_nstime,_solvable); }
const char* addFactor(
    NavAbilityDFG* nfg,
    const char* vls,
    Pose2Pose2_FullNormal* s,
    const char* tags,
    const char *_timestamp,
    size_t _nstime,
    size_t _solvable
) { return add_FactorDFG_Pose2Pose2_FullNormal(nfg,vls,s,tags,_timestamp,_nstime,_solvable); }
const char* addFactor(
    NavAbilityDFG* nfg,
    const char* vls,
    Pose3Pose3_FullNormal* s,
    const char* tags,
    const char *_timestamp,
    size_t _nstime,
    size_t _solvable
) { return add_FactorDFG_Pose3Pose3_FullNormal(nfg,vls,s,tags,_timestamp,_nstime,_solvable); }


#endif
