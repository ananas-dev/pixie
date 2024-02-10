#include <stdlib.h>
#include "raylib.h"
#include "rlgl.h"
#include "raymath.h"
#include "stdio.h"
#include "components.h"

#define GRID_WIDTH 50
#define GRID_HEIGHT 30
#define CELL_SIZE 50
#define NUM_COMPONENTS 4

// Dynamic array implementation

typedef struct {
    Component_t* array;
    size_t used;
    size_t size;
} Array;

void insertArray(Array *a, Component_t component) {
  // a->used is the number of used entries, because a->array[a->used++] updates a->used only *after* the array has been accessed.
  // Therefore a->used can go up to a->size 
  if (a->used == a->size) {
    a->size *= 2;
    a->array = realloc(a->array, a->size * sizeof(int));
  }
  a->array[a->used++] = component;
}

void removeArray(Array *a, int index) {
    int sizeOfArray = a->size;
    int* temp = malloc((sizeOfArray - 1) * sizeof(int)); // allocate an array with a size 1 less than the current one

    if (index != 0)
        memcpy(temp, a, index * sizeof(int)); // copy everything BEFORE the index

    if (index != (sizeOfArray - 1))
        memcpy(temp+index, a+index+1, (sizeOfArray - index - 1) * sizeof(int)); // copy everything AFTER the index
          
    free(a);
    return temp;
}

void freeArray(Array *a) {
  free(a->array);
  a->array = NULL;
  a->used = a->size = 0;
}



int resistorCount = 0;
int currentSourceCount = 0;
int voltageSourceCount = 0;
int diodeCount = 0;

Component_t components[];

void drawResistor(Texture2D resistorTexture, int cellX, int cellY){
    DrawTexture(resistorTexture, cellX*CELL_SIZE, cellY*CELL_SIZE - resistorTexture.height/2, WHITE);
    char* text = (char*) malloc(4); // R + 2 digits + '\0' 
    sprintf(text, "R%d", resistorCount);
    DrawText(text, cellX*CELL_SIZE + resistorTexture.width / 2 - 10, cellY*CELL_SIZE - resistorTexture.height + 10, 25, WHITE);
}


