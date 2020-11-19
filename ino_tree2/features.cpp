#include "features.h"

short pixel_total(short* buffer) {
    short sum = 0;
    for (unsigned char i = 0; i < 9; ++i) {
        sum += buffer[i];
    }
    return sum;
}

// We need to take a different approach compared to
// lib_feature implementation, because we are very limited in RAM
// Hence we will calculate it in place
void center_of_gravity_distribution_x(short frame_buffer[80][9], int num_recorded_frames, float* arg_buffer) {
    // We collect a minimum of 6 frames per gesture, hence this will be >0
    float merge_amount = ((float)num_recorded_frames) / 6.0;
    float j = 1.0;
    int index = 0;
    for (char i = 0; i < 6; ++i) {
        arg_buffer[i] = 0.0;
        for (; j <= (merge_amount * ((float)(i + 1))); j += 1.0) {
            short* current_buffer = frame_buffer[index];
            short total = pixel_total(current_buffer);
            if (total > 0)
                arg_buffer[i] += (float)(current_buffer[0] + current_buffer[3] + current_buffer[6] - current_buffer[2] - current_buffer[5] - current_buffer[8]) / ((float)total);
            ++index;
        }
        arg_buffer[i] = arg_buffer[i] / merge_amount;
    }
}

void center_of_gravity_distribution_y(short frame_buffer[80][9], int num_recorded_frames, float* arg_buffer) {
    float merge_amount = ((float)num_recorded_frames) / 6.0;
    float j = 1.0;
    int index = 0;
    for (char i = 0; i < 6; ++i) {
        arg_buffer[i] = 0.0;
        for (; j <= (merge_amount * ((float)(i + 1))); j += 1.0) {
            short* current_buffer = frame_buffer[index];
            short total = pixel_total(current_buffer);
            if (total > 0)
                arg_buffer[i] += (float)(current_buffer[0] + current_buffer[1] + current_buffer[2] - current_buffer[6] - current_buffer[7] - current_buffer[8]) / ((float)total);
            ++index;
        }
        arg_buffer[i] = arg_buffer[i] / merge_amount;
    }
}
