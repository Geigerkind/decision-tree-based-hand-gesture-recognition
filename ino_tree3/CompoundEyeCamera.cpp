/*
 * This module is used to read frames from the camera, 
 * write frame to the serial port and delay the execution in the main loop.
 * 
 * The module supports different camera types, that is 
 * - The 4x4 pixel compound eye camera with purpose build board 
 * - The old compound eye camera with 3x3 pixels and Arduino Uno board 
 * - The old pinhole camera with 3x3 pixels and Arduino Nano board 
 * 
 * Module was written by Marcus Venzke.
 */

#include "CompoundEyeCamera.h"
#include <Arduino.h>

#if CAMERA == COMPOUND_EYE_4x4
  // Sets the frequency of the CPU to 20 MHz
  #define F_CPU 20000000UL
  
  // Pin to be used to measure the frequency of repeating the main loop. 
  #define PIN_FREQUENCY_MEASUREMENT 8 // PA2

  // Numbers of AD converters connected to the light sensors
  short pixelADConverters[4][4] = {{12, 13, 14, 15},
                                   {10, 11,  8,  9},
                                   { 7,  4,  6,  5},
                                   { 3,  2,  1,  0}};
                                   
#else // Cameras with Arduino Uno / Nano Board
  // Pin to be used to measure the frequency of repeating the main loop. 
  #define PIN_FREQUENCY_MEASUREMENT 5 

  // Lowest and highes output pin used to select a column of the compound eye
  #define OUT_PIN_MIN 2
  #define OUT_PIN_MAX 4

  // Lowest and highes analog input pin used as imput of the compound eye  
  #define AIN_PIN_MIN A0
  #define AIN_PIN_MAX A2

  // Time in ms to wait per row, that is, between the output pin is set an the analog inputs are read.
  #define DELAY_PER_ROW 1

  // Number of rows of a frame physically implemented by the camera
  #define NO_ROWS_NATIVE (OUT_PIN_MAX - OUT_PIN_MIN + 1)
  
  // Number of rows of a frame physically implemented by the camera
  #define NO_COLS_NATIVE (AIN_PIN_MAX - AIN_PIN_MIN + 1)

  #if CAMERA == PINHOLE
    // Factors to multiply all brightnessValues used for pinhole camera.
    static int brightnessCoorrectionFactors[NO_ROWS][NO_COLS] = {{28, 18, 28},
                                                                 {22, 16, 18},
                                                                 {28, 18, 28}};
    // Value to devide all brightnessValues after multiplying with brightnessCoorrectionFactors.
    #define brightnessCoorrectionDivider 16
  #endif 
#endif

void CompoundEyeCamera_init() {
  // Set baud rate of serial port.
  Ser.begin(BAUDRATE);  

  #if CAMERA == COMPOUND_EYE_4x4
    pinMode(LED_BUILTIN, INPUT);  // Has to be set to use all AD-Converters. 
  #else // Cameras with Arduino Uno / Nano Board    
    // Configure output pins to control rows.
    for (int i = OUT_PIN_MIN; i <= OUT_PIN_MAX; i++) {
       pinMode(i, OUTPUT);
       digitalWrite(i, LOW); 
    }
  #endif

  // Configure pin to measure frame rate. 
  pinMode(PIN_FREQUENCY_MEASUREMENT, OUTPUT);
  digitalWrite(PIN_FREQUENCY_MEASUREMENT, LOW); 
}

#if CAMERA == COMPOUND_EYE_4x4
  void readFrame(short *buffer) {
    int bufferIndex = 0;
  
    for (int y = 0; y < NO_ROWS; y++) {
      for (int x = 0; x < NO_COLS; x++) {
        buffer[bufferIndex] = analogRead(pixelADConverters[y][x]);
        bufferIndex++;
      }
    }
  }
#else // Cameras with Arduino Uno / Nano Board -> Read row by row
  /* 
   * Reads one row and write it to array frame. 
   * The parameter row identifies the row to be read. The first row has the value 0. 
   * The parameter frame is the buffer to write the row to. 
   */
  void readRow(int row, short (*frame)[NO_ROWS][NO_COLS]) {
    // Activate pin that supplys voltage to the row.
    digitalWrite(row + OUT_PIN_MIN, HIGH);

    // Wait some time. 
    delay(DELAY_PER_ROW);

    // Read add values of the row from the analog input
    for (int col = 0; col < NO_COLS; col++) {
      if (col < NO_COLS_NATIVE) {
        int brightnessValue = analogRead(col + AIN_PIN_MIN);
        #if CAMERA == PINHOLE
          brightnessValue = brightnessValue * brightnessCoorrectionFactors[row][col] / brightnessCoorrectionDivider; 
        #endif
        (*frame)[row][col] = brightnessValue;
      } else { // More colums wanted as physically available: Copy previous column
        (*frame)[row][col] = (*frame)[row][col - 1];
      }
    }

    // Deactivate pin that supplys voltage to the row.
    digitalWrite(row + OUT_PIN_MIN, LOW);
  }

  void readFrame(short *buffer) {
    short (*frame)[NO_ROWS][NO_COLS] = (short(*)[NO_ROWS][NO_COLS]) buffer; // Buffer as 2D-Array. 

    for (int row = 0; row < NO_ROWS; row++) {
      if (row < NO_ROWS_NATIVE) { // Row physically available
        readRow(row, frame);     // Read row from sensors
      } else { // More rows than physically available: Copy previous row
        for (int col = 0; col < NO_COLS; col++) {
          (*frame)[row][col] = (*frame)[row - 1][col];
        }
      }
    }
  }
#endif

void writeFrame(short *frame) {
  for (int i = 0; i < NO_PIXELS; i++) {
    Ser.print(frame[i]);
    if (i < NO_PIXELS - 1) {
      Ser.print(",");
    }
  }
  Ser.println();
}

void writeFrame(float *frame) {
  for (int i = 0; i < NO_PIXELS; i++) {
    Ser.print(frame[i], 4);
    if (i < NO_PIXELS - 1) {
      Ser.print(",");
    }
  }
  Ser.println();
}

void writeFrameArray(short *frame) {
  static char printBuffer[7];
  for (int y = 0; y < NO_ROWS; y++) {
    for (int x = 0; x < NO_COLS; x++) {
      sprintf(printBuffer, "%4d ", frame[y * NO_COLS + x]);
      Ser.print(printBuffer);
    }
    Ser.println();
  }  
  Ser.println();
}

void writeFrameArray(float *frame) {
  for (int y = 0; y < NO_ROWS; y++) {
    for (int x = 0; x < NO_COLS; x++) {
      Ser.print(frame[y * NO_COLS + x], 4);
      Ser.print("\t");
    }
    Ser.println();
  }  
  Ser.println();
}

void delayWithFequencyMeasurement(unsigned long timeMs) {
  unsigned long halfTime = timeMs / 2;
  digitalWrite(PIN_FREQUENCY_MEASUREMENT, HIGH); // Set pin for frequency measurement to high
  delay(halfTime);                               // Wait half of the time
  digitalWrite(PIN_FREQUENCY_MEASUREMENT, LOW);  // Set pin for frequency measurement to log
  delay(timeMs- halfTime);                       // Wait the other half of the time
}

void toggleFrequencyPin() {
  digitalWrite(PIN_FREQUENCY_MEASUREMENT, !digitalRead(PIN_FREQUENCY_MEASUREMENT));
}
