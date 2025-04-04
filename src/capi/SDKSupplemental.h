
// manually define the prototypes for the FactorDFG functions we need

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

// struct FactorDFG_PriorPoint2_FullNormal *add_FactorDFG_PriorPoint2_FullNormal(
//     const char *varlbls,
//     const struct PriorPoint2_FullNormal *fnc
// );


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
        NavAbilityDFG*:           free_NavAbilityDFG,         \
        VariableDFG*:             free_VariableDFG,           \
        FullNormal*:              free_FullNormal,            \
        PriorPoint2_FullNormal*:  free_PriorPoint2,          \
        PriorPoint3_FullNormal*:  free_PriorPoint3,          \
        PriorPose2_FullNormal*:   free_PriorPose2,            \
        PriorPose3_FullNormal*:   free_PriorPose3,            \
        Point2Point2_FullNormal*: free_Point2Point2,        \
        Point3Point3_FullNormal*: free_Point3Point3,        \
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
