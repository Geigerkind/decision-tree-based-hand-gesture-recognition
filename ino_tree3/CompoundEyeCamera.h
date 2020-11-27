/*
 * This module is used to read frames from the camera, 
 * write frame to the serial port and delay the execution in the main loop.
 * 
 * The module supports different camera types, that is 
 * - The 4x4 pixel compound eye camera with purpose build board 
 * - The old compound eye camera with 3x3 pixels and Arduino Uno board 
 * - The old pinhole camera with 3x3 pixels and Arduino Nano board 
 * 
 * For definitions are assumed to be Modified: 
 * - CAMERA is set to the camera to be used. 
 * - BAUDRATE is set to desired baud rate of the serial port.
 * - NO_ROWS and NO_COLS to the desired number of rows and columns of a frame. 
 *   This can be less than available in the camera, and a different number for columns and rows.  
 *   For 3x3 cameras it may be set to up 4x4. In this case the light value of the nearest available pixel is used. 
 * 
 * Module was written by Marcus Venzke.
 */

// 4x4 pixel compound eye camera with purpose build board
#define COMPOUND_EYE_4x4 1

// Old compound eye camera with 3x3 pixels and Arduino Uno board
#define COMPOUND_EYE_3x3 2

// Pinhole camera with 3x3 pixels and Arduino Nano board
#define PINHOLE 3

// Camera to be used
// #define CAMERA COMPOUND_EYE_4x4
#define CAMERA COMPOUND_EYE_3x3
// #define CAMERA PINHOLE

// Baud rate used for the serial interface
#define BAUDRATE 115200

// Number of rows of a frame that shall be logically available. 
// The last row is repeated if bigger than NO_ROWS_NATIVE
#define NO_ROWS 3

// Number of colums of a frame that shall be logically available. 
// The column is repeated if bigger than NO_ROWS_NATIVE
#define NO_COLS 3

// Serial interface to be used
#if CAMERA == COMPOUND_EYE_4x4
  #define Ser Serial3
#else
  #define Ser Serial
#endif

// Number of pixels of a frame
#define NO_PIXELS (NO_ROWS * NO_COLS)

/*
 * Initialized this module.  
 * This function must be called before the module is used. 
 */
void CompoundEyeCamera_init();

/* Reads the current frame from the camera.
    The frame is returned as an array of NO_PIXELS int values.
    The order of values is row by row, that is the first row
    is in the first light values of the array.
    
    The function can be used for any number of rows and colums.
    The number of rows is taken from NO_ROWS and 
    the number of columns is taken from NO_COLS.
    
    Each light value is between 0 and 1023.
*/
void readFrame(short *buffer);

/*  Writes short type the light values of a frame comma seperated to the seral interface as one line.
    The serial interface to be used is references by the variable Ser
*/
void writeFrame(short *frame);

/*  Writes foat type light values of a frame comma seperated to the seral interface as one line.
    The serial interface to be used is references by the variable Ser
*/
void writeFrame(float *frame);

/*  Writes short type light values of a frame in a two dimensional structure to the seral interface as several lines.
    The serial interface to be used is references by the variable Ser
*/
void writeFrameArray(short *frame);

/*  Writes float type light values of a frame in a two dimensional structure to the seral interface as several lines.
    The serial interface to be used is references by the variable Ser
*/
void writeFrameArray(float *frame);

/*  Delay for waiting in the main loop.   
 *   
 *  The functions sets the PIN_FREQUENCY_MEASUREMENT to high, waits half of the time, 
 *  sets the pin to low and waits the other half of the time. 
 *  This is used in the main loop to measure the time for executing the loop 
 *  as the frequency on the pin. 
 *   
 *  timeMs is the number of milliseconds to wait.
 */

void delayWithFequencyMeasurement(unsigned long timeMs);

/* Toggles the PIN PIN_FREQUENCY_MEASUREMENT. 
 *  
 *  This can be used to measure the speed of the main loop. 
 */
 void toggleFrequencyPin();
