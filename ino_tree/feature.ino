#define RING_BUFFER_SIZE 80

int pixel_total(int** buffer) {
    int sum = 0;
    for (unsigned char i = 0; i < 3; ++i) {
        for (unsigned char j = 0; j < 3; ++j) {
            sum += buffer[i][j];
        }
    }
    return sum;
}

// We need to take a different approach compared to
// lib_feature implementation, because we are very limited in RAM
// Hence we will calculate it in place
void center_of_gravity_distribution_x(float* buffer) {
    // We collect a minimum of 6 frames per gesture, hence this will be >0
    unsigned char merge_amount = get_num_recorded_frames() / 6;
    unsigned char gesture_index = calculate_pad_index(0);
    for (unsigned char i = 0; i < 6; ++i) {
        buffer[i] = 0.0;
        int **current_frame = get_gesture(gesture_index);
        // TODO: Last few frames may not be processed due to rounding!
        for (unsigned char j = 0; j < merge_amount; ++j) {
            int total = pixel_total(current_frame);
            if (total > 0)
                buffer[i] += (current_frame[0][0] + current_frame[1][0] + current_frame[2][0] - current_frame[0][2] - current_frame[1][2] - current_frame[2][2]) / ((float)total);
            gesture_index = (gesture_index + 1) % RING_BUFFER_SIZE;
        }
        buffer[i] = buffer[i] / merge_amount;
    }
}

void center_of_gravity_distribution_y(float* buffer) {
    unsigned char merge_amount = get_num_recorded_frames() / 6;
    unsigned char gesture_index = calculate_pad_index(0);
    for (unsigned char i = 0; i < 6; ++i) {
        buffer[i] = 0.0;
        int **current_frame = get_gesture(gesture_index);
        // TODO: Last few frames may not be processed due to rounding!
        for (unsigned char j = 0; j < merge_amount; ++j) {
            int total = pixel_total(current_frame);
            if (total > 0)
                buffer[i] += (current_frame[0][0] + current_frame[0][1] + current_frame[0][2] - current_frame[2][0] - current_frame[2][1] - current_frame[2][2]) / ((float)total);
            gesture_index = (gesture_index + 1) % RING_BUFFER_SIZE;
        }
        buffer[i] = buffer[i] / merge_amount;
    }
}