int main()
{
    SetConfigFlags(FLAG_WINDOW_RESIZABLE | FLAG_VSYNC_HINT);

    // CheckCollisionCircles() nodes[GRID_HEIGHT][GRID_WIDTH];

    // for (int i = 0; i < GRID_WIDTH; i++) {
    //     for (int j = 0; j < GRID_HEIGHT; j++) {
    //         nodes
    //     }
    // }

    const int screenWidth = 800;
    const int screenHeight = 450;

    InitWindow(screenWidth, screenHeight, "Pixie");

    Texture2D resistorTexture = LoadTexture("resources/Resistor.png");

    float aspectRatio = resistorTexture.width / resistorTexture.height;
    resistorTexture.width = 3*CELL_SIZE;
    resistorTexture.height = resistorTexture.width / aspectRatio;
    

    Camera2D camera = {0};
    camera.zoom = 1.0f;

    bool wiring = false;
    Vector2 wiringOrigin = { 0, 0 };

    Rectangle toggleComponents[NUM_COMPONENTS]; // Array of rectangles each representing a component that you can select
    int mouseHoverComponents = -1; // Index of the component currently hovered (-1 if the mouse is not on a component)
    componentType selectedComponent = NONE; // Type of the selected component, NONE if no component is selected (see components.h for definition)

    float rectangleWidth = 250;
    float rectangleHeight = 100;
    for (int i = 0; i < NUM_COMPONENTS; i++) toggleComponents[i] = (Rectangle){ 10, (float)(2*rectangleHeight + rectangleHeight*i), rectangleWidth, rectangleHeight };

    SetTargetFPS(60);

    while (!WindowShouldClose()) {
        Vector2 mouseWorldPos = GetScreenToWorld2D(GetMousePosition(), camera);

        if (IsMouseButtonDown(MOUSE_BUTTON_LEFT) && IsKeyDown(KEY_LEFT_CONTROL)) {
            Vector2 delta = GetMouseDelta();
            delta = Vector2Scale(delta, -1.0f / camera.zoom);

            camera.target = Vector2Add(camera.target, delta);
        }

        float wheel = GetMouseWheelMove();
        if (wheel != 0) {
            camera.offset = GetMousePosition();

            camera.target = mouseWorldPos;

            const float zoomIncrement = 1.1;

            if (wheel > 0) {
                camera.zoom *= zoomIncrement;
            } else {
                camera.zoom *= 1. / zoomIncrement;
            }

            if (camera.zoom < 0.18) {
                camera.zoom = 0.18;
            } else if (camera.zoom > 7) {
                camera.zoom = 7;
            }
        }


        // Mouse Toggle logic for component selection
        for(int i = 0; i < NUM_COMPONENTS; i++){
            if(CheckCollisionPointRec(GetMousePosition(), toggleComponents[i])){ // If the mouse is on a component
                mouseHoverComponents = i; // Then updates which component the mouse is hovering

                if(IsMouseButtonReleased(MOUSE_BUTTON_LEFT)){
                    selectedComponent = i; // Updates currently selected component if you click on it
                }
                break;
            }
            else{
                mouseHoverComponents = -1;
            }
        }

        if(mouseHoverComponents != -1) printf("%s\n", componentsName[mouseHoverComponents]);

        float gridx = floorf(mouseWorldPos.x / CELL_SIZE);
        float gridy = floorf(mouseWorldPos.y / CELL_SIZE);

        BeginDrawing();
        ClearBackground(BLACK);

        BeginMode2D(camera);
        Color gridColor = {20, 20, 20, 255};
        Color borderColor = {45, 45, 45, 255};

        float borderSize = 5. / camera.zoom;

        for (int i = 0; i < 50 + 1; i++) {
            DrawLineV((Vector2){50 * i, 0}, (Vector2){50 * i, 50 * 30}, gridColor);
        }

        for (int i = 0; i < 30 + 1; i++) {
            DrawLineV((Vector2){0, 50 * i}, (Vector2){50 * 50, 50 * i}, gridColor);
        }

        DrawRectangleV((Vector2){-borderSize, -borderSize}, (Vector2){borderSize, 50 * 30 + 2 * borderSize}, borderColor);
        DrawRectangleV((Vector2){50 * 50, -borderSize}, (Vector2){borderSize, 50 * 30 + 2 * borderSize}, borderColor);
        DrawRectangleV((Vector2){0, -borderSize}, (Vector2){50 * 50, borderSize}, borderColor);
        DrawRectangleV((Vector2){0, 50 * 30}, (Vector2){50 * 50, borderSize}, borderColor);

        //DrawTexture(resistorTexture, 0, 500 - resistorTexture.height / 2, WHITE);
        //DrawText("R1", 500 - 10 + resistorTexture.width / 2, 500 - resistorTexture.height - 10, 20, WHITE);
        drawResistor(resistorTexture, 5, 10);
        drawResistor(resistorTexture, 12, 0);
        drawResistor(resistorTexture, 0, 12);

        for (int dx = 0; dx <= 1; dx++) {
            for (int dy = 0; dy <= 1; dy++) {
                if (gridx < GRID_WIDTH && gridx > 0 && gridy < GRID_HEIGHT && gridy > 0) {
                    Vector2 point = {(gridx + dx) * CELL_SIZE, (gridy + dy) * CELL_SIZE};

                    if (CheckCollisionPointCircle(mouseWorldPos, point, 10)) {
                        DrawCircle(point.x, point.y, 8, WHITE);

                        if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
                            wiring = true;
                            wiringOrigin = point;
                        }

                        break;
                    }
                }
            }
        }

        if (IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) {
            wiring = false;
        }

        if (wiring) {
            DrawLine(wiringOrigin.x, wiringOrigin.y, (mouseWorldPos.x + wiringOrigin.x)/2, wiringOrigin.y, WHITE);
            DrawLine((mouseWorldPos.x + wiringOrigin.x)/2, wiringOrigin.y, (mouseWorldPos.x + wiringOrigin.x)/2, mouseWorldPos.y, WHITE);
            DrawLine((mouseWorldPos.x + wiringOrigin.x)/2, mouseWorldPos.y, mouseWorldPos.x, mouseWorldPos.y, WHITE);
            DrawCircle(wiringOrigin.x, wiringOrigin.y, 8, WHITE);
        }

        EndMode2D();

        DrawText("Pixie v0.1", 10, 10, 20, WHITE);

        // Component selection menu 
        DrawText("Select a component", 10, 50, 20, WHITE);
        
        for (int i = 0; i < NUM_COMPONENTS; i++)
        {
            bool selectedOrHovered = ((i == selectedComponent) || (i == mouseHoverComponents));

            DrawRectangleRec(toggleComponents[i], selectedOrHovered ? RED : DARKGRAY);
            DrawRectangleLines((int) toggleComponents[i].x, (int) toggleComponents[i].y, (int) toggleComponents[i].width, (int) toggleComponents[i].height, selectedOrHovered ? BLACK : GRAY);
            DrawText(componentsName[i], (int)(toggleComponents[i].x + toggleComponents[i].width/2 - MeasureText(componentsName[i], rectangleHeight/4)/2), (int) toggleComponents[i].y + toggleComponents[i].height/2 - 10, rectangleHeight / 4, selectedOrHovered ? BLACK : WHITE);
        }
        

        EndDrawing();
    }

    CloseWindow();
    return 0;
}