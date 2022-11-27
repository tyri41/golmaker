#version 450

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer DataIn {
    uint data[];
} dataIn;

layout(set = 0, binding = 1) buffer DataOut {
    uint data[];
} dataOut;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    bool reverse = dataOut.data[idx] > dataIn.data[idx];

    if (reverse) {
        dataIn.data[idx] = dataOut.data[idx] + 1;
    } else {
        dataOut.data[idx] = dataIn.data[idx] + 1;
    }

}