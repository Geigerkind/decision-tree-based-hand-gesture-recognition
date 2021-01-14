#define COCD_CURRENT_RETURN_TYPE float
// #define COCD_CURRENT_RETURN_TYPE short
#define COCD_CURRENT_BUFFER_TYPE float
// #define COCD_CURRENT_BUFFER_TYPE short

void center_of_gravity_distribution(COCD_CURRENT_BUFFER_TYPE cocd_buffer[125], int num_recorded_frames, COCD_CURRENT_RETURN_TYPE* arg_buffer);
