#include "hpx_v1.h"

#define MAX_VERTICES 256
#define MAX_FACES 512
#define MODEL_PATH "/appfiles/objviewer/cyl_01.obj"

typedef struct {
    float x, y, z;
} Vertex;

typedef struct {
    int v[3];
} Face;

typedef struct {
    float cam_x, cam_y, cam_z;
    float yaw, pitch;
    Vertex vertices[MAX_VERTICES];
    Face faces[MAX_FACES];
    int vertex_count;
    int face_count;
    int notice; /* 0: none, 1: loaded obj, 2: using default cube, 3: file error */
    uint32_t color;
    int proj_x[MAX_VERTICES];
    int proj_y[MAX_VERTICES];
    float proj_z[MAX_VERTICES];
    int visible[MAX_VERTICES];
    int initialized;
} ObjViewerState;

static float my_sin(float x) {
    while (x > 3.14159265f) x -= 6.28318531f;
    while (x < -3.14159265f) x += 6.28318531f;
    float x2 = x * x;
    float x3 = x2 * x;
    float x5 = x3 * x2;
    float x7 = x5 * x2;
    return x - (x3 / 6.0f) + (x5 / 120.0f) - (x7 / 5040.0f);
}

static float my_cos(float x) {
    return my_sin(x + 1.57079632f);
}

static float parse_float(const char *str, const char **endptr) {
    const char *p = str;
    while (*p == ' ' || *p == '\t') p++;
    float sign = 1.0f;
    if (*p == '-') { sign = -1.0f; p++; }
    else if (*p == '+') { p++; }

    float val = 0.0f;
    while (*p >= '0' && *p <= '9') {
        val = val * 10.0f + (float)(*p - '0');
        p++;
    }
    if (*p == '.') {
        p++;
        float div = 10.0f;
        while (*p >= '0' && *p <= '9') {
            val += (float)(*p - '0') / div;
            div *= 10.0f;
            p++;
        }
    }
    if (*p == 'e' || *p == 'E') {
        p++;
        float esign = 1.0f;
        if (*p == '-') { esign = -1.0f; p++; }
        else if (*p == '+') { p++; }
        int exp_val = 0;
        while (*p >= '0' && *p <= '9') {
            exp_val = exp_val * 10 + (*p - '0');
            p++;
        }
        for (int k = 0; k < exp_val; k++) {
            if (esign > 0.0f) val *= 10.0f;
            else val /= 10.0f;
        }
    }
    if (endptr) *endptr = p;
    return val * sign;
}

static int parse_int(const char *str, const char **endptr) {
    const char *p = str;
    while (*p == ' ' || *p == '\t') p++;
    int sign = 1;
    if (*p == '-') { sign = -1; p++; }
    else if (*p == '+') { p++; }

    int val = 0;
    while (*p >= '0' && *p <= '9') {
        val = val * 10 + (*p - '0');
        p++;
    }
    if (endptr) *endptr = p;
    return val * sign;
}

static void load_default_cube(ObjViewerState *state) {
    state->vertices[0] = (Vertex){-1.0f, -1.0f,  1.0f};
    state->vertices[1] = (Vertex){ 1.0f, -1.0f,  1.0f};
    state->vertices[2] = (Vertex){ 1.0f,  1.0f,  1.0f};
    state->vertices[3] = (Vertex){-1.0f,  1.0f,  1.0f};
    state->vertices[4] = (Vertex){-1.0f, -1.0f, -1.0f};
    state->vertices[5] = (Vertex){ 1.0f, -1.0f, -1.0f};
    state->vertices[6] = (Vertex){ 1.0f,  1.0f, -1.0f};
    state->vertices[7] = (Vertex){-1.0f,  1.0f, -1.0f};
    state->vertex_count = 8;

    state->faces[0] = (Face){{0, 1, 2}};
    state->faces[1] = (Face){{0, 2, 3}};
    state->faces[2] = (Face){{1, 5, 6}};
    state->faces[3] = (Face){{1, 6, 2}};
    state->faces[4] = (Face){{5, 4, 7}};
    state->faces[5] = (Face){{5, 7, 6}};
    state->faces[6] = (Face){{4, 0, 3}};
    state->faces[7] = (Face){{4, 3, 7}};
    state->faces[8] = (Face){{3, 2, 6}};
    state->faces[9] = (Face){{3, 6, 7}};
    state->faces[10] = (Face){{4, 5, 1}};
    state->faces[11] = (Face){{4, 1, 0}};
    state->face_count = 12;

    state->notice = 2;
}

