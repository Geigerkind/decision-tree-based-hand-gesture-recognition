// NOTE: We cant access variables from other files

// Number of milliseconds between each frame
#define MILLISECONDS_PER_FRAME 25


////////////////
// Serial.ino //
////////////////

// This is copied from Dr. Venzke's implementation (CameraSendingFramesToSerial)
// Baud rate used for the serial interface
#define BAUDRATE 115200

// Lowest and highes output pin used to select a column of the compound eye
#define OUT_PIN_MIN 2
#define OUT_PIN_MAX 4

// Lowest and highes analog input pin used as imput of the compound eye
#define AIN_PIN_MIN A0
#define AIN_PIN_MAX A2

// Lowest and highest digital input pin used to recored buttons that are pressed
#define IN_PIN_MIN  8
#define IN_PIN_MAX 13

// PIN that is inverted after each frame to measure frame rate.
#define PIN_FRAME_RATE 5

// Time in ms to wait per row, that is, between the output pin is set an the analog inputs are read.
#define DELAY_PER_ROW 3

// Number of rows of a frame
#define NO_ROWS (OUT_PIN_MAX - OUT_PIN_MIN + 1)

// Number of rows of a frame
#define NO_COLS (AIN_PIN_MAX - AIN_PIN_MIN + 1)

// Number of pixels of a frame
#define NO_PIXELS (NO_ROWS * NO_COLS)

// Storage for one frame read analog inputs and sent to the serial interface
static int frame[NO_ROWS][NO_COLS];

/*
 * Reads one row and write it to array frame.
 * The parameter identifies the row. The first row has the value 0.
 */
void readRow(int row) {
  // Activate pin that supplys voltage to the row.
  digitalWrite(row + OUT_PIN_MIN, HIGH);

  // Wait some time.
  delay(DELAY_PER_ROW);

  // Read add values of the row from the analog input
  for (int col = 0; col < NO_COLS; col++) {
    int brightnessValue = analogRead(col + AIN_PIN_MIN);
    frame[row][col] = brightnessValue;
  }

  // Deactivate pin that supplys voltage to the row.
  digitalWrite(row + OUT_PIN_MIN, LOW);
}

void readFrame() {
  // Read all rows
  for (int row = 0; row < NO_ROWS; row++) {
    readRow(row);
  }
}

void setup_serial() {
  Serial.begin(BAUDRATE);

  // Configure output pins to control rows
  for (int i = OUT_PIN_MIN; i <= OUT_PIN_MAX; i++) {
    pinMode(i, OUTPUT);
    digitalWrite(i, LOW);
  }

  // Configure pin to measure frame rate
  pinMode(PIN_FRAME_RATE, OUTPUT);
  digitalWrite(PIN_FRAME_RATE, LOW);
}


/////////////////
// Gesture.ino //
/////////////////

// Reimplementation of the gesture reader found in lib_gesture

// Constants
#define ALPHA 0.01
#define MARGIN 0.15
#define PAD_GESTURE true
#define RING_BUFFER_SIZE 60

// Variables to recognize the start and end of a gesture
static float threshold_low = 0.0;
static float threshold_high = 0.0;
static float trigger_mean = 0.0;
static int last_frame[3][3] = {};
static bool record_gesture = false;
static bool is_initialized = false;

// Variables to stitch together the gesture
static char padding_counter = 3;
static int pad_buffer[3][3][3] = {};
static char pad_buffer_index = 0;
// Lets do a ring buffer of 60 frames
// We only have 2kb RAM
static int gesture[RING_BUFFER_SIZE][3][3] = {};
static unsigned char ring_buffer_index = 0;
static unsigned char num_recorded_frames = 0;

