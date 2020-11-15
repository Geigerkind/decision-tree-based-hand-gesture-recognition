// Reimplementation of the gesture reader found in lib_gesture

// Constants
#define ALPHA 0.1
#define PIXEL_THRESHOLD 3
#define MARGIN 0.1
#define PAD_GESTURE true
#define RING_BUFFER_SIZE 80

// Variables to recognize the start and end of a gesture
static float threshold_low = 0.0;
static float threshold_high = 0.0;
static float trigger_mean = 0.0;
static int last_frame[3][3] = {};
static bool record_gesture = false;
static bool is_initialized = false;

// Variables to stitch together the gesture
static char padding_counter = 3;
static char pad_buffer[3][3][3] = {};
static char pad_buffer_index = 0;
// Lets do a ring buffer of 60 frames
// We only have 2kb RAM
static int gesture[RING_BUFFER_SIZE][3][3] = {};
static unsigned char ring_buffer_index = 0;
static unsigned char num_recorded_frames = 0;

// Getter for static variables
unsigned char get_num_recorded_frames() {
  return num_recorded_frames;
}

int ** get_gesture(unsigned char index) {
  return (int**) gesture[index];
}

// Returns true if the threshold between the last frame
// and the current frame of any pixel is surpassed
bool pixel_diff() {
    int sum = 0;
    int **frame = get_frame();
    for (unsigned char i = 0; i < 3; ++i) {
        for (unsigned char j = 0; j < 3; ++j) {
            int diff = last_frame[i][j] - frame[i][j];
            if (diff >= PIXEL_THRESHOLD || diff <= -PIXEL_THRESHOLD)
                return true;
        }
    }
    return false;
}

float get_frame_mean(int** buffer) {
    float sum = 0.0;
    // Cant remember which way to iterate is the most efficient
    for (unsigned char i = 0; i < 3; ++i) {
        for (unsigned char j = 0; j < 3; ++j) {
            // Normalizing to prevent wraparounds
            sum += (((float) buffer[i][j]) / 1024.0);
        }
    }
    return sum / 9.0;
}

void copy_frame(int** buffer, int** other_buffer) {
    // TODO: For some reason this causes a crash, though it should be correct o.o
    //memcpy(buffer, other_buffer, sizeof(int) * 9);
    memcpy(buffer, other_buffer, 9);
}

unsigned char calculate_pad_index(unsigned char offset) {
    if (ring_buffer_index >= num_recorded_frames + offset)
        return ring_buffer_index - num_recorded_frames - offset;
    unsigned char wrap_around = num_recorded_frames + offset - ring_buffer_index;
    return RING_BUFFER_SIZE - wrap_around - 1;
}

// Returns true if the ring buffer may be read to retrieve the gesture
// We read the "frame" buffer from the serial module
// Its called after reading the frame
bool feed_frame() {
    int ** frame = (int**)get_frame();
    float frame_mean = get_frame_mean(frame);
    
    if (!is_initialized) {
        trigger_mean = frame_mean;
        threshold_low = trigger_mean * (1.0 - MARGIN);
        threshold_high = trigger_mean * (1.0 + MARGIN);
        copy_frame((int**)last_frame, frame);
        is_initialized = true;
        return false;
    }

    if (record_gesture && (!PAD_GESTURE || padding_counter > 0)) {
        copy_frame((int**)gesture[ring_buffer_index], frame);
        ring_buffer_index = (ring_buffer_index + 1) % RING_BUFFER_SIZE;
        if (num_recorded_frames < RING_BUFFER_SIZE)
          num_recorded_frames += 1;
        if (PAD_GESTURE) {
            padding_counter -= 1;
        }
    }

    // Stop recording
    if (frame_mean >= threshold_low && frame_mean <= threshold_high) {
        // Finish recording!
        if (record_gesture && (!PAD_GESTURE || padding_counter == 0)) {
            padding_counter = 3;
            record_gesture = false;
            if ((PAD_GESTURE && num_recorded_frames >= 3) || num_recorded_frames >= 6) {
                if (PAD_GESTURE) {
                    if (num_recorded_frames >= RING_BUFFER_SIZE - 3) {
                        return true;
                    }
                    copy_frame((int**)gesture[calculate_pad_index(2)], (int**)pad_buffer[(pad_buffer_index + 2) % 3]);
                    copy_frame((int**)gesture[calculate_pad_index(1)], (int**)pad_buffer[(pad_buffer_index + 1) % 3]);
                    copy_frame((int**)gesture[calculate_pad_index(0)], (int**)pad_buffer[pad_buffer_index]);
                    num_recorded_frames += 3;
                }
                return true;
            }
            return false;
        }

        if (!record_gesture && PAD_GESTURE) {
            copy_frame((int**)pad_buffer[pad_buffer_index], frame);
            pad_buffer_index = (pad_buffer_index + 1) % 3;
        }

        if (!record_gesture) {
            copy_frame((int**)last_frame, frame);
        }

        return false;
    }
    
    // Start recording
    if (frame_mean < threshold_low || frame_mean > threshold_high) {
        if (record_gesture && PAD_GESTURE) {
            padding_counter = 3;
        } else if (!record_gesture) {
            padding_counter = 2;
            copy_frame((int**)gesture[ring_buffer_index], frame);
            ring_buffer_index = (ring_buffer_index + 1) % RING_BUFFER_SIZE;
            record_gesture = true;
            num_recorded_frames = 1;
        }
    }

    if (!record_gesture) {
        trigger_mean = trigger_mean * (1.0 - ALPHA) + frame_mean * ALPHA;
        threshold_low = trigger_mean * (1.0 - MARGIN);
        threshold_high = trigger_mean + (1.0 + MARGIN);
    }
    
    return false;
}
