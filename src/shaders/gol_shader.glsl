#version 450

layout(local_size_x = 16, local_size_y = 16, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer DataIn {
    uint data[];
} dataIn;

layout(set = 0, binding = 1) buffer DataOut {
    uint data[];
} dataOut;


layout(push_constant) uniform Params {
    uint w;
    uint h;
} params;

uint get(int i, int j) {
    uint ii = (i + int(params.w)) % params.w;
    uint jj = (j + int(params.h)) % params.h;
    return dataIn.data[ii * params.w + jj];
}

void main() {
    int x = int(gl_GlobalInvocationID.x);
    int y = int(gl_GlobalInvocationID.y);

    if (x >= params.h || y >= params.w) {
        return;
    }

    uint sum =
        get(x + 1, y + 1) + get(x + 1, y) + get(x + 1, y - 1) + 
        get(x,     y + 1)                 + get(x,     y - 1) +
        get(x - 1, y + 1) + get(x - 1, y) + get(x - 1, y - 1);

    uint survived = get(x, y) & uint(sum == 2);
    uint revived = uint(sum == 3);

    dataOut.data[x * params.w + y] = survived | revived;
}