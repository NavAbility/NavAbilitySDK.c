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

/**
 * Multidimensional normal distribution specified by means and a covariance matrix.
 */
typedef struct FullNormal FullNormal;

typedef struct NavAbilityBlobStore NavAbilityBlobStore;

typedef struct NavAbilityClient NavAbilityClient;

typedef struct NavAbilityDFG NavAbilityDFG;

typedef struct NvaNode_T NvaNode_T;

typedef struct Option_Vec______c_char Option_Vec______c_char;

typedef struct Option______c_char Option______c_char;

typedef struct Option_____c_char Option_____c_char;

typedef struct Option_i64 Option_i64;

typedef struct Option_usize Option_usize;

/**
 * Create a Pose3->Pose3 factor with a distribution Z representing the (x,y,z,a,b,c) relationship
 * between the variables, e.g. `FullNormal([1;zeros(5)], diagm(0.01*ones(6)))`.
 *
 * Example value: Z = `FullNormal(zeros(6), diagm(0.01*ones(6)))`.
 */
typedef struct Pose3Pose3_FullNormal Pose3Pose3_FullNormal;

/**
 * The Variable information packed in a way that accomdates multi-lang using json.
 */
typedef struct VariableDFG VariableDFG;

typedef struct RVec_Agent {
  struct Agent *ptr;
  size_t len;
} RVec_Agent;

struct BlobEntry *BlobEntry_basic(const char *label, const char *mimeType);

struct BlobEntry *BlobEntry_new(const char *blobId,
                                const char *label,
                                const char *blobstore,
                                const char *hash,
                                const char *origin,
                                int64_t size,
                                const char *description,
                                const char *mimeType,
                                const char *metadata,
                                const char *timestamp);

struct FullNormal *FullNormal_new(size_t dim, const double *array_mean, const double *array_covr);

struct NavAbilityBlobStore *NavAbilityBlobStore_new(const struct NavAbilityClient *nvacl,
                                                    const char *label);

struct NavAbilityClient *NavAbilityClient_new(const char *api_url,
                                              const char *orgid,
                                              const char *api_token);

struct NavAbilityDFG *NavAbilityDFG_new(const struct NavAbilityClient *_nvacl,
                                        const char *fgLabel,
                                        const char *agentLabel,
                                        const char *storeLabel,
                                        const bool *addAgentIfAbsent,
                                        const bool *addGraphIfAbsent);

struct Pose3Pose3_FullNormal *Pose3Pose3(const struct FullNormal *Z);

const char *addAgentBlobEntry(const struct NavAbilityClient *_nvacl,
                              const char *agent_label,
                              const struct BlobEntry *_entry);

struct Option_____c_char addVariable(const struct NavAbilityDFG *nvafg,
                                     const char *label,
                                     const char *variableType,
                                     struct Option_Vec______c_char _tags,
                                     struct Option_i64 _solvable,
                                     struct Option______c_char _timestamp,
                                     struct Option_usize _nstime,
                                     struct Option______c_char _metadata);

void free_BlobEntry(struct BlobEntry*);

void free_FullNormal(struct FullNormal*);

void free_NavAbilityBlobStore(struct NavAbilityBlobStore*);

void free_NavAbilityClient(struct NavAbilityClient*);

void free_NavAbilityDFG(struct NavAbilityDFG*);

void free_Pose3Pose3(struct Pose3Pose3_FullNormal*);

void free_VariableDFG(struct VariableDFG*);

void free_cstr(char *pointer);

void free_rvecagent(struct RVec_Agent *rvec);

struct RVec_Agent *getAgents(const struct NavAbilityClient *_nvacl);

const char *getLabel_Agent(const struct Agent *agent);

const char *getLabel_BlobEntry(const struct BlobEntry *bentry);

const char *getLabel_NavAbilityBlobStore(const struct NavAbilityBlobStore *store);

const char *getLabel_NavAbilityClient(const struct Agent *input);

const char *getLabel_NavAbilityDFG(const struct NavAbilityDFG *input);

const char *getLabel_NvaNode(const struct NvaNode_T *input);

struct VariableDFG *getVariable(const struct NavAbilityDFG *nvafg, const char *label);

char *get_apiurl(const struct NavAbilityClient *nvacl);

struct Agent *get_index(const struct RVec_Agent *rv_agent, size_t index);

size_t length(const struct RVec_Agent *rv_agent);

const char *updateAgentMetadata(const struct NavAbilityClient *_nvacl,
                                const char *agent_label,
                                const char *metadata);


#define getLabel(obj)                                         \
    _Generic(obj,                                             \
        Agent*:                getLabel_Agent,                \
        BlobEntry*:            getLabel_BlobEntry,            \
        NavAbilityBlobStore*:  getLabel_NavAbilityBlobStore,  \
        NavAbilityDFG*:        getLabel_NavAbilityDFG         \
    ) (obj)
// https://stackoverflow.com/a/73458289 
// _Generic wont work since Rust type sizes likely unknown to C compiler
// https://thelinuxcode.com/function-overloading-c/
// http://www.robertgamble.net/2012/01/c11-generic-selections.html
// https://stackoverflow.com/a/76240760
// printf("[%s] @ line [%d]: \n", #obj, __LINE__);  









//