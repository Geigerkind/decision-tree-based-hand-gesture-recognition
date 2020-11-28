#include "features.h"

void center_of_gravity_distribution(COCD_CURRENT_BUFFER_TYPE cocd_buffer[125], int num_recorded_frames, COCD_CURRENT_RETURN_TYPE* arg_buffer) {
    char amount_always_merge = num_recorded_frames / 5;
    char rest = num_recorded_frames % 5;
    char merge_pattern[5] = {amount_always_merge, amount_always_merge, amount_always_merge, amount_always_merge, amount_always_merge};
    if (rest == 1) {
        ++merge_pattern[0];
    } else if (rest == 2) {
        ++merge_pattern[0];
        ++merge_pattern[4];
    } else if (rest == 3) {
        ++merge_pattern[0];
        ++merge_pattern[2];
        ++merge_pattern[4];
    } else if (rest == 4) {
        ++merge_pattern[0];
        ++merge_pattern[1];
        ++merge_pattern[2];
        ++merge_pattern[4];
    }

    COCD_CURRENT_BUFFER_TYPE* current_buffer = cocd_buffer;
    COCD_CURRENT_RETURN_TYPE* current_arg_buffer = arg_buffer;
    for (char i = 0; i < 5; ++i) {
        *current_arg_buffer = 0;
        for (char j = 0; j < merge_pattern[i]; ++j) {
            *current_arg_buffer += *current_buffer;
            ++current_buffer;
        }
        *current_arg_buffer /= ((COCD_CURRENT_RETURN_TYPE)merge_pattern[i]);
        ++current_arg_buffer;
    }
}
