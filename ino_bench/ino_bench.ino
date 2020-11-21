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

void setup() {
  setup_serial();
}

void loop() {
    unsigned long iterations = 1000000;
    unsigned long bench_sum = 0;
    unsigned long max_val = 0;

    float avg_measurement_failure = 0.0;
    unsigned long max_measurement_failure = 0;

    // max start to end delay
    bench_sum = 0;
    max_val = 0;
    for (unsigned long i=0; i < iterations; ++i) {
        unsigned long startTime = micros();
        unsigned long endTime = micros();
        if (endTime-startTime > max_val) {
          max_val = endTime - startTime;
        }
        bench_sum += (endTime - startTime);
    }
    Serial.print("start to end failure: ");
    Serial.println(((float)bench_sum)/((float)iterations));
    Serial.print("Max: ");
    Serial.println(max_val);

    avg_measurement_failure = ((float)bench_sum)/((float)iterations);
    max_measurement_failure = max_val;

    // __divmodhi4
    bench_sum = 0;
    max_val = 0;
    char dont_opt_away7 = 0;
    for (unsigned long i=0; i < iterations; ++i) {
        int some_int = (int)i;
        int some_div = (int)(i+3);
        dont_opt_away7 += (char)some_int;
        dont_opt_away7 += (char)some_div;
        unsigned long startTime = micros();
        char some_char = some_int / some_div;
        unsigned long endTime = micros();
        if (endTime-startTime > max_val) {
          max_val = endTime - startTime;
        }
        
        dont_opt_away7 += some_char;
        bench_sum += (endTime - startTime);
    }
    Serial.print("__divmodhi4: ");
    Serial.println(((float)bench_sum)/((float)iterations) - avg_measurement_failure);
    Serial.print("Max: ");
    Serial.println(max_val - max_measurement_failure);

    // __divmodsi
    bench_sum = 0;
    long some_long = 42;
    max_val = 0;
    long dont_opt_away6 = 0;
    for (unsigned long i=0; i < iterations; ++i) {
        long test = (long)i;
        long test2 = some_long + (long)(i + 3);
        dont_opt_away6 += test;
        dont_opt_away6 += test2;
        unsigned long startTime = micros();
        test2 /= test;
        unsigned long endTime = micros();
        if (endTime-startTime > max_val) {
          max_val = endTime - startTime;
        }
        dont_opt_away6 += test2;
        bench_sum += (endTime - startTime);
    }
    Serial.print("__divmodsi: ");
    Serial.println(((float)bench_sum)/((float)iterations) - avg_measurement_failure);
    Serial.print("Max: ");
    Serial.println(max_val - max_measurement_failure);

    // __floatsisf
    bench_sum = 0;
    max_val = 0;
    float dont_opt_away = 0.0;
    for (unsigned long i=0; i < iterations; ++i) {
        //char test2 = i + 3 * i + (42 / i);
        char test2 = (char)i;
        unsigned long startTime = micros();
        dont_opt_away += (float)test2;
        unsigned long endTime = micros();
        if (endTime-startTime > max_val) {
          max_val = endTime - startTime;
        }
        bench_sum += (endTime - startTime);
    }
    Serial.print("__floatsisf: ");
    Serial.println(((float)bench_sum)/((float)iterations) - avg_measurement_failure - 9.09);
    Serial.print("Max: ");
    Serial.println(max_val - max_measurement_failure - 12);

    // __addsf3
    bench_sum = 0;
    max_val = 0;
    float some_float = 32.24;
    float dont_opt_away2 = 0.0;
    for (unsigned long i=0; i < iterations; ++i) {
        unsigned long startTime = micros();
        dont_opt_away2 += some_float;
        unsigned long endTime = micros();
        bench_sum += (endTime - startTime);
        if (endTime-startTime > max_val) {
          max_val = endTime - startTime;
        }
    }
    Serial.print("__addsf: ");
    Serial.println(((float)bench_sum)/((float)iterations) - avg_measurement_failure);
    Serial.print("Max: ");
    Serial.println(max_val - max_measurement_failure);

    // __subsf3
    bench_sum = 0;
    max_val = 0;
    float some_float2 = 312.24;
    float dont_opt_away3 = 0.0;
    for (unsigned long i=0; i < iterations; ++i) {
        unsigned long startTime = micros();
        dont_opt_away3 -= some_float2;
        unsigned long endTime = micros();
        if (endTime-startTime > max_val) {
          max_val = endTime - startTime;
        }
        bench_sum += (endTime - startTime);
    }
    Serial.print("__subsf: ");
    Serial.println(((float)bench_sum)/((float)iterations) - avg_measurement_failure);
    Serial.print("Max: ");
    Serial.println(max_val - max_measurement_failure);

    // __divsf3
    bench_sum = 0;
    max_val = 0;
    float some_float3 = 5.0;
    float dont_opt_away4 = 0.0;
    for (unsigned long i=0; i < iterations; ++i) {
        float test = (float)i;
        float test2 = some_float3 + (float)(i + 3);
        dont_opt_away4 += test;
        dont_opt_away4 += test2;
        unsigned long startTime = micros();
        test /= test2;
        unsigned long endTime = micros();
        if (endTime-startTime > max_val) {
          max_val = endTime - startTime;
        }
        dont_opt_away4 += test;
        bench_sum += (endTime - startTime);
    }
    Serial.print("__divsf: ");
    Serial.println(((float)bench_sum)/((float)iterations) - avg_measurement_failure);
    Serial.print("Max: ");
    Serial.println(max_val - max_measurement_failure);

    // __lesf2
    bench_sum = 0;
    max_val = 0;
    float some_float4 = 33.24;
    float dont_opt_away5 = 0.0;
    for (unsigned long i=0; i < iterations; ++i) {
        float test = (float)i;
        float test2 = some_float3 + (float)(i + 3);
        dont_opt_away5 += test;
        dont_opt_away5 += test2;
        unsigned long startTime = micros();
        if (test <= test2) {
          unsigned long endTime = micros();
          bench_sum += (endTime - startTime);
          dont_opt_away5 += 1.0;
          if (endTime-startTime > max_val) {
            max_val = endTime - startTime;
          }
        } else {
          unsigned long endTime = micros();
          bench_sum += (endTime - startTime);
          dont_opt_away5 += 2.0;
          if (endTime-startTime > max_val) {
            max_val = endTime - startTime;
          }
        }
    }
    Serial.print("__lesf2: ");
    Serial.println(((float)bench_sum)/((float)iterations) - avg_measurement_failure);
    Serial.print("Max: ");
    Serial.println(max_val - max_measurement_failure);

    Serial.print("Ignore this: ");
    Serial.print(dont_opt_away);
    Serial.print(dont_opt_away2);
    Serial.print(dont_opt_away3);
    Serial.print(dont_opt_away4);
    Serial.print(dont_opt_away5);
    Serial.print(dont_opt_away6);
    Serial.print(dont_opt_away7);
    Serial.println();
}
