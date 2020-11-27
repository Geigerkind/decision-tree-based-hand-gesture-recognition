#include "CompoundEyeCamera.h"
#include "decision_tree.h"
#include "features.h"

// Maximum number of frames that can be stored in the buffer.
// This is thus the maximum number of frames a gesture can consist of.
#define FRAME_BUFFER_SIZE 80

// Number of milliseconds between each frame
#define MILLISECONDS_PER_FRAME 13

// Number of frame that are added to a candidate at the beginning and ending 
#define NO_FRAMES_ADDED_AT_BEGINNING_AND_END 3

// Parameter of the rolling average determining the speed of change 
// 1 would mean, that the rolling average is immediately the new value 
// 0 would mean, that the rolling average never changes. 
#define ROLLING_AVERAGE_ALPHA 0.01

// Parameter of the rolling average determining which values are used to calculate it. 
// Only brightness values are used, that deviate from the previous value by less than ROLLING_AVERAGE_DELTA.
#define ROLLING_AVERAGE_DELTA 5

// Parameter that determines how much darker a frame has to be to be accepted as a frame of a candidate. 
// A value of 0.1 means that its average brightness needs to be 10% less than the rolling average. 
#define THRESHOLD_DARKER_FRAMES_OF_CANDIDATES 0.1

/* The following describes the state used to read a candidate of a gesture from the sensors */
// First state in which NO_FRAMES_ADDED_AT_BEGINNING_AND_END frames are read to initialize reading. 
// It is used at program start, after recognizing or aborting a gesture.
#define READ_INITIAL_FRAMES 0

// State in which frames are about as bright as the frames before. 
#define SEARCH_DARKER_FRAMES 1

// State in which the darker frames are collected as part of the candidates
#define RECORD_CANDIDATE_FRAMES 2

// State in which NO_FRAMES_ADDED_AT_BEGINNING_AND_END frame are added to finalizie the candidate. 
#define RECORD_ENDING_FRAMES 3

// Value to normalize light sensor values to features. 
// Light sensor values are divided by this value. 
#define NORMALIZING_DIVIDER 1024.0

// Time (im milliseconds) when the processing of the current frame started
int frameStartTime;

// Number of frames already stored in frameBuffer.
int noFramesInBuffer;

// Rolling average of the brightness of the previous frames.
float rollingAverageBrightness;

// Buffer to store the scaled frames of a candidate.
short frameBuffer[FRAME_BUFFER_SIZE][NO_PIXELS];

// Arg buffer for the decision tree
long dt_args[12];


/*
    Initializes the LED-Pins.
*/
void initalizeLEDs() {
  pinMode(11, OUTPUT);
  pinMode(32, OUTPUT);
  pinMode(33, OUTPUT);
  pinMode(34, OUTPUT);
}

/**
   Blicks LEDs to show a reset.
*/
void displayResetOnLED() {
  digitalWrite(11, HIGH);
  delay(200);
  digitalWrite(11, LOW);
  delay(200);
  digitalWrite(11, HIGH);
  delay(200);
  digitalWrite(11, LOW);
  digitalWrite(32, HIGH);
}

/*
   Sets the LEDs according to the gesture.
   Parameter gesture contains the gesture to be shown on the LEDs
*/
void displayGestureOnLEDs(int gesture) {
  if (gesture == 1) {
    digitalWrite(32, HIGH); // PC0
    digitalWrite(33, LOW);  // PC1
    digitalWrite(34, LOW);  // PC2
    digitalWrite(11, LOW);  // PC3
  } else if (gesture == 2) {
    digitalWrite(32, LOW);
    digitalWrite(33, HIGH);
    digitalWrite(34, LOW);
    digitalWrite(11, LOW);
  } else if (gesture == 3) {
    digitalWrite(32, LOW);
    digitalWrite(33, LOW);
    digitalWrite(34, HIGH);
    digitalWrite(11, LOW);
  } else if (gesture == 4) {
    digitalWrite(32, LOW);  // PC0
    digitalWrite(33, LOW);  // PC1
    digitalWrite(34, LOW);  // PC2
    digitalWrite(11, HIGH); // PC3
  } else {
    digitalWrite(32, LOW);
    digitalWrite(33, LOW);
    digitalWrite(34, LOW);
    digitalWrite(11, LOW);
  }
}

/* 
 * Calculates the average brightness of all light values of a frame.   
 * The frame is given to the function as parameter. 
 */
