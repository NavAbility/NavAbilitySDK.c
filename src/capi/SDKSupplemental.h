
// manually define the prototypes for the FactorDFG functions we need

// // FIXME -- DONT USE YET
// #define GET_MACRO(_1, _2, _3, NAME, ...) NAME
// #define SArr(...) GET_MACRO(__VA_ARGS__, SArr3, SArr2, SArr1)(__VA_ARGS__)
// // #define SArr0() printf("%s","")
// #define SArr1(a) strcat(strcat("",a),";")
// #define SArr2(a, b) strcat(strcat(strcat(strcat("",a),";"),b),";")
// #define SArr3(a, b, c) strcat(strcat(strcat(strcat(strcat("",a),";"),b),";"),";")


// typedef struct FactorDFG_##FNCTYPE ##FNCTYPE;

#define GEN_ADD_FACTOR(FNCTYPE) \
    struct FactorDFG_##FNCTYPE *add_FactorDFG_##FNCTYPE( \
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




// Overloading via C macros using _Generic

// https://stackoverflow.com/a/73458289 
// _Generic wont work since Rust type sizes likely unknown to C compiler
// https://thelinuxcode.com/function-overloading-c/
// http://www.robertgamble.net/2012/01/c11-generic-selections.html
// https://stackoverflow.com/a/76240760
// printf("[%s] @ line [%d]: \n", #obj, __LINE__);  


#define length(obj)                                           \
    _Generic(obj,                                             \
        RVec_String*:               length_RVec_String,             \
        RVec_Agent*:                length_RVec_Agent,             \
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
        RVec_String*:               getIndex_RVec_String,     \
        RVec_Agent*:                getIndex_RVec_Agent,           \
        RVec_NvaNode_Factorgraph*:  getIndex_RVec_NvaNode_Factorgraph \
    ) (obj,i)


#define freeR(obj)                                            \
    _Generic(obj,                                             \
        char*:                    free_cstr,                  \
        Agent*:                   free_Agent,                 \
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