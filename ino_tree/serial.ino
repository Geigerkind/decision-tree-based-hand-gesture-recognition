// This is copied from Dr. Venzke's implementation (CameraSendingFramesToSerial)

// Baud rate used for the serial interface
#define BAUDRATE 115200

// Correct Brightness? 1 = Yes, 0 = No
#define PINHOLE_CAMERA 0

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

// Factors to multiply all brightnessValues
static int brightnessCoorrectionFactors[NO_ROWS][NO_COLS] = {{28, 18, 28},
                                                             {22, 16, 18},
                                                             {28, 18, 28}};
// Value to devide all brightnessValues after multiplying with brightnessCoorrectionFactors.
#define brightnessCoorrectionDivider 16

// Storage for one frame read analog inputs and sent to the serial interface
static int frame[NO_ROWS][NO_COLS];

// Getter for static variables
int ** get_frame() {
  return (int**) frame;
}

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

/**
 * Corrects the brightness of the frame of every sensor with factors from brightnessCoorrectionFactors.
 */
void correctBrightness() {
  for (int row = 0; row < NO_ROWS; row++) {
    for (int col = 0; col < NO_COLS; col++) {
        frame[row][col] = frame[row][col] * brightnessCoorrectionFactors[row][col] / brightnessCoorrectionDivider;
    }
  }
}

void readFrame() {
  // Read all rows
  for (int row = 0; row < NO_ROWS; row++) {
    readRow(row);
  }

  if (PINHOLE_CAMERA) {
    correctBrightness();
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