short brightnessOfFrame(short *frame) {
  short sum = 0;
  for (int i = 0; i < NO_PIXELS; i++) {
    sum += frame[i];
  }
  return sum / NO_PIXELS;
}

/*
 * Copies one frame from one location to another. 
 * source is a pointer to the frame to copy from. 
 * destination is a pointer to the memory to copy the frame to. 
 */
void copyFrame(short *source, short *destination) {
  for (int i = 0; i < NO_PIXELS; i++) {
    destination[i] = source[i];
  }
}

/* 
 *  Removes the first image from the frameBuffer. 
 */
void removeFirstFrameFromBuffer() {
  for (int i = 1; i < noFramesInBuffer; i++) {
    copyFrame(frameBuffer[i], frameBuffer[i-1]);
  }
  noFramesInBuffer--; 
}

/*
 * Calculates a rolling average of light values.
 * 
 * This changes the global variable rollingAverageBrightness. 
 * If this value is not initialized (has the value -1) it is set to the averageBrightness
 * The rolling average is only calculated, if the different to the previous call to this function is less thn ROLLING_AVERAGE_DELTA. 
 * 
 * Parameter averageBrightnessCurrentFrame: Average brightness the frame just read. 
 */
void calculateRollingAverageBrightness(short averageBrightnessCurrentFrame) {
  static short averageBrightnessPreviousFrame = -1;

  if (averageBrightnessPreviousFrame < 0) {  // The the average brightness of the previous call to this function is not set, 
    averageBrightnessPreviousFrame = averageBrightnessCurrentFrame; // Then set this value to the current brightness. 
  }
  
  if (rollingAverageBrightness < 0) { // rolling average not yet set?
    rollingAverageBrightness = averageBrightnessCurrentFrame; // initialize rolling average. 
  } 

  // Calculate rolling average only if the average bightness of the current frame does not deviate from the previous by ROLLING_AVERAGE_DELTA. 
  if (abs(averageBrightnessCurrentFrame - averageBrightnessPreviousFrame) < ROLLING_AVERAGE_DELTA) { 
    rollingAverageBrightness = (1 - ROLLING_AVERAGE_ALPHA) * rollingAverageBrightness + ROLLING_AVERAGE_ALPHA * averageBrightnessCurrentFrame;
  }

  // Remember parameter for next call. 
  averageBrightnessPreviousFrame = averageBrightnessCurrentFrame;
}

/*
 * Initializes the extraction of a candidate. 
 */
void init_candidate_extraction() {
  noFramesInBuffer = 0; // Flush all frames in the frameBuffer.

  // Signal that there is no rolling average yet.
  // The value will be set, when the first frame is read.
  rollingAverageBrightness = -1;

  // Ser.println("--- Init ---"); 
}

/* Initialize the program. */
void setup() {
  CompoundEyeCamera_init();
  initalizeLEDs();

  Ser.println("------ Reset -----"); // Displays reset on serial line
  displayResetOnLED();

  init_candidate_extraction(); // Initialize the variables for recognizing extraction.
  frameStartTime = millis();   // Remember the current start time to consider it the start time of the first frame.
}

