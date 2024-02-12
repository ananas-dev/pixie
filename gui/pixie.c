#include <stdlib.h>
#include <string.h>
#include "raylib.h"
#include "rlgl.h"
#include "raymath.h"
#include "stdio.h"
#include "components.h"

#define GRID_WIDTH 50
#define GRID_HEIGHT 30
#define CELL_SIZE 50

int resistorCount = 0;
int currentSourceCount = 0;
int voltageSourceCount = 0;
int diodeCount = 0;

Component_t components[10];

/****

Component drawing functions

****/
Texture2D resistorTexture;
Texture2D currentSourceTexture;
Texture2D voltageSourceTexture;
Texture2D diodeTexture;

void drawResistor(Resistor_t resistor, Texture2D resistorTexture){
    int cellX = resistor.a.x;
    int cellY = resistor.a.y;
    DrawTexture(resistorTexture, cellX*CELL_SIZE, cellY*CELL_SIZE - resistorTexture.height/2, WHITE);
    char name[4]; // R + 2 digits + '\0' 
    char value[10]; // Resistor value with scientific notation
    sprintf(name, "R%d", resistor.id);
    sprintf(value, "%.3e", resistor.r);
    DrawText(name, cellX*CELL_SIZE + resistorTexture.width / 2 - MeasureText(name, 25)/2, cellY*CELL_SIZE - resistorTexture.height + 25, 25, WHITE);
    DrawText(value, cellX*CELL_SIZE + resistorTexture.width / 2 - MeasureText(value, 25)/2, cellY*CELL_SIZE + 25, 25, WHITE);
}

void drawCurrentSource(CurrentSource_t currentSource, Texture2D currentSourceTexture){
    int cellX = currentSource.p.x;
    int cellY = currentSource.p.y;
    DrawTexture(currentSourceTexture, cellX*CELL_SIZE, cellY*CELL_SIZE - currentSourceTexture.height/2, WHITE);
    char name[4]; // R + id
    char value[10]; // Current value with scientific notation
    sprintf(name, "I%d", currentSource.id);
    sprintf(value, "%.3e", currentSource.i);
    DrawText(name, cellX*CELL_SIZE + currentSourceTexture.width / 2 - MeasureText(name, 25)/2, cellY*CELL_SIZE - currentSourceTexture.height + 25, 25, WHITE);
    DrawText(value, cellX*CELL_SIZE + currentSourceTexture.width / 2 - MeasureText(value, 25)/2, cellY*CELL_SIZE + 25, 25, WHITE);
}

void drawVoltageSource(VoltageSource_t voltageSource, Texture2D voltageSourceTexture){
    int cellX = voltageSource.p.x;
    int cellY = voltageSource.p.y;
    DrawTexture(voltageSourceTexture, cellX*CELL_SIZE, cellY*CELL_SIZE - voltageSourceTexture.height/2, WHITE);
    char name[4]; // R + id
    char value[10]; // Voltage value with scientific notation
    sprintf(name, "V%d", voltageSource.id);
    sprintf(value, "%.3e", voltageSource.v);
    DrawText(name, cellX*CELL_SIZE + voltageSourceTexture.width / 2 - MeasureText(name, 25)/2, cellY*CELL_SIZE - voltageSourceTexture.height + 25, 25, WHITE);
    DrawText(value, cellX*CELL_SIZE + voltageSourceTexture.width / 2 - MeasureText(value, 25)/2, cellY*CELL_SIZE + 25, 25, WHITE);
}

void drawDiode(Diode_t diode, Texture2D diodeTexture){
    int cellX = diode.p.x;
    int cellY = diode.p.y;
    DrawTexture(diodeTexture, cellX*CELL_SIZE, cellY*CELL_SIZE - diodeTexture.height/2, WHITE);
    char name[4]; // R + id
    char value[10]; // Voltage value with scientific notation
    sprintf(name, "V%d", diode.id);
    sprintf(value, "%.3e", diode.is);
    DrawText(name, cellX*CELL_SIZE + diodeTexture.width / 2 - MeasureText(name, 25)/2, cellY*CELL_SIZE - diodeTexture.height + 25, 25, WHITE);
    DrawText(value, cellX*CELL_SIZE + diodeTexture.width / 2 - MeasureText(value, 25)/2, cellY*CELL_SIZE + 25, 25, WHITE);
}


void drawComponent(Component_t component) {
    componentType type = component.type;

    switch (type) {
        case RESISTOR : drawResistor(component.as.resistor, resistorTexture); break;
        case CURRENTSOURCE : drawCurrentSource(component.as.current, currentSourceTexture); break;
        case VOLTAGESOURCE : drawVoltageSource(component.as.voltage, voltageSourceTexture); break;
        case DIODE : drawDiode(component.as.diode, diodeTexture);
    }
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

    // Loads and resize components texture
    resistorTexture = LoadTexture("resources/Resistor.png");
    float aspectRatio = resistorTexture.width / resistorTexture.height;
    resistorTexture.width = 3*CELL_SIZE;
    resistorTexture.height = resistorTexture.width / aspectRatio;

    currentSourceTexture = LoadTexture("resources/Current_Source.png");
    aspectRatio = currentSourceTexture.width / currentSourceTexture.height;
    currentSourceTexture.width = 3*CELL_SIZE;
    currentSourceTexture.height = currentSourceTexture.width / aspectRatio;

    voltageSourceTexture = LoadTexture("resources/DC_Voltage.png");
    aspectRatio = voltageSourceTexture.width / voltageSourceTexture.height;
    voltageSourceTexture.width = 3*CELL_SIZE;
    voltageSourceTexture.height = voltageSourceTexture.width / aspectRatio;

    diodeTexture = LoadTexture("resources/Diode.png");
    aspectRatio = diodeTexture.width / diodeTexture.height;
    diodeTexture.width = 3*CELL_SIZE;
    diodeTexture.height = diodeTexture.width / aspectRatio;


    Resistor_t r1 = {0, {10, 10}, {13, 10}, 330};
    Component_t r1_comp;
    r1_comp.type = RESISTOR;
    r1_comp.as.resistor = r1;

    Diode_t d1 = {0, {12, 15}, {15, 15}, 0.7};
    Component_t d1_comp;
    d1_comp.type = DIODE;
    d1_comp.as.diode = d1;

    components[0] = r1_comp;
    components[1] = d1_comp;

    Camera2D camera = {0};
    camera.zoom = 1.0f;

    bool wiring = false;
    Vector2 wiringOrigin = { 0, 0 };

    Rectangle toggleComponents[NUM_COMPONENTS]; // Array of rectangles each representing a component that you can select
    int mouseHoverComponents = -1; // Index of the component currently hovered (-1 if the mouse is not on a component)
    componentType selectedComponent = NONE; // Type of the selected component, NONE if no component is selected (see components.h for definition)

    const float rectangleWidth = 250;
    const float rectangleHeight = 100;
    for (int i = 0; i < NUM_COMPONENTS; i++) toggleComponents[i] = (Rectangle){ 10, (float)(100 + rectangleHeight*i), rectangleWidth, rectangleHeight };

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


        // Draw components
        drawComponent(components[0]);
        drawComponent(components[1]);
        

        EndMode2D();

        DrawText("Pixie v0.1", 10, 10, 20, WHITE);

        // Component selection menu 
        DrawText("Select a component", 40, 50, 20, WHITE);
        
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