static void parse_obj_data(const uint8_t *data, size_t len, ObjViewerState *state) {
    state->vertex_count = 0;
    state->face_count = 0;

    size_t i = 0;
    while (i < len && state->vertex_count < MAX_VERTICES && state->face_count < MAX_FACES) {
        while (i < len && (data[i] == ' ' || data[i] == '\r' || data[i] == '\t')) i++;
        if (i >= len) break;

        if (data[i] == 'v' && (i + 1 < len && (data[i+1] == ' ' || data[i+1] == '\t'))) {
            i += 2;
            char line_buf[128];
            size_t start = i;
            while (i < len && data[i] != '\n' && data[i] != '\r' && (i - start) < (sizeof(line_buf) - 1)) {
                line_buf[i - start] = (char)data[i];
                i++;
            }
            line_buf[i - start] = '\0';

            const char *p = line_buf;
            float vx = parse_float(p, &p);
            float vy = parse_float(p, &p);
            float vz = parse_float(p, &p);
            if (state->vertex_count < MAX_VERTICES) {
                state->vertices[state->vertex_count++] = (Vertex){vx, vy, vz};
            }
        } else if (data[i] == 'f' && (i + 1 < len && (data[i+1] == ' ' || data[i+1] == '\t'))) {
            i += 2;
            char line_buf[128];
            size_t start = i;
            while (i < len && data[i] != '\n' && data[i] != '\r' && (i - start) < (sizeof(line_buf) - 1)) {
                line_buf[i - start] = (char)data[i];
                i++;
            }
            line_buf[i - start] = '\0';

            const char *p = line_buf;
            int f1 = parse_int(p, &p);
            while (*p && *p != ' ' && *p != '\t') p++;
            int f2 = parse_int(p, &p);
            while (*p && *p != ' ' && *p != '\t') p++;
            int f3 = parse_int(p, &p);
            if (f1 != 0 && f2 != 0 && f3 != 0) {
                if (state->face_count < MAX_FACES) {
                    state->faces[state->face_count++] = (Face){{f1 - 1, f2 - 1, f3 - 1}};
                }
            }
        } else {
            while (i < len && data[i] != '\n') i++;
        }
        while (i < len && (data[i] == '\n' || data[i] == '\r')) i++;
    }

    if (state->vertex_count == 0 || state->face_count == 0) {
        load_default_cube(state);
    } else {
        state->notice = 1;
    }
}

static void load_obj_file(const HpxHostApi *host, ObjViewerState *state) {
    const uint8_t *path = (const uint8_t *)MODEL_PATH;
    int64_t length = host->read_file(path, sizeof(MODEL_PATH) - 1, 0, 0);

    // 1. Guard against missing file, error (< 0), or zero-length file (== 0)
    if (length <= 0) {
        load_default_cube(state);
        state->notice = 3;
        return;
    }

    // 2. Safely allocate buffer for non-zero length
    size_t alloc_size = (size_t)length;
    uint8_t *bytes = (uint8_t *)host->allocate(alloc_size, 8);
    if (!bytes) {
        load_default_cube(state);
        state->notice = 3;
        return;
    }

    // 3. Read file contents into allocated memory
    int64_t actual = host->read_file(path, sizeof(MODEL_PATH) - 1, bytes, alloc_size);
    if (actual != length) {
        // Only deallocate if allocation succeeded and size matches
        host->deallocate(bytes, alloc_size, 8);
        load_default_cube(state);
        state->notice = 3;
        return;
    }

    // 4. Parse OBJ data and free memory with exact same alloc_size
    parse_obj_data(bytes, alloc_size, state);
    host->deallocate(bytes, alloc_size, 8);
}

uint32_t hpx_step(const HpxHostApi *host, void *opaque) {
    (void)host;
    (void)opaque;
    return 1;
}