float get_frame_mean(int buffer[3][3]) {
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

void copy_frame(int buffer[3][3], int other_buffer[3][3]) {
    memcpy(buffer, other_buffer, sizeof(int) * 9);
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
    float frame_mean = get_frame_mean(frame);

    if (!is_initialized) {
        trigger_mean = frame_mean;
        threshold_low = trigger_mean * (1.0 - MARGIN);
        threshold_high = trigger_mean * (1.0 + MARGIN);
        copy_frame(last_frame, frame);
        is_initialized = true;
        return false;
    }

    if (record_gesture && (!PAD_GESTURE || padding_counter > 0)) {
        copy_frame(gesture[ring_buffer_index], frame);
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
                    copy_frame(gesture[calculate_pad_index(2)], pad_buffer[(pad_buffer_index + 2) % 3]);
                    copy_frame(gesture[calculate_pad_index(1)], pad_buffer[(pad_buffer_index + 1) % 3]);
                    copy_frame(gesture[calculate_pad_index(0)], pad_buffer[pad_buffer_index]);
                    num_recorded_frames += 3;
                }
                return true;
            }
            return false;
        }

        if (!record_gesture && PAD_GESTURE) {
            copy_frame(pad_buffer[pad_buffer_index], frame);
            pad_buffer_index = (pad_buffer_index + 1) % 3;
        }

        if (!record_gesture) {
            copy_frame(last_frame, frame);
        }

        return false;
    }

    // Start recording
    if (frame_mean < threshold_low || frame_mean > threshold_high) {
        if (record_gesture && PAD_GESTURE) {
            padding_counter = 3;
        } else if (!record_gesture) {
            padding_counter = 2;
            copy_frame(gesture[ring_buffer_index], frame);
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

/////////////////
// feature.ino //
/////////////////

int pixel_total(int buffer[3][3]) {
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
void center_of_gravity_distribution_x(float buffer[6]) {
    // We collect a minimum of 6 frames per gesture, hence this will be >0
    float merge_amount = ((float)num_recorded_frames) / 6.0;
    unsigned char gesture_index = calculate_pad_index(0);
    float j = 1.0;
    for (unsigned char i = 0; i < 6; ++i) {
        buffer[i] = 0.0;
        for (; j <= (merge_amount * ((float)(i + 1))); j += 1.0) {
            int total = pixel_total(gesture[gesture_index]);
            if (total > 0)
                buffer[i] += (float)(gesture[gesture_index][0][0] + gesture[gesture_index][1][0] + gesture[gesture_index][2][0] - gesture[gesture_index][0][2] - gesture[gesture_index][1][2] - gesture[gesture_index][2][2]) / ((float)total);
            gesture_index = (gesture_index + 1) % RING_BUFFER_SIZE;
        }
        buffer[i] = buffer[i] / ((float)merge_amount);
    }
}

void center_of_gravity_distribution_y(float buffer[6]) {
    float merge_amount = ((float)num_recorded_frames) / 6.0;
    unsigned char gesture_index = calculate_pad_index(0);
    float j = 1.0;
    for (unsigned char i = 0; i < 6; ++i) {
        buffer[i] = 0.0;
        for (; j <= (merge_amount * ((float)(i + 1))); j += 1.0) {
            int total = pixel_total(gesture[gesture_index]);
            if (total > 0)
                buffer[i] += (float)(gesture[gesture_index][0][0] + gesture[gesture_index][0][1] + gesture[gesture_index][0][2] - gesture[gesture_index][2][0] - gesture[gesture_index][2][1] - gesture[gesture_index][2][2]) / ((float)total);
            gesture_index = (gesture_index + 1) % RING_BUFFER_SIZE;
        }
        buffer[i] = buffer[i] / ((float)merge_amount);
    }
}


void setup() {
  setup_serial();
}

void loop() {
  int frameStartTime = millis();
  readFrame();

  bool gesture_recognized = feed_frame();

  if (gesture_recognized) {
    // Gesture is parsed
    // Evaluate the features
    float args[12];
    Serial.print("Number recorded frames: ");
    Serial.println(num_recorded_frames);
    int featureStartTime = millis();
    center_of_gravity_distribution_x(args);
    center_of_gravity_distribution_y(args + 6);
    int featureEndTime = millis();
    Serial.print("Feature Execution time: ");
    Serial.println(featureEndTime - featureStartTime);
    for (int i=0; i<12; ++i) {
        Serial.print(args[i]);
        Serial.print(",");
    }
    Serial.println("");


    // Run it through the tree!
    int treeStartTime = millis();
    unsigned char result = evaluate(args);
    int treeEndTime = millis();
    Serial.print("Tree Execution time: ");
    Serial.println(treeEndTime - treeStartTime);
    Serial.print("Result: ");
    Serial.println(result);
  }

  int frameExecutionTime = millis() - frameStartTime;
  if (gesture_recognized) {
    Serial.print("Total Execution time: ");
    Serial.println(frameExecutionTime);
  }
  if (frameExecutionTime < MILLISECONDS_PER_FRAME) {
    delay(MILLISECONDS_PER_FRAME - frameExecutionTime); // Wait until the next frame shall start.
  }
}
