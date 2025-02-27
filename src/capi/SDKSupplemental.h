
// manually define the prototypes for the FactorDFG functions we need

// // FIXME -- DONT USE YET
// #define GET_MACRO(_1, _2, _3, NAME, ...) NAME
// #define SArr(...) GET_MACRO(__VA_ARGS__, SArr3, SArr2, SArr1)(__VA_ARGS__)
// // #define SArr0() printf("%s","")
// #define SArr1(a) strcat(strcat("",a),";")
// #define SArr2(a, b) strcat(strcat(strcat(strcat("",a),";"),b),";")
// #define SArr3(a, b, c) strcat(strcat(strcat(strcat(strcat("",a),";"),b),";"),";")


struct FactorDFG_PriorPoint2_FullNormal *FactorDFG_PriorPoint2_FullNormal_new(
    const char *const *varlbls,
    size_t varlbls_len,
    const struct PriorPoint2_FullNormal *fnc
);


struct FactorDFG_PriorPoint3_FullNormal *FactorDFG_PriorPoint3_FullNormal_new(
    const char *const *varlbls,
    size_t varlbls_len,
    const struct PriorPoint3_FullNormal *fnc
);


struct FactorDFG_PriorPose2_FullNormal *FactorDFG_PriorPose2_FullNormal_new(
    const char *const *varlbls,
    size_t varlbls_len,
    const struct PriorPose2_FullNormal *fnc
);


struct FactorDFG_PriorPose3_FullNormal *FactorDFG_PriorPose3_FullNormal_new(
    const char *const *varlbls,
    size_t varlbls_len,
    const struct PriorPose3_FullNormal *fnc
);


struct FactorDFG_Point2Point2_FullNormal *FactorDFG_Point2Point2_FullNormal_new(
    const char *const *varlbls,
    size_t varlbls_len,
    const struct Point2Point2_FullNormal *fnc
);

struct FactorDFG_Point3Point3_FullNormal *FactorDFG_Point3Point3_FullNormal_new(
    const char *const *varlbls,
    size_t varlbls_len,
    const struct Point3Point3_FullNormal *fnc
);

struct FactorDFG_Pose2Pose2_FullNormal *FactorDFG_Pose2Pose2_FullNormal_new(
    const char *const *varlbls,
    size_t varlbls_len,
    const struct Pose2Pose2_FullNormal *fnc
);


struct FactorDFG_Pose3Pose3_FullNormal *FactorDFG_Pose3Pose3_FullNormal_new(
    const char *const *varlbls,
    size_t varlbls_len,
    const struct Pose3Pose3_FullNormal *fnc
);





// Overloading via C macros using _Generic

// https://stackoverflow.com/a/73458289 
// _Generic wont work since Rust type sizes likely unknown to C compiler
// https://thelinuxcode.com/function-overloading-c/
// http://www.robertgamble.net/2012/01/c11-generic-selections.html
// https://stackoverflow.com/a/76240760
// printf("[%s] @ line [%d]: \n", #obj, __LINE__);  


#define getLabel(obj)                                         \
    _Generic(obj,                                             \
        Agent*:                getLabel_Agent,                \
        BlobEntry*:            getLabel_BlobEntry,            \
        NavAbilityBlobStore*:  getLabel_NavAbilityBlobStore,  \
        NavAbilityDFG*:        getLabel_NavAbilityDFG        \
    ) (obj)

    
#define getIndex(obj,i)                                       \
    _Generic(obj,                                             \
        RVec_Agent*:                getIndex_Agent           \
    ) (obj,i)


#define freeR(obj)                                            \
    _Generic(obj,                                             \
        char*:                    free_cstr,                  \
        Agent*:                   free_Agent,                 \
        RVec_Agent*:              free_RVec_Agent,            \
        BlobEntry*:               free_BlobEntry,             \
        NavAbilityClient*:        free_NavAbilityClient,      \
        NavAbilityBlobStore*:     free_NavAbilityBlobStore,   \
        NavAbilityDFG*:           free_NavAbilityDFG,         \
        VariableDFG*:             free_VariableDFG,           \
        FullNormal*:              free_FullNormal,            \
        PriorPoint2_FullNormal*:   free_PriorPoint2,          \
        PriorPoint3_FullNormal*:   free_PriorPoint3,          \
        PriorPose2_FullNormal*:   free_PriorPose2,            \
        PriorPose3_FullNormal*:   free_PriorPose3,            \
        Point2Point2_FullNormal*:   free_Point2Point2,        \
        Point3Point3_FullNormal*:   free_Point3Point3,        \
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


#define addFactor(nfg,vl,obj,tags)                                                   \
    _Generic(obj,                                                             \
        PriorPoint2_FullNormal*:    FactorDFG_PriorPoint2_FullNormal_new,        \
        PriorPoint3_FullNormal*:    FactorDFG_PriorPoint3_FullNormal_new,        \
        PriorPose2_FullNormal*:    FactorDFG_PriorPose2_FullNormal_new,        \
        PriorPose3_FullNormal*:    FactorDFG_PriorPose3_FullNormal_new,        \
        Point2Point2_FullNormal*:    FactorDFG_Point2Point2_FullNormal_new,        \
        Point3Point3_FullNormal*:    FactorDFG_Point3Point3_FullNormal_new,        \
        Pose2Pose2_FullNormal*:    FactorDFG_Pose2Pose2_FullNormal_new,        \
        Pose3Pose3_FullNormal*:    FactorDFG_Pose3Pose3_FullNormal_new        \
    ) (nfg,vl,obj,tags)


//