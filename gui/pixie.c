#include "raylib.h"

#include "rlgl.h"
#include "raymath.h"
#include "stdio.h"

#define GRID_WIDTH 50
#define GRID_HEIGHT 30
#define CELL_SIZE 50

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

    Texture2D resistorTexture = LoadTexture("resources/resistor.png");

    resistorTexture.height *= (100. / (float)resistorTexture.width);
    resistorTexture.width = 100;

    Camera2D camera = {0};
    camera.zoom = 1.0f;

    bool wiring = false;
    Vector2 wiringOrigin = { 0, 0 };

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

        DrawTexture(resistorTexture, 500, 500 - resistorTexture.height / 2, LIGHTGRAY);
        DrawText("R1", 500 - 10 + resistorTexture.width / 2, 500 - resistorTexture.height - 10, 20, LIGHTGRAY);

        for (int dx = 0; dx <= 1; dx++) {
            for (int dy = 0; dy <= 1; dy++) {
                if (gridx < GRID_WIDTH && gridx > 0 && gridy < GRID_HEIGHT && gridy > 0) {
                    Vector2 point = {(gridx + dx) * CELL_SIZE, (gridy + dy) * CELL_SIZE};

                    if (CheckCollisionPointCircle(mouseWorldPos, point, 10)) {
                        DrawCircle(point.x, point.y, 8, LIGHTGRAY);

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
            DrawLineBezier(wiringOrigin, mouseWorldPos, 3, LIGHTGRAY);
            DrawCircle(wiringOrigin.x, wiringOrigin.y, 8, LIGHTGRAY);
        }

        EndMode2D();

        DrawText("Pixie v0.1", 10, 10, 20, LIGHTGRAY);

        EndDrawing();
    }

    CloseWindow();
    return 0;
}