void setup() {
  setup_serial();
}

void loop() {
  readFrame();

  if (feed_frame()) {
    // Gesture is parsed
    // Evaluate the features
    float args[12];
    center_of_gravity_distribution_x(args);
    center_of_gravity_distribution_y(args + 6);

    // Run it through the tree!
    unsigned char result = evaluate_forest(args);
    Serial.print("Result: ");
    Serial.println(result);
  }

  delay(13);
}
