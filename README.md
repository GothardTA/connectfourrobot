# Connect Four Robot Code
This repository contains the code that will be used in my robot that plays connect four. A basic diagram of how this works is below.<br><br>

Raspberry Pi Camera<br>
Captures and image that is saved to the drive<br>
||<br>
||<br>
\\/<br>
Image Processor<br>
Gets data about where pieces are played and creates a data object with this data<br>
||<br>
||<br>
\\/
Calculate Best Move<br>
Uses an ai to calculate what collumn the robot should drop a piece in<br>
||<br>
||<br>
\\/<br>
Send Move to Arduino<br>
Send collumn to move to over serial to my stepper motor driver board
