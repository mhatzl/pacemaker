
typedef struct PulseParam {
    float amplitude;
    float width;
};

typedef struct Param {
    struct PulseParam atrial;
    struct PulseParam ventricular;
    unsigned int lrl;
    unsigned int vrp
};