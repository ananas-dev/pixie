typedef enum { 
    NONE = -1,
    WIRE,
    RESISTOR,
    CURRENTSOURCE,
    VOLTAGESOURCE,
    DIODE,

    // Place new Components above // 
    NUM_COMPONENTS // Evaluates to the number of components (NONE is excluded)
} componentType;

static const char* componentsName[] = {
    "Wire",
    "Resistor",
    "Current Source",
    "Voltage Source",
    "Diode",
};

typedef struct Wire {
    Vector2 origin; // Origin node
    Vector2 destination; // Destination node
} Wire_t;

typedef struct Resistor {
    int id; // Identificator of the resistor
    Vector2 a; // First pin
    Vector2 b; // Second pin
    float r; // Resistance value
} Resistor_t;

typedef struct CurrentSource {
    int id;
    Vector2 p; // Positive pin
    Vector2 n; // N*gative pin
    float i; // Current value
} CurrentSource_t;

typedef struct VoltageSource {
    int id;
    Vector2 p; // Positive pin
    Vector2 n; // N*gative pin
    float v; // Voltage value
} VoltageSource_t;

typedef struct Diode {
    int id;
    Vector2 p; // Positive pin
    Vector2 n; // N*gative pin
    float is; // Current saturation
    float temp; // Diode temperature
} Diode_t;


// Struct to group all components together
typedef struct Component {
    componentType type;
    union {
        Wire_t wire;
        Resistor_t resistor;
        CurrentSource_t current;
        VoltageSource_t voltage;
        Diode_t diode;
    } as;
}Component_t;



/******************************

 Dynamic Component_t array implementation

******************************/

typedef struct {
    Component_t* ptr;
    int used;
    int size;
} ComponentArray;



/**
 * @brief Initialize a  ComponentArray with an initial size
 * 
 * @param array Pointer to a ComponentArray 
 * @param init_size initial size of the array
 * @return 0 if successful, 1 otherwise
 */
int init_array(ComponentArray* array, int init_size) {
    Component_t* a = (Component_t*) malloc(init_size*sizeof(Component_t));

    if (a == NULL) {
        printf("Error occured while allocating memory in 'init_array'\n");
        return 1;
    }

    array->ptr = a;
    array->used = 0;
    array->size = init_size;
    return 0;
}



/**
 * @brief Insert a component at the end of the ComponentArray. Reallocates memory if needed.
 * In this case, reallocates the current size of the array on top of the memory already allocated.
 * 
 * @param array Pointer to a ComponentArray 
 * @param component Component to add at the end of array
 * @return 0 if the operation is successful, 1 otherwise
 */
int insert_array(ComponentArray* array, Component_t component) {

    // Reallocates memory if needed
    if (array->used == array->size) {
        array->ptr = (Component_t*) realloc(array->ptr, 2 * array->size * sizeof(Component_t));

        if (array->ptr == NULL) {
            printf("Error occurred while reallocating memory in 'insert_array'\n");
            return 1;
        }

        array->ptr[array->used] = component;
        array->used++;
        array->size *= 2; 
    }
    else {
        array->ptr[array->used] = component;
        array->used++;
    }

    return 0;
}

/**
 * @brief Remove the component at the specified index 
 * 
 * @param array Pointer to a ComponentArray  
 * @param index The index of the component we want to remove (Could be n*gative, in which case the indexing start from the end)
 * @return 0 if the operation is successful, 1 otherwise
 */
int remove_array(ComponentArray* array, int index) {

    if(array->used == 0) {
        printf("ComponentArray is empty : 'remove_array' ignored\n");
        return 0;
    }

    if(index >= array->used) {
        printf("Out of bound index : You tried to remove component with index greater than the last element of the array\n");
        return 1;
    }

    // Handle n*gative index
    if(index < 0) {
        index = array->used + index;
    }

    Component_t* newPtr = (Component_t*) malloc((array->size-1) * sizeof(Component_t));

    if(newPtr == NULL) {
        printf("Error allocating memory\n");
        return 1;
    }

    // Copy every element except for the one at index 'index'
    for(int i = 0; i < array->used; i++) {
        if(i == index) {
            continue;
        }
        newPtr[i < index ? i : i - 1] = array->ptr[i];
    }

    free(array->ptr);
    array->ptr = newPtr;
    array->used--; 

    return 0;
}


/**
 * @brief Clear the ComponentArray
 * 
 * @param array The ComponentArray to be cleared
 */
void clear_array(ComponentArray* array){
    memset(array->ptr, NULL, array->size * sizeof(Component_t));
    array->used = 0;
}