void hpx_draw(const HpxHostApi *host, void *opaque, size_t x, size_t y) {
    ObjViewerState *state = (ObjViewerState *)opaque;

    // Initialization check runs strictly ONCE on startup
    if (!state->initialized) {
        state->cam_x = 0.0f;
        state->cam_y = 0.0f;
        state->cam_z = -5.0f;
        state->yaw = 0.0f;
        state->pitch = 0.0f;
        state->color = 0x00FF88;
        state->notice = 0;

        load_obj_file(host, state);
        state->initialized = 1;
    }

    host->fill_rect(x, y, 400, 400, 0x181824);

    float s_y = my_sin(-state->yaw);
    float c_y = my_cos(-state->yaw);
    float s_p = my_sin(-state->pitch);
    float c_p = my_cos(-state->pitch);

    for (int i = 0; i < state->vertex_count; i++) {
        Vertex v = state->vertices[i];
        float dx = v.x - state->cam_x;
        float dy = v.y - state->cam_y;
        float dz = v.z - state->cam_z;

        float rx = dx * c_y + dz * s_y;
        float rz = -dx * s_y + dz * c_y;

        float ry = dy * c_p - rz * s_p;
        float final_z = dy * s_p + rz * c_p;

        state->proj_z[i] = final_z;
        if (final_z < 0.2f) {
            state->visible[i] = 0;
        } else {
            state->visible[i] = 1;
            float factor = 300.0f / final_z;
            state->proj_x[i] = (int)(rx * factor + 200.0f + (float)x);
            state->proj_y[i] = (int)(ry * factor + 200.0f + (float)y);
        }
    }

    uint32_t wire_color = state->color ? state->color : 0x00FF88;
    for (int i = 0; i < state->face_count; i++) {
        Face f = state->faces[i];
        int i1 = f.v[0];
        int i2 = f.v[1];
        int i3 = f.v[2];

        if (i1 >= 0 && i1 < state->vertex_count &&
            i2 >= 0 && i2 < state->vertex_count &&
            i3 >= 0 && i3 < state->vertex_count) {
            if (state->visible[i1] && state->visible[i2] && state->visible[i3]) {
                host->draw_line((size_t)state->proj_x[i1], (size_t)state->proj_y[i1], (size_t)state->proj_x[i2], (size_t)state->proj_y[i2], wire_color);
                host->draw_line((size_t)state->proj_x[i2], (size_t)state->proj_y[i2], (size_t)state->proj_x[i3], (size_t)state->proj_y[i3], wire_color);
                host->draw_line((size_t)state->proj_x[i3], (size_t)state->proj_y[i3], (size_t)state->proj_x[i1], (size_t)state->proj_y[i1], wire_color);
            }
        }
    }

    host->fill_rect(x, y + 340, 400, 60, 0x101018);
    static const uint8_t title[] = "3D .obj Viewer";
    host->draw_text(x + 10, y + 345, title, sizeof(title) - 1, 0xFFFFFF);

    static const uint8_t controls[] = "WASD:Move IJKL:Look Space/X:Down/Up O:Load";
    host->draw_text(x + 10, y + 365, controls, sizeof(controls) - 1, 0xA0A0A0);

    if (state->notice == 1) {
        static const uint8_t msg[] = "Loaded \\model.obj successfully";
        host->draw_text(x + 10, y + 382, msg, sizeof(msg) - 1, 0x80FF80);
    } else if (state->notice == 2) {
        static const uint8_t msg[] = "Using Default Cube Model";
        host->draw_text(x + 10, y + 382, msg, sizeof(msg) - 1, 0x80D0FF);
    } else if (state->notice == 3) {
        static const uint8_t msg[] = "File \\model.obj missing (Cube fallback)";
        host->draw_text(x + 10, y + 382, msg, sizeof(msg) - 1, 0xFF8080);
    }
}

void hpx_input(const HpxHostApi *host, void *opaque, uint32_t packed_key) {
    ObjViewerState *state = (ObjViewerState *)opaque;
    if (packed_key & 0x10000) return;

    float move_speed = 0.5f;
    float rot_speed = 0.1f;

    switch (packed_key & 0xFFFF) {
        case 'w':
        case 'W':
            state->cam_x += my_sin(state->yaw) * move_speed;
            state->cam_z += my_cos(state->yaw) * move_speed;
            break;
        case 's':
        case 'S':
            state->cam_x -= my_sin(state->yaw) * move_speed;
            state->cam_z -= my_cos(state->yaw) * move_speed;
            break;
        case 'a':
        case 'A':
            state->cam_x -= my_cos(state->yaw) * move_speed;
            state->cam_z += my_sin(state->yaw) * move_speed;
            break;
        case 'd':
        case 'D':
            state->cam_x += my_cos(state->yaw) * move_speed;
            state->cam_z -= my_cos(state->yaw) * move_speed;
            break;
        case 'j':
        case 'J':
            state->yaw -= rot_speed;
            break;
        case 'l':
        case 'L':
            state->yaw += rot_speed;
            break;
        case 'i':
        case 'I':
            state->pitch -= rot_speed;
            break;
        case 'k':
        case 'K':
            state->pitch += rot_speed;
            break;
        case ' ':
            state->cam_y -= 0.5f;
            break;
        case 'x':
        case 'X':
            state->cam_y += 0.5f;
            break;
        case 'o':
        case 'O':
            load_obj_file(host, state);
            break;
        case 'c':
        case 'C':
            state->color = state->color == 0x00FFFF ? 0x00FF88 : 0x00FFFF;
            break;
        default:
            break;
    }
}