/* Main-Routine of the program that is repleated again and again. */
void loop() {
  static int state = READ_INITIAL_FRAMES;       // State of the process to extract a candidate. 
  int averageBrightnessCurrentFrame;            // Average brightness of the light values of the current frame 
  int noFramesToBeRecordedInRecordEndingFrames; // No of frames that shall be recorded in state RECORD_ENDING_FRAMES

  unsigned long annStartTime;  
  unsigned long annEndTime;
  int frameEndTime;  // Time (in milliseconds) when the processing of this frame ended.
  int frameExecutionTime; // Time for executing the frame (in milliseconds).

  readFrame(frameBuffer[noFramesInBuffer]);  // Read a frame values from the sensors.
  // writeFrameArray(frameBuffer[noFramesInBuffer]);  // Display frame on serial interface
  averageBrightnessCurrentFrame = brightnessOfFrame(frameBuffer[noFramesInBuffer]);
  noFramesInBuffer++; // One more frame in the sensor.

  if (noFramesInBuffer >= FRAME_BUFFER_SIZE) { // If the buffer gets bigger than the maximum buffer size
    init_candidate_extraction();               // Abort the candidate
    state = READ_INITIAL_FRAMES;               // Restart the recognition process
  }

  if (state == READ_INITIAL_FRAMES) {  // Read first frames that will precide the frames with lower brightness. 
    calculateRollingAverageBrightness(averageBrightnessCurrentFrame); // Calculate the rolling of the brightness
    if (noFramesInBuffer >= NO_FRAMES_ADDED_AT_BEGINNING_AND_END) { // If enough frames are read ... 
      state = SEARCH_DARKER_FRAMES;                                 // Goto next state. 
    }
  } else if (state == SEARCH_DARKER_FRAMES) { // Read frames and remove first frame until a frame is much darker starting a candidate. 
    calculateRollingAverageBrightness(averageBrightnessCurrentFrame); // Calculate the rolling of the brightness

    if (averageBrightnessCurrentFrame < rollingAverageBrightness * (1.0 - THRESHOLD_DARKER_FRAMES_OF_CANDIDATES)) { // Average brightness dark enough? 
      state = RECORD_CANDIDATE_FRAMES;  // Goto state recording the dark frames.
    } else { 
      removeFirstFrameFromBuffer();    // Remove the first frame from the buffer and stay in the state. 
    }
  } else if (state == RECORD_CANDIDATE_FRAMES) { // Read the frames that are darker and become frames of the candidate. 
    // Do not change the rolling average here! The candidate shall not change it. 

    if (averageBrightnessCurrentFrame >= rollingAverageBrightness * (1.0 - THRESHOLD_DARKER_FRAMES_OF_CANDIDATES)) { // Average brightness again too bright for a candidate?
      state = RECORD_ENDING_FRAMES; // Record a few more frames.  
      // (NO_FRAMES_ADDED_AT_BEGINNING_AND_END - 1) frames have still to be recorded, as the current frame is the first. 
      noFramesToBeRecordedInRecordEndingFrames = NO_FRAMES_ADDED_AT_BEGINNING_AND_END - 1; 
    }
  } else if (state == RECORD_ENDING_FRAMES) { // Record noFramesToBeRecordedInRecordEndingFrames more frames 
    noFramesToBeRecordedInRecordEndingFrames--;        // Remember that one more frame has been read.
    if (noFramesToBeRecordedInRecordEndingFrames <= 0) { // If all frames have been read ... 
      int gesture;
      // Process frame buffer
      Ser.println(" * * * * * Candidate found * * * * *");
      Ser.print("Number frames: ");
      Ser.println(noFramesInBuffer);

      // *************************** From here: ANN *******************************

      unsigned long featureStartTime = micros();
      center_of_gravity_distribution_long_x(frameBuffer, noFramesInBuffer, dt_args);
      center_of_gravity_distribution_long_y(frameBuffer, noFramesInBuffer, dt_args + 6);
      unsigned long featureEndTime = micros();

      unsigned long treeStartTime = micros();
      unsigned char prediction = evaluate(dt_args);
      
      unsigned long treeEndTime = micros();

      Ser.print("Feature Execution Time: ");
      Ser.print(featureEndTime - featureStartTime);
      Ser.println(" micro seconds");
      Ser.print("Tree Execution Time: ");
      Ser.print(treeEndTime - treeStartTime);
      Ser.println(" micro seconds");

      Ser.print("Decision Tree Result: ");
      Ser.println(prediction);

      // *************************** Before here: ANN *******************************

      init_candidate_extraction();               // Initialize to extract for the next candidate. 
      state = READ_INITIAL_FRAMES;               // Restart the extraction process. 
    }
  }

  // Debug-Output
  // Ser.println(state);
  // Ser.println(noFramesInBuffer);
  // Ser.println(averageBrightnessCurrentFrame);
  // Ser.println(rollingAverageBrightness);

  toggleFrequencyPin();                 // Toggle a pin to allow measuring the speed of the loop.

  frameEndTime = millis();              // Remember when the processing of the Frame ended.
  frameExecutionTime = frameEndTime - frameStartTime;
  // Ser.println(frameExecutionTime);
  // Ser.println(MILLISECONDS_PER_FRAME - frameExecutionTime);
  if (frameExecutionTime < MILLISECONDS_PER_FRAME) {
    delay(MILLISECONDS_PER_FRAME - frameExecutionTime); // Wait until the next frame shall start.
  }

  frameStartTime = frameEndTime;        // Use frameEndTime as the start time of the next frame.
}
